use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    if is_leap_year {
        return None;
    }

    let middle_day = NaiveDate::from_ymd_opt(year, 7, 2)?; 
    Some(middle_day.weekday())
}