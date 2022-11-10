use crate::order_store::OrderStoreError;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

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
