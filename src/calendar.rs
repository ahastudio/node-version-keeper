use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_DAY: u64 = 86_400;

// Returns today's date in UTC as "YYYY-MM-DD"
pub fn today_utc() -> String {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    format_date_from_epoch_days((elapsed.as_secs() / SECONDS_PER_DAY) as i64)
}

pub fn format_date_from_epoch_days(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let (year_of_era, day_of_year) = year_and_day_of_year(day_of_era);
    month_and_day(era * 400 + year_of_era, day_of_year)
}

fn year_and_day_of_year(day_of_era: i64) -> (i64, i64) {
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    (year_of_era, day_of_year)
}

fn month_and_day(year: i64, day_of_year: i64) -> (i64, i64, i64) {
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 { month_index + 3 } else { month_index - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}
