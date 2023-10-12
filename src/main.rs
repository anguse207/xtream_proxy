use std::{net::SocketAddr, time::Duration};

use axum::{
    body::StreamBody,
    extract::{ConnectInfo, OriginalUri},
    response::IntoResponse,
    routing::get,
    Router,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use reqwest::{self, Client};

use once_cell::sync::Lazy;

const USER_AGENT: &str = "tivimate";

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "0.0.0.0";
const HOST_PORT: &str = "1081";

// TARGET
const XT_ADDR: &str = "http://thelads.ddns.net:80";
const XT_USER: &str = "jN9AhFAHmf";
const XT_PASS: &str = "r9amExT7Qm";

const USER: &str = "citrus";
const PASS: &str = "fire";

static CLIENT: Lazy<Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(360))
        .build()
        .unwrap()
});

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "xc_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create routes w/ states
    let app = Router::new().route("/*O", get(proxy));

    let bind_address = format!("{}:{}", HOST_ADDR, HOST_PORT);
    tracing::info!("listening on {bind_address}");

    // Start Server
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn proxy(
    OriginalUri(path): OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // If using USER/PASS, replace it with XT_USER/XT_PASS
    let path = format!("{path}")
        .replace(USER, XT_USER)
        .replace(PASS, XT_PASS);

    let target: String = format!("{XT_ADDR}{path}");

    tracing::info!("{addr} => {path}");

    let response = CLIENT.get(target).send().await.unwrap();

    let stream = response.bytes_stream();

    StreamBody::new(stream)
}
