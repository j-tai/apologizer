mod config;
mod service;

use crate::config::load_config;
use axum::handler::HandlerWithoutStateExt;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, trace};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    // Set up logging
    const DEFAULT_FILTER: &str = if cfg!(debug_assertions) {
        concat!(env!("CARGO_CRATE_NAME"), "=trace,info")
    } else {
        concat!(env!("CARGO_CRATE_NAME"), "=info,warn")
    };

    let subscriber = tracing_subscriber::registry().with(
        std::env::var("APOLOGIZER_LOG")
            .map(EnvFilter::new)
            .unwrap_or_else(|_| EnvFilter::try_new(DEFAULT_FILTER).unwrap()),
    );

    match std::env::var("APOLOGIZER_LOG_TO").as_deref() {
        Ok("journald") => subscriber.with(tracing_journald::layer().unwrap()).init(),
        Ok("stdout") => subscriber.with(tracing_subscriber::fmt::layer()).init(),
        _ => subscriber
            .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
            .init(),
    }
    trace!("Logging set up.");

    let config = load_config("apologizer.toml".as_ref()).await;

    let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), config.port))
        .await
        .unwrap();
    info!("Listening on port {}.", config.port);
    
    let config = Arc::new(config);
    axum::serve(
        listener,
        (move |req| service::handle(config, req)).into_make_service(),
    )
    .await
    .unwrap();
}
