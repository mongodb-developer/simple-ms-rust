use mongodb::{
    bson::{doc, spec::BinarySubtype, Binary},
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
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

    async fn list_orders(&self, _user_id: Uuid) -> Result<Vec<Order>, OrderStoreError> {
        unimplemented!()
    }

    async fn add_item(
        &self,
        _order_id: Uuid,
        _product_id: Uuid,
        _quantity: i32,
    ) -> Result<(), OrderStoreError> {
        unimplemented!()
    }

    async fn delete_item(&self, _order_id: Uuid, _index: usize) -> Result<(), OrderStoreError> {
        unimplemented!()
    }
}
