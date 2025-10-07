use crate::creator::tk_creator::TickerParams;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::to_string;
use std::env;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TickerRequest {
    #[serde(default)]
    pub ticker: String,
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub end: String,
}

impl TickerRequest {
    pub fn new() -> Self {
        Self {
            ticker: env::var("DEFAULT_TICKER".to_string()).unwrap_or_else(|_| "XAUUSD".to_string()),
            start: env::var("DEFAULT_START".to_string())
                .unwrap_or_else(|_| "2025-01-01".to_string()),
            end: env::var("DEFAULT_END".to_string()).unwrap_or_else(|_| "2025-02-01".to_string()),
        }
    }
}

pub fn convert_to_string(tk_params: &TickerParams) -> Result<String> {
    to_string(tk_params)
        .context("JSON -> String serialization failed")
        .map_err(Into::into)
}
