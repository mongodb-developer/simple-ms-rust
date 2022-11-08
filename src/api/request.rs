use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddItem {
    pub product_id: Uuid,
    pub quantity: i32,
}
