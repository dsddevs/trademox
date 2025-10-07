use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use serde_json::json;

pub async fn send_error_to_socket(socket: &mut WebSocket, e: &anyhow::Error) {
    let error_msg: Utf8Bytes = json!({"error": e.to_string()}).to_string().into();
    let msg_text = Message::text(error_msg);
    let _ = socket.send(msg_text).await;
}
