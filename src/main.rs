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

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "0.0.0.0:1081";
const PROXY_AUTH: bool = true;
const HOST_USER: &str = "citrus";
const HOST_PASS: &str = "fire";
const USER_AGENT: &str = "tivimate";

// TARGET
const XT_ADDR: &str = "http://thelads.ddns.net:80";
const XT_USER: &str = "jN9AhFAHmf";
const XT_PASS: &str = "r9amExT7Qm";

// Client can be re-used for each request
// Has custom user agent
// timeout to stop requests running forever (Unsure if needed?!)
static CLIENT: Lazy<Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(360))
        .build()
        .unwrap()
});

// Create config from file / create new file
static _CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

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

    // Create config from file / create new file
    let _config = Config::new();

    // Create routes w/ states
    let app = Router::new().route("/*O", get(proxy));

    tracing::info!("Will re-write:\nUSER: {HOST_USER} => {XT_USER}\nPASS: {HOST_PASS} => {XT_PASS}\nlistening on {HOST_ADDR}");

    // Start Server
    axum::Server::bind(&HOST_ADDR.parse().unwrap())
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
        path = format!("{path}")
            .replace(HOST_USER, XT_USER)
            .replace(HOST_PASS, XT_PASS)
            .try_into()
            .unwrap();
    }
    // Replace HOST_USER/PASS, WITH XT_USER/PASS

    tracing::info!("{addr} => {path}");

    // Form target from XT_ADDR and modified path
    let target: String = format!("{XT_ADDR}{path}");

    // send the get request, and await the response
    let response = CLIENT.get(target).send().await.unwrap();

    // turn the response into a byte stream
    let stream = response.bytes_stream();

    // return the byte stream, within a stream body
    StreamBody::new(stream)
}
