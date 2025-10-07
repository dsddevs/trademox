use crate::app::app_box::AppBox;
use crate::creator::tk_creator::TickerParams;
use crate::errors::{tk_errors::TickerDateRangeError::InvalidDateRange, yf_error::YFinanceError};
use crate::requests::tk_request::TickerRequest;
use anyhow::{anyhow, Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData;

impl TickerData {
    pub fn new() -> Self {
        Self
    }
    pub async fn fetch_data(
        &self,
        tk_request: &TickerRequest,
        app: &AppBox,
    ) -> Result<Vec<TickerParams>> {
        let start = NaiveDate::parse_from_str(&tk_request.start, "%Y-%m-%d")
            .context("Invalid start date")?;
        let end =
            NaiveDate::parse_from_str(&tk_request.end, "%Y-%m-%d").context("Invalid end date")?;

        let history_data = app
            .yfinance_data
            .fetch_history_data(&tk_request.ticker, start, end)
            .await
            .context(YFinanceError::FetchFailed {
                ticker: tk_request.ticker.to_string(),
                source: anyhow!("Context"),
            })?;

        let ticker_data = app
            .ticker_box
            .create_ticker_data(history_data, start, end)
            .await
            .context(InvalidDateRange { start, end })?;

        Ok(ticker_data)
    }
}
