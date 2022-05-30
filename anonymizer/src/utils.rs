use chrono::{NaiveDate, ParseResult};

pub fn is_dcm_path(path: &str) -> bool {
    path.ends_with(".dcm")
}

pub fn parse_date(value: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
}
