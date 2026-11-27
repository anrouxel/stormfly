use jiff::{Timestamp, tz::TimeZone};

use crate::pac::date_time::get_day::get_day;

pub fn weekday_range(args: &[&str]) -> bool {
    let (days, utc) = match args.split_last() {
        Some((&"GMT", days)) => (days, true),
        Some(_) => (args, false),
        None => return false,
    };

    let Some(start) = days.first() else {
        return false;
    };
    let end = if days.len() == 2 { &days[1] } else { start };

    let (Some(start), Some(end)) = (get_day(start), get_day(end)) else {
        return false;
    };

    let zone = match utc {
        true => TimeZone::UTC,
        false => TimeZone::system(),
    };
    let today = Timestamp::now().to_zoned_datetime(zone).weekday();

    today.since(start) <= end.since(start)
}
