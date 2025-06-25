use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub type Result<T> = core::result::Result<T, DateError>;

#[derive(Error, Debug)]
pub enum DateError {
    #[error("Failed to parse! Date: {0}")]
    FailToDateParse(String),
}

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

pub fn now_utc_plus_sec_str(sec: &u64) -> String {
    let new_time = now_utc() + Duration::seconds(*sec as i64);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| DateError::FailToDateParse(moment.to_string()))
}
