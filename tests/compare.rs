use calendar_events::{compare_dates, EventDateType};
use time::{Date, OffsetDateTime, Time};

#[test]
fn compare_dates_more_time() {
    let d = Date::from_calendar_date(2024, time::Month::May, 5).unwrap();
    let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
    let res = compare_dates("2024-05-09T00:00:00Z", &d);

    assert!(res.is_none());
}

#[test]
fn compare_dates_three_days_fail() {
    let d = Date::from_calendar_date(2024, time::Month::May, 6).unwrap();
    let d = OffsetDateTime::new_utc(d, Time::MIDNIGHT);
    let res = compare_dates("2024-05-09T01:00:00Z", &d);

    assert!(res.is_none());
}

#[test]
fn compare_dates_three_days() {
    let d = Date::from_calendar_date(2024, time::Month::May, 6).unwrap();
    let d = OffsetDateTime::new_utc(d, Time::from_hms(22, 00, 00).unwrap());
    let res = compare_dates("2024-05-09T00:00:00Z", &d);

    assert_eq!(res, Some(EventDateType::ThreeDays));
}

#[test]
fn compare_dates_one_hour() {
    let d = Date::from_calendar_date(2024, time::Month::May, 9).unwrap();
    let d = OffsetDateTime::new_utc(d, Time::from_hms(8, 0, 0).unwrap());
    let res = compare_dates("2024-05-09T09:10:00Z", &d);

    assert_eq!(res, Some(EventDateType::OneHour));
}

#[test]
fn compare_dates_one_hour_with_seconds() {
    let d = Date::from_calendar_date(2024, time::Month::May, 9).unwrap();
    let d = OffsetDateTime::new_utc(d, Time::from_hms_milli(8, 0, 0, 17).unwrap());
    let res = compare_dates("2024-05-09T09:10:00Z", &d);

    assert_eq!(res, Some(EventDateType::OneHour));
}
