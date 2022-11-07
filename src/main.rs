mod in_mem_order_store;
mod order_store;

use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let server_addr = ([127, 0, 0, 1], 8080).into();
    let app = Router::new().route("/", get(hello));

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
