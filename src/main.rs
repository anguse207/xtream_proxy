use std::{,
    net::SocketAddr,
    sync::Arc,
};

use axum::{
    extract::{State, OriginalUri},
    response::IntoResponse,
    routing::{get, any, post, },
    Router,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, fmt::format};


use reqwest::{self, Client};

//
const USER_AGENT: &str = "tivimate";

// HOST / SERVER ADDRESS
const HOST_ADDR: &str = "192.168.68.100";
const HOST_PORT: &str = "1081";

// TARGET
const SEND_PROTOCOL: &str = "http";
const SEND_ADDR: &str = "thelads.ddns.net";
const SEND_PORT: &str = "80";
const USER: &str = "jN9AhFAHmf";
const PASS: &str = "r9amExT7Qm";


#[tokio::main]
async fn main() {
    let address = format!("{}:{}", HOST_ADDR, HOST_PORT);
    
    println!("Listening on {address}");

    // put client in arc or once cell
    let client = Arc::new(reqwest::Client::builder()
    .user_agent(USER_AGENT)
    .build().unwrap());


    // Create routes w/ states
    let app = Router::new()
        .route("/*O", get(get_wildcard))
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
 
    //println!("\nReceived:\n{HOST_ADDR}{original_uri}\nFrom:\n{client_addr}");
    println!("get => {original_uri}");

    let target = format!("{SEND_PROTOCOL}://{SEND_ADDR}:{SEND_PORT}{original_uri}");

    let response = client
        .get(target)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
    //println!("Got a response, sending...",);
    response
}

// extract ip & port from host
// async fn stream(
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     OriginalUri(original_uri): OriginalUri,
//     Path(id): Path<String>,
//     ) -> impl IntoResponse {
    
//     let client = reqwest::Client::builder()
//     .user_agent(USER_AGENT)
//     .build().unwrap();
//     //println!("\nReceived:\n{HOST_ADDR}{original_uri}\nFrom:\n{addr}");

//     let target = format!("{SEND_PROTOCOL}://{SEND_ADDR}:{SEND_PORT}/{USER}/{PASS}/{id}");

//     //println!("Sending request:\n{target}");
    
//     // Maybe tokio spawn, and send the stream to client?
//     let mut byte_stream = client.get(target).send().await.unwrap().bytes_stream();
    
//     // while let Some(item) = byte_stream.next().await {
//     //     let bts = item.unwrap();
//     //     //println!("Some bytes {:?}", bts);
//     //     return bts;
//     // }

//     tokio::spawn(async move{
//         let mut stream = UdpStream::connect(addr).await.unwrap();
//         //println!("Connected to {addr}");
        
//         while let Some(item) = byte_stream.next().await {
//             let mut x = item.unwrap();
//             //println!("Writing to stream: {}", &x.len());
//             stream.write(&mut x).await.unwrap();
//         }
//     });
// }

// async fn example(
//     Path(path): Path<String>,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     OriginalUri(original_uri): OriginalUri,
//     ) -> impl IntoResponse {
//     //println!("{original_uri}"); -> "/api/x/y/z"
//     //println!("{path}"); -> "api/x/y/z"
// }

/* EXAMPLE REQUESTS
http://thelads.ddns.net:1081/player_api.php?username=USER&password=PASS ->
"
{"user_info":
    {"username":"jN9AhFAHmf",
    "password":"r9amExT7Qm",
    "message":"Welcome to XUI.one",
    "auth":1,
    "status":"Active",
    "exp_date":"1704732639",
    "is_trial":"0",
    "active_cons":1,
    "created_at":"1696605429",
    "max_connections":"3",
    "allowed_output_formats":["m3u8","ts","rtmp"]
},
"server_info":
    {"xui":true,
    "version":"1.5.12",
    "revision":2,
    "url":"thelads.ddns.net",
    "port":"80",
    "https_port":"443",
    "server_protocol":"http",
    "rtmp_port":"8880",
    "timestamp_now":1696794145,
    "time_now":"2023-10-08 20:42:25",
    "timezone":"Europe/London"}
}
"
Stream Request ?!
Received:
192.168.68.100/jN9AhFAHmf/r9amExT7Qm/295699

*/