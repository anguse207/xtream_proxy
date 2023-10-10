use std::{net::SocketAddr, sync::Arc, time::Instant};

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
    let address = format!("{}:{}", HOST_ADDR, HOST_PORT);

    println!("Listening on {address}");

    // <Client> is an arc internally, no need to put it in one
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();

    // Create routes w/ states
    let app = Router::new()
        .route("/*O", get(get_wildcard))
        //.route("/*O", get(tcp_proxy))
        .with_state(client);

    // Start Server
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn get_wildcard(
    OriginalUri(original_uri): OriginalUri,
    State(client): State<Client>,
) -> impl IntoResponse {
    let target = format!("{SEND_PROTOCOL}://{SEND_ADDR}:{SEND_PORT}{original_uri}");

    let byte_stream = client
        .get(&target)
        .send()
        .await
        .unwrap()
        .bytes_stream();

    StreamBody::new(byte_stream)
}