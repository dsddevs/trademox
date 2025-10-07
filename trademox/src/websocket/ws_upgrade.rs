use crate::app::app_box::AppBox;
use crate::errors::api_error::ApiError;
use crate::requests::tk_request::TickerRequest;
use crate::requests::tk_request_validator::{validate_date, validate_ticker};
use crate::websocket::ws_handler::handle_socket_data;
use axum::{
    extract::{rejection::QueryRejection, Query, State, WebSocketUpgrade},
    response::IntoResponse,
};
use tracing::{info, span, Instrument, Level};

pub async fn upgrade_socket_data(
    tk_query: Result<Query<TickerRequest>, QueryRejection>,
    ws_upgrade: WebSocketUpgrade,
    State(app): State<AppBox>,
) -> impl IntoResponse {
    match tk_query {
        Ok(query) => {
            //validation of ticker request params
            if let Err(e) = validate_ticker(&query.0.ticker) {
                return ApiError::invalid_ticker(e.to_string()).into_response();
            }
            if let Err(e) = validate_date(&query.0.start) {
                return ApiError::invalid_date(e.to_string(), "start".to_string()).into_response();
            }
            if let Err(e) = validate_date(&query.0.end) {
                return ApiError::invalid_date(e.to_string(), "end".to_string()).into_response();
            }

            // logging of success response
            info!(
                ticker = %query.0.ticker,
                start = %query.0.start,
                end = %query.0.end,
                "WebSocket upgrade successful"
            );

            // WebSocket upgrading
            ws_upgrade.on_upgrade(move |socket| {
                let span = span!(Level::INFO, "ws_session");
                handle_socket_data(socket, app, query.0).instrument(span)
            })
        }

        Err(rej) => ApiError::invalid_query(
            "Invalid query parameters format".to_string(),
            Some(format!("{:?}", rej)),
        )
        .into_response(),
    }
}
