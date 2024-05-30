use std::collections::HashMap;
use std::marker::PhantomData;

use html2md::common::get_tag_attr;
use html2md::{parse_html_custom, TagHandler, TagHandlerFactory};
use reqwest::Client;
use serde_json::json;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use worker::{console_debug, console_warn};

use crate::calendar::Event;
use crate::EventDateType;

const THREE_DAYS: &str = r#"@announce@

@description@@date@
@hour@
@location@@link@"#;

const ONE_HOUR: &str = r#":warning: :warning: Atenciooon!!!! :warning: :warning: 
En una hora @start@:

@events@

> **NOTA:** Haciendo click en cada evento pueden ver los detalles en el calendario
> Y recuerda que puedes agregar el calendario para estar al dia de los eventos en Español desde [este enlace](<https://calendar.google.com/calendar/u/0?cid=ZGFmYzYyODQwMzRkOWJlZjNlMzFkZTNiZDE1OTI2OGQ5OGU4YzlhOGY2MjU3Mzk4NGI3MGE1OWQ2NjU3ZjVhZkBncm91cC5jYWxlbmRhci5nb29nbGUuY29t>)
"#;

async fn send(
    client: &Client,
    endpoint: &str,
    apikey: &str,
    message: &str,
    roles: &[i64],
    channel: i64,
) {
    let req = json!({
        "message": message,
        "channelId": channel,
        "roles": roles,
    });

    let res = client
        .post(endpoint)
        .header("content-type", "application/json")
        .header("Authorization", apikey)
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))
        .unwrap()
        .text()
        .await
        .inspect_err(|e| console_warn!("Json Error: {e:?}"))
        .unwrap();

    console_debug!("Result: {res:?}");
}

#[allow(clippy::too_many_arguments)]
pub async fn build_message(
    client: &Client,
    endpoint: &str,
    apikey: &str,
    three_days: &[(EventDateType, Event)],
    one_hour: &[(EventDateType, Event)],
    roles: &[i64],
    channel: i64,
    bot_channel: i64,
) {
    for (_, e) in three_days {
        let timestamp = OffsetDateTime::parse(&e.start.date_time, &Rfc3339)
            .unwrap_or_else(|err| panic!("Cannot parse date {}: {err}", e.start.date_time))
            .unix_timestamp();
        let msg = THREE_DAYS
            .replace("@announce@", "📢 ¡Atencion @anuncios !")
            .replace("@title@", &format!("**{}**", e.summary))
            .replace(
                "@description@",
                &e.description
                    .clone()
                    .map(|d| format!("{}\n\n", html_to_md(&d)))
                    .unwrap_or_default(),
            )
            .replace("@date@", &format!("📅 Fecha: <t:{timestamp}:D>"))
            .replace("@hour@", &format!("🕓 Hora: <t:{timestamp}:t>"))
            .replace(
                "@location@",
                &e.location
                    .clone()
                    .map(|l| format!("📍 Lugar: {l}\n"))
                    .unwrap_or_default(),
            )
            .replace("@link@", &format!("🖥️ Enlace: <{}>", e.html_link));
        send(client, endpoint, apikey, &msg, roles, channel).await;
    }

    let events = one_hour
        .iter()
        .map(|(_, e)| format!("- [{}](<{}>)", e.summary, e.html_link))
        .collect::<Vec<String>>()
        .join("\n");

    if one_hour.is_empty() {
        return;
    }

    let start = if one_hour.len() > 1 {
        "comienzan los eventos de"
    } else {
        "comienza el evento"
    };

    let msg = ONE_HOUR
        .replace("@start@", start)
        .replace("@events@", &events);
    send(client, endpoint, apikey, &msg, roles, bot_channel).await;
}

fn html_to_md(s: &str) -> String {
    let mut custom_parser: HashMap<String, Box<dyn TagHandlerFactory>> = HashMap::new();
    custom_parser
        .entry("img".to_owned())
        .or_insert(Box::<DefaultFactory<ImgHandler>>::default());
    custom_parser
        .entry("a".to_owned())
        .or_insert(Box::<DefaultFactory<LinkHandler>>::default());

    parse_html_custom(s, &custom_parser)
}

#[derive(Default)]
struct ImgHandler;
impl TagHandler for ImgHandler {
    fn handle(&mut self, tag: &html2md::Handle, printer: &mut html2md::StructuredPrinter) {
        let src = get_tag_attr(tag, "src").unwrap_or_default();
        let alt = get_tag_attr(tag, "alt");

        console_debug!("SRC image content: {src}");
        console_debug!("ALT image content: {alt:?}");

        if let Some(alt) = alt {
            printer.append_str(&format!("![{}]({})", alt, &src));
        } else {
            printer.append_str(&src);
        }
    }

    fn after_handle(&mut self, _printer: &mut html2md::StructuredPrinter) {}
}

#[derive(Default)]
struct LinkHandler {
    start_pos: usize,
    url: String,
}
impl TagHandler for LinkHandler {
    fn handle(&mut self, tag: &html2md::Handle, printer: &mut html2md::StructuredPrinter) {
        self.start_pos = printer.data.len();

        // try to extract a hyperlink
        self.url = match tag.data {
            html2md::NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let href = attrs
                    .iter()
                    .find(|attr| attr.name.local.to_string() == "href");
                match href {
                    Some(link) => link.value.to_string(),
                    None => "https://rustlang-es.org".to_owned(),
                }
            }
            _ => "https://rustlang-es.org".to_owned(),
        };
    }

    fn after_handle(&mut self, printer: &mut html2md::StructuredPrinter) {
        // add braces around already present text, put an url afterwards
        printer.insert_str(self.start_pos, "[");
        printer.append_str(&format!("](<{}>)", self.url))
    }
}

#[derive(Default)]
struct DefaultFactory<T> {
    _marker: PhantomData<T>,
}
impl<T> TagHandlerFactory for DefaultFactory<T>
where
    T: 'static + Default + TagHandler,
{
    fn instantiate(&self) -> Box<dyn TagHandler> {
        Box::<T>::default()
    }
}
