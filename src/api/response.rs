use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Item {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<Item>,
}
