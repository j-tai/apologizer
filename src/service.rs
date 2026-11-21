use std::sync::Arc;
use crate::config::Config;
use axum::extract::Request;
use axum::http::StatusCode;
use tracing::info;

pub async fn handle(config: Arc<Config>, request: Request) -> (StatusCode, String) {
    info!("{} {}", request.method(), request.uri());

    let status = StatusCode::from_u16(config.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE);
    let body = config
        .body
        .clone()
        .unwrap_or_else(|| status.to_string());

    (status, body)
}
