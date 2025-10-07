use anyhow::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YFinanceError {
    #[error("Failed to fetch history for {ticker}: {source}")]
    FetchFailed { ticker: String, source: Error },
}
