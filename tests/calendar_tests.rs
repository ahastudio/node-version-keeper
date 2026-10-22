use node_version_keeper::calendar::{format_date_from_epoch_days, today_utc};

#[test]
fn formats_the_epoch_day_zero() {
    assert_eq!(format_date_from_epoch_days(0), "1970-01-01");
}

#[test]
fn formats_the_day_before_the_epoch() {
    assert_eq!(format_date_from_epoch_days(-1), "1969-12-31");
}

#[test]
fn formats_a_leap_day_and_the_day_after_it() {
    assert_eq!(format_date_from_epoch_days(11_016), "2000-02-29");
    assert_eq!(format_date_from_epoch_days(11_017), "2000-03-01");
}

#[test]
fn formats_a_recent_date() {
    assert_eq!(format_date_from_epoch_days(20_748), "2026-10-22");
}

#[test]
fn today_has_the_date_format() {
    let today = today_utc();
    let parts: Vec<&str> = today.split('-').collect();
    assert_eq!(today.len(), 10);
    assert_eq!(parts.iter().map(|part| part.len()).collect::<Vec<_>>(), vec![4, 2, 2]);
}
