use std::{net::SocketAddr, time::Duration};

use axum::{
    body::StreamBody,
    extract::{OriginalUri, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use reqwest::{self, Client};

const USER_AGENT: &str = "tivimate";

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "0.0.0.0";
const HOST_PORT: &str = "1081";

// TARGET
const XC_ADDR: &str = "http://thelads.ddns.net:80";
const _XC_USER: &str = "jN9AhFAHmf";
const _XC_PASS: &str = "r9amExT7Qm";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "xc_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    

    // Client is an arc internally, no need to put it in one
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    // Create routes w/ states
    let app = Router::new().route("/*O", get(proxy)).with_state(client);
    
    let bind_address = format!("{}:{}", HOST_ADDR, HOST_PORT);
    tracing::info!("listening on {bind_address}");

    // Start Server
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn proxy(
    OriginalUri(original_uri): OriginalUri,
    State(client): State<Client>,
) -> impl IntoResponse {
    tracing::info!("=> {original_uri}");

    let target: String = format!("{XC_ADDR}{original_uri}");

    let response = client.get(target).send().await.unwrap();

    let stream = response.bytes_stream();

    StreamBody::new(stream)
}
