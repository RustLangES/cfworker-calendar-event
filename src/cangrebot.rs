use reqwest::Client;
use serde_json::json;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use worker::{console_debug, console_warn};

use crate::calendar::Event;
use crate::EventDateType;

const DATE_FORMAT: &str = "📅 Fecha: [day padding:none] de [month repr:long] del [year repr:full]";
const TIME_FORMAT: &str =
    "🕓 Hora: [hour padding:zero repr:12]:[minute padding:zero][period case:lower] [zone]";

const THREE_DAYS: &str = r#"@announce@

@anuncios

@description@@date@
@hour@
@location@@link@"#;

const ONE_HOUR: &str = r#":warning: :warning: Atenciooon!!!! :warning: :warning: 
En una hora @start@:

@events@

> **NOTA:** Haciendo click en cada evento pueden ver los detalles en el calendario
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
        let date = OffsetDateTime::parse(&e.start.date_time, &Rfc3339)
            .expect(&format!("Cannot parse date {}", e.start.date_time));
        let msg = THREE_DAYS
            .replace("@announce@", "📢 ¡Este martes 2 de abril!")
            .replace("@title@", &format!("**{}**", e.summary))
            .replace(
                "@description@",
                &e.description
                    .clone()
                    .map(|d| format!("{d}\n"))
                    .unwrap_or_default(),
            )
            .replace(
                "@date@",
                &date
                    .format(
                        &time::format_description::parse(DATE_FORMAT)
                            .expect("Cannot parse human format date"),
                    )
                    .expect("Cannot format human date"),
            )
            .replace(
                "@hour@",
                &date
                    .time()
                    .format(
                        &time::format_description::parse(TIME_FORMAT)
                            .expect("Cannot parse human format time"),
                    )
                    .expect("Cannot format human time"),
            )
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
