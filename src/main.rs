use std::net::TcpListener;

use axum_intro::server::{app, run_app};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    println!("--> Listening on {}:{}", addr.ip(), addr.port());
    run_app(app(), listener).await;
}
