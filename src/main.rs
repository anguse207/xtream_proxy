#![allow(unused_imports)]
#![allow(dead_code)]

use std::{convert::Infallible, net::SocketAddr, time::Duration};

use axum::{
    body::{Body, Bytes},
    extract::{Path, State, ConnectInfo, Host, OriginalUri},
    response::{IntoResponse, Response},
    routing::{get, any},
    Json, Router, Server,
};
use tokio_stream::StreamExt;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "0.0.0.0";
const HOST_PORT: &str = "1081";

// Where we are redirecting the requests
const SEND_ADDR: &str = "127.0.0.1";
const SEND_PORT: &str = "3000";

#[tokio::main]
async fn main() {
    let address = format!("{}:{}", HOST_ADDR, HOST_PORT);

    println!("Listening on {address}");

    // Create routes w/ states
    let app = Router::new()
        .route("/*O", get(wildcard))
        .route("/direct", get(direct));
    // Start Server
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn wildcard(
    Path(path): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    OriginalUri(original_uri): OriginalUri,
    ) -> impl IntoResponse {
    println!("Received:\n{original_uri}\nFrom:\n{addr}\n");
    format!("echo: {path}\n")
}

async fn direct(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    OriginalUri(original_uri): OriginalUri,
    ) -> impl IntoResponse {
    println!("Received:\n{original_uri}\nFrom:\n{addr}\n");
    format!("Hello Proxee\n")
}

// async fn example(
//     Path(path): Path<String>,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     OriginalUri(original_uri): OriginalUri,
//     ) -> impl IntoResponse {
//     println!("{original_uri}"); -> "/api/x/y/z"
//     println!("{path}"); -> "api/x/y/z"

// }