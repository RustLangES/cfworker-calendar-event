use time::format_description::well_known::Rfc3339;
use time::{Date, Duration, OffsetDateTime, Time};

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
