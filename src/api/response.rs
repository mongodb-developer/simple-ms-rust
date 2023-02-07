use serde::Serialize;
use uuid::Uuid;

use crate::order_store;

#[derive(Serialize)]
pub struct Item {
    pub product_id: Uuid,
    pub quantity: i32,
}

impl From<order_store::Item> for Item {
    fn from(item: order_store::Item) -> Self {
        Item {
            product_id: item.product_id,
            quantity: item.quantity,
        }
    }
}

#[derive(Serialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<Item>,
}

impl From<order_store::Order> for Order {
    fn from(order: order_store::Order) -> Self {
        Order {
            id: order.id,
            user_id: order.user_id,
            items: order.items.iter().map(|i| Item::from(i.clone())).collect(),
        }
    }
}
