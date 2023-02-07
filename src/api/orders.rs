use axum::{extract::Path, http::StatusCode};
use tracing::debug;
use uuid::Uuid;

pub async fn create() -> StatusCode {
    debug!("Creating order");
    StatusCode::FORBIDDEN
}

pub async fn list() -> StatusCode {
    debug!("Listing orders");
    StatusCode::FORBIDDEN
}

pub async fn get(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Get order id: {id}");
    StatusCode::FORBIDDEN
}

pub async fn add_item(Path(id): Path<Uuid>) -> StatusCode {
    debug!("Add item to order id: {id}");
    StatusCode::FORBIDDEN
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Delete item {index} from order id: {id}");
    StatusCode::FORBIDDEN
}
