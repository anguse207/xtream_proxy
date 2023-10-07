#![allow(unused_imports)]
#![allow(dead_code)]

use std::{convert::Infallible, net::SocketAddr, time::Duration};

use axum::{
    body::{Body, Bytes},
    extract::{Path, State, ConnectInfo},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router, Server,
};
use reqwest::{Client, StatusCode};
use tokio_stream::StreamExt;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "127.0.0.1";
const HOST_PORT: &str = "3000";

// Where we are redirecting the requests
const SEND_ADDR: &str = "127.0.0.1";
const SEND_PORT: &str = "3000";

#[tokio::main]
async fn main() {
    let address = format!("{}:{}", HOST_ADDR, HOST_PORT);

    println!("Listening on {address}");

    // Create routes w/ states
    let app = Router::new()
        .route("/:path/:path/:path", get(proxy));
    // Start Server
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn proxy(Path(path): Path<String>, ConnectInfo(addr): ConnectInfo<SocketAddr>,) -> impl IntoResponse {
    println!("Received:\n{path}\nFrom:\n{addr}\n");
    format!("{path}\n")
}
