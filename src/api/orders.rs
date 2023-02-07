use axum::{extract::Path, http::StatusCode, Json};
use tracing::debug;
use uuid::Uuid;

use super::{request::AddItem, response::Order};

pub async fn create() -> (StatusCode, Json<Option<Order>>) {
    debug!("Creating order");
    (StatusCode::FORBIDDEN, Json(None))
}

pub async fn list() -> (StatusCode, Json<Option<Vec<Order>>>) {
    debug!("Listing orders");
    (StatusCode::FORBIDDEN, Json(None))
}

pub async fn get(Path(id): Path<Uuid>) -> (StatusCode, Json<Option<Order>>) {
    debug!("Get order id: {id}");
    (StatusCode::FORBIDDEN, Json(None))
}

pub async fn add_item(Path(id): Path<Uuid>, Json(request): Json<AddItem>) -> StatusCode {
    debug!(
        "Add item to order id: {}: product_id={} quantity={}",
        id, request.product_id, request.quantity
    );
    StatusCode::FORBIDDEN
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Delete item {index} from order id: {id}");
    StatusCode::FORBIDDEN
}
