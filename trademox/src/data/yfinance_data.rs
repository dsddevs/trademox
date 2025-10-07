use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use yfinance_rs::Interval::D1;
use yfinance_rs::{Candle, Range, Ticker, YfClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YFinanceData;

impl YFinanceData {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_history_data(
        &self,
        ticker: &String,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Candle>> {
        let mut all_candles = Vec::new();
        let mut current_start = start;

        while current_start < end {
            let chunk_end = (current_start + Duration::days(365)).min(end);

            let chunk = self
                .fetch_daily_chunk(ticker, current_start, chunk_end)
                .await?;
            all_candles.extend(chunk);

            current_start = chunk_end + Duration::days(1);
        }

        Ok(all_candles)
    }

    async fn fetch_daily_chunk(
        &self,
        ticker: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Candle>> {
        let yf_client = YfClient::default();
        let yf_ticker = Ticker::new(&yf_client, ticker);

        let start_dt = start.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end_dt = end.and_hms_opt(0, 0, 0).unwrap().and_utc();

        let history_builder = yf_ticker
            .history_builder()
            .range(Range::Max)
            .interval(D1)
            .auto_adjust(true)
            .prepost(false)
            .actions(true);

        let history_builder = history_builder.between(start_dt, end_dt);

        history_builder
            .fetch()
            .await
            .context("Error: History builder is failed")
    }
}
