#[cfg(test)]
mod tests {
    use axum::{http::StatusCode, routing::get, serve, Router};
    use futures_util::StreamExt;
    use tokio::{
        net::TcpListener,
        spawn,
        sync::oneshot::{channel, Sender},
        time::{timeout, Duration},
    };
    use tokio_tungstenite::connect_async;
    use tradebox::{build_app, websocket::ws_upgrade::upgrade_socket_data};

    async fn start_test_server() -> (String, Sender<()>) {
        let app = build_app();
        let router = Router::new()
            .route("/ws", get(upgrade_socket_data))
            .with_state(app);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (sender, receiver) = channel::<()>();

        spawn(async move {
            serve(listener, router)
                .with_graceful_shutdown(async {
                    receiver.await.ok();
                })
                .await
                .unwrap();
        });

        (format!("127.0.0.1:{}", addr.port()), sender)
    }

    #[tokio::test]
    async fn test_websocket_upgrade_success() {
        let (addr, shutdown_tx) = start_test_server().await;
        let url = format!(
            "ws://{}/ws?ticker=BTC-USD&start=2023-01-01&end=2023-01-10",
            addr
        );

        let (mut ws_stream, _) = connect_async(&url).await.expect("Failed to connect");

        match timeout(Duration::from_secs(5), ws_stream.next()).await {
            Ok(Some(Ok(msg))) => assert!(msg.is_text()),
            Ok(None) => panic!("Connection closed unexpectedly"),
            Ok(Some(Err(e))) => panic!("WebSocket error: {}", e),
            Err(_) => println!("Timeout waiting for message, but connection established"),
        }

        let _ = shutdown_tx.send(());
    }

    #[tokio::test]
    async fn test_websocket_invalid_ticker() {
        let (addr, shutdown_tx) = start_test_server().await;
        let invalid_tickers = vec!["INVALID$", "1NV@LID", "TOO_LONG_TICKER_123"];

        for ticker in invalid_tickers {
            let url = format!(
                "ws://{}/ws?ticker={}&start=2025-01-01&end=2025-01-10",
                addr, ticker
            );

            let result = connect_async(&url).await;
            assert!(
                result.is_err(),
                "Server should reject invalid ticker: {}",
                ticker
            );
        }

        let _ = shutdown_tx.send(());
    }

    async fn make_websocket_request(client: &reqwest::Client, url: &str) -> reqwest::Response {
        client
            .get(url)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
            .send()
            .await
            .expect("HTTP request failed")
    }

    #[tokio::test]
    async fn test_http_invalid_query_params() {
        let (addr, shutdown_tx) = start_test_server().await;
        let client = reqwest::Client::new();

        let test_cases = vec![
            (
                "ticker=INVALID$&start=2023-01-01&end=2023-01-10",
                "InvalidTicker",
            ),
            (
                "ticker=123&start=2023-01-01&end=2023-01-10",
                "InvalidTicker",
            ),
            (
                "ticker=BTC-USD&start=not-a-date&end=2023-01-10",
                "InvalidDate",
            ),
            (
                "ticker=BTC-USD&start=2023-01-01&end=not-a-date",
                "InvalidDate",
            ),
            ("invalid=param", "InvalidTicker"),
        ];

        for (query, expected_error) in test_cases {
            let url = format!("http://{}/ws?{}", addr, query);
            println!("Testing: {}", url);

            let response = make_websocket_request(&client, &url).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);

            let body: serde_json::Value = response.json().await.expect("Valid JSON expected");
            println!("Response: {}", serde_json::to_string_pretty(&body).unwrap());

            assert_eq!(
                body["error"], expected_error,
                "For query '{}': expected '{}', got '{}'",
                query, expected_error, body["error"]
            );
        }

        let _ = shutdown_tx.send(());
    }
}
