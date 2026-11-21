use crate::config::Config;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::Html;
use handlebars::Handlebars;
use std::sync::Arc;
use tracing::info;

pub type PreparedResponse = (StatusCode, Html<String>);

pub fn prepare_response(config: &Config) -> PreparedResponse {
    let status = StatusCode::from_u16(config.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE);

    let mut handlebars = Handlebars::new();
    if cfg!(debug_assertions) {
        handlebars.set_dev_mode(true);
        handlebars
            .register_template_file("generic", "src/generic.hbs")
            .unwrap();
    } else {
        handlebars
            .register_template_string("generic", include_str!("generic.hbs"))
            .unwrap();
    }

    let context = serde_json::json!({
        "status": status.to_string(),
        "status_code": status.as_u16(),
        "status_text": status.canonical_reason(),
        "body": config.body,
    })
    .into();

    let body = handlebars.render_with_context("generic", &context).unwrap();
    (status, Html(body))
}

pub async fn handle(
    config: Arc<Config>,
    response: PreparedResponse,
    request: Request,
) -> PreparedResponse {
    info!("{} {}", request.method(), request.uri());

    if cfg!(debug_assertions) {
        prepare_response(&config)
    } else {
        response
    }
}
