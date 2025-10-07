use crate::app::app_box::AppBox;
use crate::errors::ws_errors::send_error_to_socket;
use crate::requests::tk_request::TickerRequest;
use crate::requests::tk_request_checker::{
    get_default_or_end, get_default_or_start, get_default_or_ticker,
};
use crate::websocket::ws_msg_handler::handle_socket_message;
use crate::websocket::ws_sender::send_data_to_socket;
use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use axum::Error;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

pub async fn handle_socket_data(mut socket: WebSocket, app: AppBox, ticker_req: TickerRequest) {
    info!("New websocket connection");

    let ticker_req_init = init_ticker_request(&ticker_req)
        .await
        .unwrap_or_else(|_| TickerRequest::new());

    if let Err(e) = send_data_to_socket(&mut socket, &app, &ticker_req_init).await {
        send_error_to_socket(&mut socket, &e).await;
        error!(error = %e, "Initial ticker request send failed");
    }

    let mut ping_interval = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
                received_msg = socket.recv() => {
                    if !handle_received_msg(&mut socket, &app, received_msg).await {
                        break
                    }
            }
              _ = ping_interval.tick() => {
                let _ = socket.send(Message::Ping(vec![].into())).await;
              }
        }

        info!("WebSocket session ended");
    }

    async fn init_ticker_request(ticker_req: &TickerRequest) -> Result<TickerRequest> {
        Ok(TickerRequest {
            ticker: get_default_or_ticker(ticker_req)?,
            start: get_default_or_start(ticker_req)?,
            end: get_default_or_end(ticker_req)?,
        })
    }

    async fn handle_received_msg(
        socket: &mut WebSocket,
        app: &AppBox,
        received_msg: Option<Result<Message, Error>>,
    ) -> bool {
        let Some(Ok(msg)) = received_msg else {
            return false;
        };

        match handle_socket_message(socket, app, Ok(msg)).await {
            Ok(()) => true,
            Err(e) => {
                error!(error = %e, "Message handling failed");
                false
            }
        }
    }
}
