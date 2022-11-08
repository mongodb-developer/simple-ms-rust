mod in_mem_order_store;
mod order_store;

use axum::{
    error_handling::HandleErrorLayer,
    extract::Path,
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post},
    BoxError, Router, Server,
};
use dotenv::dotenv;
use std::{env, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Init Logging
    tracing_subscriber::fmt::init();

    // Load environment configuration from .env
    dotenv().expect("Set your configuration in a .env file");
    let server_addr = env::var("SERVER").expect("Define SERVER=host:port in your .env");
    let server_addr = server_addr
        .parse()
        .expect("Define SERVER=host:port in your .env");
    let app = Router::new()
        .route("/health", get(health))
        .route("/orders", get(list_orders).post(create_order))
        .route("/orders/:id", get(get_order))
        .route("/orders/:id/items", post(add_item_to_order))
        .route("/orders/:id/items/:index", delete(delete_item_from_order))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .fallback(fallback_handler.into_service());

    info!("Launching server: http://{server_addr}/");
    Server::bind(&server_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn create_order() -> StatusCode {
    debug!("Creating order");
    StatusCode::FORBIDDEN
}

async fn list_orders() -> StatusCode {
    debug!("Listing orders");
    StatusCode::FORBIDDEN
}

async fn get_order(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Get order id: {id}");
    StatusCode::FORBIDDEN
}

async fn add_item_to_order(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Add item to order id: {id}");
    StatusCode::FORBIDDEN
}

async fn delete_item_from_order(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Delete item {index} from order id: {id}");
    StatusCode::FORBIDDEN
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
