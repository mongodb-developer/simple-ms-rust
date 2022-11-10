use crate::order_store::{Order, OrderStore, OrderStoreError};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use uuid::Uuid;

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
        unimplemented!()
    }

    async fn list_orders(&self, user_id: Uuid) -> Result<Vec<Order>, OrderStoreError> {
        unimplemented!()
    }

    async fn add_item(
        &self,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), OrderStoreError> {
        unimplemented!()
    }

    async fn delete_item(&self, order_id: Uuid, index: usize) -> Result<(), OrderStoreError> {
        unimplemented!()
    }
}
