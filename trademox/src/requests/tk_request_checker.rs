use crate::requests::tk_request::TickerRequest;
use crate::requests::tk_request_validator::ensure_date_fmt;
use anyhow::Result;
use std::env::var;

pub fn get_default_or_ticker(t_request: &TickerRequest) -> Result<String> {
    let ticker_req = t_request.ticker.trim();
    let default_ticker = var("DEFAULT_TICKER")?;
    let val = if ticker_req.is_empty() {
        default_ticker
    } else {
        ticker_req.to_string()
    };
    Ok(val)
}

pub fn get_default_or_start(t_request: &TickerRequest) -> Result<String> {
    let start_req = t_request.start.trim();
    let default_start = var("DEFAULT_START")?;
    let val = if start_req.is_empty() {
        default_start
    } else {
        start_req.to_string()
    };
    ensure_date_fmt(&val, "start")
}

pub fn get_default_or_end(t_request: &TickerRequest) -> Result<String> {
    let end_req = t_request.end.trim();
    let default_end = var("DEFAULT_END")?;
    let val = if end_req.is_empty() {
        default_end
    } else {
        end_req.to_string()
    };
    ensure_date_fmt(&val, "end")
}
