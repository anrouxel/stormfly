use jiff::civil::Weekday;

pub fn get_day(weekday: &str) -> Option<Weekday> {
    match weekday {
        "SUN" => Some(Weekday::Sunday),
        "MON" => Some(Weekday::Monday),
        "TUE" => Some(Weekday::Tuesday),
        "WED" => Some(Weekday::Wednesday),
        "THU" => Some(Weekday::Thursday),
        "FRI" => Some(Weekday::Friday),
        "SAT" => Some(Weekday::Saturday),
        _ => None,
    }
}
