use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, spec::BinarySubtype, Binary},
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Cursor,
};
use uuid::Uuid;

use crate::order_store::{Order, OrderStore, OrderStoreError};

pub struct MongodbOrderStore {
    client: Client,
}

impl MongodbOrderStore {
    pub async fn new(client_uri: &str) -> Result<MongodbOrderStore, OrderStoreError> {
        if let Ok(options) =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await
        {
            if let Ok(client) = Client::with_options(options) {
                Ok(MongodbOrderStore { client })
            } else {
                Err(OrderStoreError::StoreUnavailable)
            }
        } else {
            Err(OrderStoreError::StoreUnavailable)
        }
    }
}

#[async_trait::async_trait]
impl OrderStore for MongodbOrderStore {
    async fn create_order(&self, user_id: Uuid) -> Result<Order, OrderStoreError> {
        let db = self.client.database("simple-ms");
        let orders: Collection<Order> = db.collection("orders");
        let order = Order {
            id: Uuid::new_v4(),
            user_id,
            items: vec![],
        };
        orders
            .insert_one(order.clone(), None)
            .await
            .map(|_| order)
            .map_err(|_| OrderStoreError::StoreUnavailable)
    }

    async fn get_order(&self, order_id: Uuid) -> Result<Order, OrderStoreError> {
        let db = self.client.database("simple-ms");
        let orders: Collection<Order> = db.collection("orders");
        let order: Result<Option<Order>, mongodb::error::Error> = orders
            .find_one(
                doc! {"_id": Binary {
                    subtype: BinarySubtype::Generic,
                    bytes: order_id.into_bytes().to_vec()
                }
                },
                None,
            )
            .await;
        match order {
            Err(_) => Err(OrderStoreError::StoreUnavailable),
            Ok(None) => Err(OrderStoreError::OrderNotFound(order_id)),
            Ok(Some(order)) => Ok(order),
        }
    }

    async fn list_orders(&self, user_id: Uuid) -> Result<Vec<Order>, OrderStoreError> {
        let db = self.client.database("simple-ms");
        let orders: Collection<Order> = db.collection("orders");
        let find_result: Result<Cursor<Order>, mongodb::error::Error> = orders
            .find(
                doc! {"user_id": Binary {
                    subtype: BinarySubtype::Generic,
                    bytes: user_id.into_bytes().to_vec()
                }
                },
                None,
            )
            .await;
        match find_result {
            Err(_) => Err(OrderStoreError::StoreUnavailable),
            Ok(cursor) => {
                let orders = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
                Ok(orders)
            }
        }
    }

    async fn add_item(
        &self,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), OrderStoreError> {
        let db = self.client.database("simple-ms");
        let orders: Collection<Order> = db.collection("orders");
        let update_result = orders
            .update_one(
                doc! {"_id": Binary {
                    subtype: BinarySubtype::Generic,
                    bytes: order_id.into_bytes().to_vec()
                }
                },
                doc! {"$push": {
                        "items": {
                "product_id": Binary {
                    subtype: BinarySubtype::Generic,
                    bytes: product_id.into_bytes().to_vec()
                },
                "quantity": quantity,
                        }
                    } },
                None,
            )
            .await;
        match update_result {
            Err(_) => Err(OrderStoreError::StoreUnavailable),
            Ok(result) => {
                if result.modified_count == 1 {
                    Ok(())
                } else if result.matched_count == 0 {
                    Err(OrderStoreError::OrderNotFound(order_id))
                } else {
                    Err(OrderStoreError::StoreUnavailable)
                }
            }
        }
    }

    async fn delete_item(&self, order_id: Uuid, index: usize) -> Result<(), OrderStoreError> {
        let db = self.client.database("simple-ms");
        let orders: Collection<Order> = db.collection("orders");
        let update_result = orders
            .update_one(
                doc! {"_id": Binary {
                    subtype: BinarySubtype::Generic,
                    bytes: order_id.into_bytes().to_vec()
                }
                },
                doc! {"$unset": {
                format!("items.{}", index): 1
                } },
                None,
            )
            .await;
        match update_result {
            Err(_) => Err(OrderStoreError::StoreUnavailable),
            Ok(result) => {
                if result.modified_count == 1 {
                    Ok(())
                } else if result.matched_count == 0 {
                    Err(OrderStoreError::OrderNotFound(order_id))
                } else {
                    Err(OrderStoreError::StoreUnavailable)
                }
            }
        }
    }
}