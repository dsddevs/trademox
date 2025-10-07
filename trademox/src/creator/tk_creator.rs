use crate::errors::tk_errors::TickerDateRangeError::InvalidDateRange;
use anyhow::Result;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::info;
use yfinance_rs::Candle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerParams {
    pub time: NaiveDate,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerBox;

impl TickerBox {
    pub fn new() -> Self {
        Self
    }
    pub async fn create_ticker_data(
        &self,
        history_data: Vec<Candle>,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<TickerParams>> {
        if start > end {
            return Err(InvalidDateRange { start, end }.into());
        }

        let ticker_data = history_data
            .into_iter()
            .map(|h| TickerParams {
                time: h.ts.date_naive(),
                open: h.open.amount(),
                high: h.high.amount(),
                low: h.low.amount(),
                close: h.close.amount(),
            })
            .collect();

        info!(start = %start, end = %end, "Created creator data");
        Ok(ticker_data)
    }
}
