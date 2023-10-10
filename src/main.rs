use std::{net::SocketAddr, sync::Arc, time::{Instant, Duration}};

use axum::{
    extract::{ConnectInfo, OriginalUri, State},
    response::IntoResponse,
    routing::{any, get, post},
    Router, body::StreamBody,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt::format, layer::SubscriberExt, util::SubscriberInitExt};

use reqwest::{self, Client};

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use bytes::BytesMut;

const USER_AGENT: &str = "tivimate";

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "0.0.0.0";
const HOST_PORT: &str = "1081";

// TARGET
const SEND_PROTOCOL: &str = "http";
const SEND_ADDR: &str = "thelads.ddns.net";
const SEND_PORT: &str = "80";
const USER: &str = "jN9AhFAHmf";
const PASS: &str = "r9amExT7Qm";

/*  todo - use tcpstreams,
1) Client <-> Host &
2) Host <-> Server

When Reading from 1, write to 2
when reading from 2, write to 1

*/
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "xc_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let bind_address = format!("{}:{}", HOST_ADDR, HOST_PORT);

    tracing::debug!("listening on {bind_address}");
    tracing::info!("info");
    // <Client> is an arc internally, no need to put it in one
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    // Create routes w/ states
    let app = Router::new()
        .route("/*O", get(proxy))
        //.route("/*O", get(tcp_proxy))
        .with_state(client);

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
    let target = format!("{SEND_PROTOCOL}://{SEND_ADDR}:{SEND_PORT}{original_uri}");
    tracing::info!("=> {original_uri}");
    let byte_stream = client
        .get(&target)
        .send()
        .await
        .unwrap()
        .bytes_stream();

    StreamBody::new(byte_stream)
}