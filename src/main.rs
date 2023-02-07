mod api;
mod in_mem_order_store;
mod mongodb_order_store;
mod order_store;

use api::{health, orders};
use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post},
    Router, Server,
};
use dotenv::dotenv;
use std::{env, sync::Arc, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use crate::in_mem_order_store::InMemOrderStore;

#[tokio::main]
async fn main() {
    // Load environment configuration from .env
    dotenv().expect("Set your configuration in a .env file");
    // Init Tracing
    tracing_subscriber::fmt::init();

    let server_addr = env::var("SERVER").expect("Define SERVER=host:port in your .env");
    let server_addr = server_addr
        .parse()
        .expect("Define SERVER=host:port in your .env");
    let repo = InMemOrderStore::new();
    let state = Arc::new(repo);
    let orders_routes = Router::new()
        .route("/", get(orders::list).post(orders::create))
        .route("/:id", get(orders::get))
        .route("/:id/items", post(orders::add_item))
        .route("/:id/items/:index", delete(orders::delete_item))
        .with_state(state);
    let app = Router::new()
        .route("/health", get(health::get))
        .nest("/orders", orders_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(|_| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .fallback(fallback_handler);

    info!("Launching server: http://{server_addr}/");
    Server::bind(&server_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
}

//#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
