use crate::app::app_box::AppBox;
use crate::requests::tk_request::TickerRequest;
use crate::requests::tk_request_checker::{
    get_default_or_end, get_default_or_start, get_default_or_ticker,
};
use crate::websocket::ws_sender::send_data_to_socket;
use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use axum::Error;
use serde_json::from_str;

pub async fn handle_socket_message(
    socket: &mut WebSocket,
    app: &AppBox,
    msg: Result<Message, Error>,
) -> Result<()> {
    let msg = msg?;

    match msg {
        Message::Close(_) => return Ok(()),
        Message::Text(text) => {
            let ticker_req: TickerRequest = from_str(&text).context("Invalid JSON")?;
            let ticker = get_default_or_ticker(&ticker_req).context("Invalid ticker")?;
            let start = get_default_or_start(&ticker_req).context("Invalid start")?;
            let end = get_default_or_end(&ticker_req).context("Invalid end")?;
            let ticker_req_init = TickerRequest { ticker, start, end };
            send_data_to_socket(socket, &app, &ticker_req_init)
                .await
                .context("Sending data to socket failed")?;
        }

        Message::Ping(p) => {
            socket.send(Message::Pong(p)).await?;
        }
        Message::Pong(_) => {}
        _ => {}
    }
    Ok(())
}
