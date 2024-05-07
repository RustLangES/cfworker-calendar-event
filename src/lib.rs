use reqwest::ClientBuilder;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};
use worker::{console_debug, event, Env, ScheduleContext, ScheduledEvent};

mod calendar;
mod cangrebot;

#[cfg(target_arch = "wasm32")]
use worker::console_error;

#[event(scheduled)]
pub async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    let calendar_api = env
        .secret("GOOGLE_APIKEY")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let calendar_id = env
        .secret("GOOGLE_CALENDAR_ID")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let endpoint = env
        .secret("ENDPOINT")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let client = ClientBuilder::default()
        .build()
        .expect("Cannot build client reqwest");

    let now = OffsetDateTime::now_utc();
    // 3 days, but 00:00UTC need +1
    let next_days = now
        .checked_add(Duration::days(4))
        .expect("Cannot get the next days");

    // Get events
    // TODO: filter and map events
    let events = calendar::get(
        &client,
        &calendar_id,
        &calendar_api,
        &now.format(&Rfc3339).expect("Cannot format min_date (now)"),
        &next_days
            .format(&Rfc3339)
            .expect("Cannot format max_date (next 3 days)"),
    )
    .await
    .iter()
    .filter(|e| compare_dates(&e.start.date_time, &now));

    cangrebot::send(&client, &endpoint).await;
}

fn compare_dates(event_date: &str, now: &OffsetDateTime) -> bool {
    let event_date = OffsetDateTime::parse(event_date, &Rfc3339)
        .expect(&format!("Cannot parse date {event_date}"));
    let diff = now.date() - event_date.date();

    console_debug!(
        "Days: {} - Hours: {}",
        diff.whole_days(),
        diff.whole_hours()
    );

    diff.whole_days() == -3 || diff.whole_hours() <= 1 && diff.whole_hours() >= 0
}

#[cfg(test)]
mod test {
    use time::format_description::well_known::Rfc3339;
    use time::{Date, Duration, OffsetDateTime, Time};

    use crate::compare_dates;

    #[test]
    fn test_format_date_rfc3339() {
        let d = Date::from_calendar_date(2024, time::Month::May, 6).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT).format(&Rfc3339);

        assert!(d.is_ok());

        let d = d.unwrap();
        assert_eq!(d.as_str(), "2024-05-06T00:00:00Z");
    }

    #[test]
    fn test_format_next_date_rfc3339() {
        let d = Date::from_calendar_date(2024, time::Month::May, 6).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);

        let d = d.checked_add(Duration::days(3));
        assert!(d.is_some());

        let d = d.unwrap();
        assert_eq!(d.day(), 9);

        let d = d.format(&Rfc3339);
        assert_eq!(&d.unwrap(), "2024-05-09T00:00:00Z");
    }

    #[test]
    fn compare_dates_more_time() {
        let d = Date::from_calendar_date(2024, time::Month::May, 5).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
        let res = compare_dates("2024-05-09T00:00:00Z", &d);

        assert!(!res);
    }

    #[test]
    fn compare_dates_tree_days() {
        let d = Date::from_calendar_date(2024, time::Month::May, 6).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
        let res = compare_dates("2024-05-09T00:00:00Z", &d);

        assert!(res);
    }

    #[test]
    fn compare_dates_one_hour() {
        let d = Date::from_calendar_date(2024, time::Month::May, 9).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
        let res = compare_dates("2024-05-09T01:10:00Z", &d);

        assert!(res);
    }

    #[test]
    fn compare_dates_thirty_minutes() {
        let d = Date::from_calendar_date(2024, time::Month::May, 9).unwrap();
        let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
        let res = compare_dates("2024-05-09T00:34:00Z", &d);

        assert!(res);
    }
}
