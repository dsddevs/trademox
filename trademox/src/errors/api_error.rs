use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub error_type: String,
    pub message: String,
    pub details: Option<String>,
    pub field: Option<String>,
}

impl ApiError {
    pub fn invalid_ticker(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error_type: "InvalidTicker".to_string(),
            message,
            details: None,
            field: Some("ticker".to_string()),
        }
    }

    pub fn invalid_date(message: String, field: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error_type: "InvalidDate".to_string(),
            message,
            details: None,
            field: Some(field),
        }
    }

    pub fn invalid_query(message: String, details: Option<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error_type: "InvalidQuery".to_string(),
            message,
            details,
            field: None,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut json_body = json!({
            "error": self.error_type,
            "message": self.message
        });

        if let Some(field) = self.field {
            json_body["field"] = json!(field);
        }

        if let Some(details) = self.details {
            json_body["details"] = json!(details);
        }

        (self.status, Json(json_body)).into_response()
    }
}
