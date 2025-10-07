use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TickerDateRangeError {
    #[error("Invalid date range: start {start} > end {end}")]
    InvalidDateRange { start: NaiveDate, end: NaiveDate },
}
