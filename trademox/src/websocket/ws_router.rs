use crate::app::app_box::AppBox;
use crate::configs::yaml_config::YamlConfig;
use crate::websocket::ws_upgrade::upgrade_socket_data;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::get;
use axum::Router;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_socket_router(app: AppBox) -> Router {
    let default_origin = env::var("DEFAULT_ORIGIN").ok().unwrap();
    let origins = match YamlConfig::from_file("config.yaml") {
        Ok(yaml_config) => yaml_config.cors.origins,
        Err(_) => vec![default_origin],
    };

    let allowed_origins = origins
        .into_iter()
        .filter_map(|ao| HeaderValue::from_str(&ao).ok())
        .collect::<Vec<_>>();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    Router::new()
        .route("/ws", get(upgrade_socket_data))
        .route("/healthz", get(health_check))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
