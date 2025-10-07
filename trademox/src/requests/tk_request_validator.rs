use anyhow::{anyhow, Context, Error, Result};
use chrono::NaiveDate;
use tracing::log::info;

pub fn validate_ticker(ticker: &str) -> Result<(), Error> {
    info!("Validating ticker: '{}'", ticker);
    if ticker.is_empty() {
        return Err(anyhow!("Ticker cannot be empty"));
    }
    if ticker.len() > 10 {
        return Err(anyhow!("Ticker too long"));
    }
    if !ticker.chars().all(|c| c.is_ascii_uppercase() || c == '-') {
        return Err(anyhow!("Invalid ticker format"));
    }
    Ok(())
}

pub fn validate_date(date: &str) -> Result<(), Error> {
    info!("Validating date: '{}'", date);
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| anyhow!("Invalid date format"))
}

pub fn ensure_date_fmt(df: &str, field: &str) -> Result<String> {
    let err_msg = format!("Invalid {field} date format, expected %Y-%m-%d: {df}");
    NaiveDate::parse_from_str(df, "%Y-%m-%d").with_context(|| err_msg)?;
    Ok(df.to_string())
}
