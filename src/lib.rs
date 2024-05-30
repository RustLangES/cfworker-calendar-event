use reqwest::ClientBuilder;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};
use worker::{console_log, event, Env, ScheduleContext, ScheduledEvent};

pub mod calendar;
pub mod cangrebot;

#[cfg(target_arch = "wasm32")]
use worker::{console_debug, console_error};

#[derive(Debug, PartialEq, Eq)]
pub enum EventDateType {
    ThreeDays,
    OneHour,
}

#[event(start)]
fn start() {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}");
    }));
}

#[event(scheduled)]
pub async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let calendar_api = env
        .secret("GOOGLE_APIKEY")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let calendar_id = env
        .secret("GOOGLE_CALENDAR_ID")
        .map(|e| e.to_string())
        .expect("Calendar Google ID Secret not found");

    let endpoint = env
        .secret("ENDPOINT")
        .map(|e| e.to_string())
        .expect("Endpoint Secret not found");

    let channel = env
        .secret("CHANNEL_ID")
        .map(|e| {
            e.to_string()
                .parse::<i64>()
                .expect("Cannot parse CHANNEL_ID")
        })
        .expect("Announce Channel ID Secret not found");

    let bot_key = env
        .secret("BOT_APIKEY")
        .map(|e| e.to_string())
        .expect("Bot APIKEY Secret not found");

    let bot_channel = env
        .secret("BOT_CHANNEL_ID")
        .map(|e| {
            e.to_string()
                .parse::<i64>()
                .expect("Cannot parse CHANNEL_ID")
        })
        .expect("Remember Channel ID Secret not found");

    let roles = env
        .secret("ROLES")
        .map(|e| {
            e.to_string()
                .split_terminator(';')
                .map(|r| r.parse::<i64>().expect(&format!("Cannot parse role '{r}'")))
                .collect::<Vec<_>>()
        })
        .expect("Roles to mention Secret not found");

    let client = ClientBuilder::default()
        .build()
        .expect("Cannot build client reqwest");

    let now = OffsetDateTime::now_utc();
    // 3 days, but 00:00UTC need +1
    let next_days = now
        .checked_add(Duration::days(4))
        .expect("Cannot get the next days");

    // Get events
    let get = calendar::get(
        &client,
        &calendar_id,
        &calendar_api,
        &now.format(&Rfc3339).expect("Cannot format min_date (now)"),
        &next_days
            .format(&Rfc3339)
            .expect("Cannot format max_date (next 3 days)"),
    )
    .await;

    let (three_days, one_hour): (Vec<(_, _)>, Vec<(_, _)>) = get
        .iter()
        .filter_map(|e| compare_dates(&e.start.date_time, &now).map(|ev| (ev, e.clone())))
        .partition(|(ev, _)| ev == &EventDateType::ThreeDays);

    cangrebot::build_message(
        &client,
        &endpoint,
        &bot_key,
        &three_days,
        &one_hour,
        &roles,
        channel,
        bot_channel,
    )
    .await;
}

pub fn compare_dates(event_date: &str, now: &OffsetDateTime) -> Option<EventDateType> {
    let event_date = OffsetDateTime::parse(event_date, &Rfc3339)
        .expect(&format!("Cannot parse date {event_date}"))
        .replace_minute(0)
        .expect(&format!("Cannot replace minutes from event date"));
    let days = event_date.checked_sub(Duration::days(3)).unwrap();
    let hours = event_date.checked_sub(Duration::hours(1)).unwrap();

    #[cfg(target_arch = "wasm32")]
    console_log!("Days new date: {days} - Now Date: {now}");
    #[cfg(target_arch = "wasm32")]
    console_log!("Hours new date: {hours} - Now Date: {now}");

    if days.day() == now.day() && now.hour() == 22 {
        return Some(EventDateType::ThreeDays);
    }
    if hours == *now {
        return Some(EventDateType::OneHour);
    }

    None
}
