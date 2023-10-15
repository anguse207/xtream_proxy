pub mod config;

use axum::{
    body::StreamBody,
    extract::{ConnectInfo, OriginalUri},
    response::IntoResponse,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use reqwest::{self, Client};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::*;

const PROXY_AUTH: bool = true;

// Load config from file / create new file
static CONFIG: Lazy<Config> = Lazy::new(Config::new);

// Client can be re-used for each request
// Has custom user agent
// timeout to stop requests running forever (Unsure if needed?!)
static CLIENT: Lazy<Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent(&CONFIG.host.user_agent)
        .timeout(Duration::from_secs(CONFIG.host.timeout))
        .build()
        .unwrap()
});

#[tokio::main]
async fn main() {
    // Create and start logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "xt_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create routes w/ states
    let app = Router::new().route("/*O", get(proxy));

    tracing::info!("listening on {}", CONFIG.host.address);

    // Start Server
    axum::Server::bind(&CONFIG.host.address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

// proxy handler
async fn proxy(
    OriginalUri(mut path): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    if PROXY_AUTH {
        // Replace HOST_USER/PASS, WITH XT_USER/PASS
        path = format!("{path}")
            .replace(&CONFIG.host.user, &CONFIG.xtream.user)
            .replace(&CONFIG.host.pass, &CONFIG.xtream.pass)
            .try_into()
            .unwrap();
    }

    tracing::info!("{addr} => {path}");

    // Form target from XT_ADDR and modified path
    let target: String = format!("{}{path}", CONFIG.xtream.address);

    // send the get request, and await the response
    let response = CLIENT.get(target).send().await.unwrap();

    // turn the response into a byte stream
    let stream = response.bytes_stream();

    // return the byte stream, within a stream body
    StreamBody::new(stream)
}
