use crate::app::app_box::AppBox;
use crate::data::ticker_data::TickerData;
use crate::requests::tk_request::{convert_to_string, TickerRequest};
use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use std::time::Duration;
use tokio::time;
use tracing::info;

pub async fn send_data_to_socket(
    socket: &mut WebSocket,
    app: &AppBox,
    tk_request: &TickerRequest,
) -> Result<()> {
    let ticker = TickerData::new();
    let tk_data = ticker
        .fetch_data(tk_request, app)
        .await
        .context("Fetching ticker data failed")?;

    for tk_params in tk_data {
        let tk_params_str = convert_to_string(&tk_params)?;
        let _ = socket.send(Message::Text(tk_params_str.into())).await;
        time::sleep(Duration::from_millis(100)).await;
    }

    info!(
        tk_request.ticker = %tk_request.ticker,
        "Data stream sent"
    );

    Ok(())
}
