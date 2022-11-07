mod in_mem_order_store;
mod order_store;

use axum::{
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment configuration from .env
    dotenv().expect("Set your configuration in a .env file");
    let server_addr = env::var("SERVER").expect("Define SERVER=host:port in your .env");
    let server_addr = server_addr
        .parse()
        .expect("Define SERVER=host:port in your .env");
    let app = Router::new()
        .route("/", get(hello))
        .fallback(fallback_handler.into_service());

    println!("Launching server: http://{server_addr}/");
    Server::bind(&server_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "SuperMicroService"
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
