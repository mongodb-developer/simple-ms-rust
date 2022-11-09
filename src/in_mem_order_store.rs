use std::sync::RwLock;
use uuid::Uuid;

use crate::order_store::{Item, Order, OrderStore, OrderStoreError};

pub struct InMemOrderStore {
    orders: RwLock<Vec<Order>>,
}

impl InMemOrderStore {
    /// Creates a new in-memory order store.
    ///
    /// # Examples
    ///
    /// ```
    /// let in_mem_store = InMemOrderStore::new();
    /// ```
    pub fn new() -> InMemOrderStore {
        InMemOrderStore {
            orders: RwLock::new(vec![]),
        }
    }
}

#[async_trait::async_trait]
impl OrderStore for InMemOrderStore {
    async fn create_order(&self, user_id: Uuid) -> Result<Order, OrderStoreError> {
        let order = Order::new(user_id);
        let mut data = self.orders.write().unwrap();
        data.push(order.clone());
        Ok(order)
    }

    async fn get_order(&self, order_id: Uuid) -> Result<Order, OrderStoreError> {
        let data = self.orders.read().unwrap();
        data.iter()
            .find(|&order| order.id == order_id)
            .cloned()
            .ok_or(OrderStoreError::OrderNotFound(order_id))
    }

    async fn list_orders(&self, user_id: Uuid) -> Result<Vec<Order>, OrderStoreError> {
        let data = self.orders.read().unwrap();
        let orders = data
            .iter()
            .filter(|x| x.user_id == user_id)
            .cloned()
            .collect::<Vec<Order>>();
        Ok(orders)
    }

    async fn add_item(
        &self,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), OrderStoreError> {
        let mut data = self.orders.write().unwrap();
        for order in data.iter_mut() {
            if order.id == order_id {
                order.items.push(Item {
                    product_id,
                    quantity,
                });
                return Ok(());
            }
        }
        Err(OrderStoreError::OrderNotFound(order_id))
    }

    async fn delete_item(&self, order_id: Uuid, index: usize) -> Result<(), OrderStoreError> {
        let mut data = self.orders.write().unwrap();
        for order in data.iter_mut() {
            if order.id == order_id {
                if index < order.items.len() {
                    order.items.remove(index);
                    return Ok(());
                } else {
                    return Err(OrderStoreError::ItemIndexOutOfBounds(index));
                }
            }
        }
        Err(OrderStoreError::OrderNotFound(order_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_context::{test_context, AsyncTestContext};

    #[test_context(Context)]
    #[tokio::test]
    async fn create_order_adds_order_to_store(ctx: &mut Context) {
        assert_eq!(
            ctx.in_mem_store
                .list_orders(ctx.user_id_1)
                .await
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            ctx.in_mem_store
                .list_orders(ctx.user_id_2)
                .await
                .unwrap()
                .len(),
            1
        );
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn get_order_retrieves_existing_order(ctx: &mut Context) {
        if let Ok(stored_order) = ctx.in_mem_store.get_order(ctx.order_1_user_1.id).await {
            assert_eq!(stored_order, ctx.order_1_user_1);
        } else {
            panic!("Order not found after being created");
        }
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn get_order_returns_error_for_non_existing_order(ctx: &mut Context) {
        let order_id = Uuid::new_v4();
        if let Err(OrderStoreError::OrderNotFound(not_found_id)) =
            ctx.in_mem_store.get_order(order_id).await
        {
            assert_eq!(order_id, not_found_id);
        } else {
            panic!("Unexpected order found");
        }
    }

    #[tokio::test]
    async fn item_cannot_be_added_to_non_existing_order() {
        let in_mem_store = InMemOrderStore::new();
        assert!(in_mem_store
            .add_item(Uuid::new_v4(), Uuid::new_v4(), 1)
            .await
            .is_err());
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn order_contains_added_item(ctx: &mut Context) {
        let product_id = Uuid::new_v4();
        let quantity = 42;
        if let Ok(()) = ctx
            .in_mem_store
            .add_item(ctx.order_1_user_1.id, product_id, quantity)
            .await
        {
            if let Ok(stored_order) = ctx.in_mem_store.get_order(ctx.order_1_user_1.id).await {
                assert_eq!(stored_order.items.len(), 1);
                assert_eq!(stored_order.items[0].product_id, product_id);
                assert_eq!(stored_order.items[0].quantity, quantity);
            } else {
                panic!("Order not found after being created");
            }
        } else {
            panic!("Failed to add item to order");
        }
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn order_contains_added_items(ctx: &mut Context) {
        let product_id_0 = Uuid::new_v4();
        let quantity_0 = 42;
        let product_id_1 = Uuid::new_v4();
        let quantity_1 = 7;
        if let (Ok(()), Ok(())) = (
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_0, quantity_0)
                .await,
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_1, quantity_1)
                .await,
        ) {
            if let Ok(stored_order) = ctx.in_mem_store.get_order(ctx.order_1_user_1.id).await {
                assert_eq!(stored_order.items.len(), 2);
                assert_eq!(stored_order.items[0].product_id, product_id_0);
                assert_eq!(stored_order.items[0].quantity, quantity_0);
                assert_eq!(stored_order.items[1].product_id, product_id_1);
                assert_eq!(stored_order.items[1].quantity, quantity_1);
            } else {
                panic!("Order not found after being created");
            }
        } else {
            panic!("Failed to add items to order");
        }
    }

    #[tokio::test]
    async fn item_cannot_be_deleted_from_non_existing_order() {
        let in_mem_store = InMemOrderStore::new();
        assert!(in_mem_store.delete_item(Uuid::new_v4(), 1).await.is_err());
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn attempt_to_delete_non_existent_item_from_order_returns_error(ctx: &mut Context) {
        let product_id_0 = Uuid::new_v4();
        let quantity_0 = 42;
        let product_id_1 = Uuid::new_v4();
        let quantity_1 = 7;
        if let (Ok(()), Ok(())) = (
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_0, quantity_0)
                .await,
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_1, quantity_1)
                .await,
        ) {
            if let Err(OrderStoreError::ItemIndexOutOfBounds(index)) =
                ctx.in_mem_store.delete_item(ctx.order_1_user_1.id, 2).await
            {
                assert_eq!(index, 2);
            } else {
                panic!("Deleting non-existent item must produce error");
            }
        } else {
            panic!("Failed to add items to order");
        }
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn last_item_can_be_deleted_from_order(ctx: &mut Context) {
        let product_id_0 = Uuid::new_v4();
        let quantity_0 = 42;
        let product_id_1 = Uuid::new_v4();
        let quantity_1 = 7;
        if let (Ok(()), Ok(())) = (
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_0, quantity_0)
                .await,
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_1, quantity_1)
                .await,
        ) {
            if let Ok(()) = ctx.in_mem_store.delete_item(ctx.order_1_user_1.id, 1).await {
                if let Ok(stored_order) = ctx.in_mem_store.get_order(ctx.order_1_user_1.id).await {
                    assert_eq!(stored_order.items.len(), 1);
                    assert_eq!(stored_order.items[0].product_id, product_id_0);
                    assert_eq!(stored_order.items[0].quantity, quantity_0);
                } else {
                    panic!("Order not found after being created");
                }
            } else {
                panic!("Failed to delete item from order");
            }
        } else {
            panic!("Failed to add items to order");
        }
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn first_item_can_be_deleted_from_order(ctx: &mut Context) {
        let product_id_0 = Uuid::new_v4();
        let quantity_0 = 42;
        let product_id_1 = Uuid::new_v4();
        let quantity_1 = 7;
        if let (Ok(()), Ok(())) = (
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_0, quantity_0)
                .await,
            ctx.in_mem_store
                .add_item(ctx.order_1_user_1.id, product_id_1, quantity_1)
                .await,
        ) {
            if let Ok(()) = ctx.in_mem_store.delete_item(ctx.order_1_user_1.id, 0).await {
                if let Ok(stored_order) = ctx.in_mem_store.get_order(ctx.order_1_user_1.id).await {
                    assert_eq!(stored_order.items.len(), 1);
                    assert_eq!(stored_order.items[0].product_id, product_id_1);
                    assert_eq!(stored_order.items[0].quantity, quantity_1);
                } else {
                    panic!("Order not found after being created");
                }
            } else {
                panic!("Failed to delete item from order");
            }
        } else {
            panic!("Failed to add items to order");
        }
    }

    struct Context {
        user_id_1: Uuid,
        user_id_2: Uuid,
        in_mem_store: InMemOrderStore,
        order_1_user_1: Order,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for Context {
        async fn setup() -> Context {
            let user_id_1 = Uuid::new_v4();
            let in_mem_store = InMemOrderStore::new();
            let order = in_mem_store.create_order(user_id_1).await;
            let ctx = Context {
                user_id_1,
                user_id_2: Uuid::new_v4(),
                in_mem_store,
                order_1_user_1: order.unwrap(),
            };
            _ = ctx.in_mem_store.create_order(ctx.user_id_2).await;
            _ = ctx.in_mem_store.create_order(ctx.user_id_1).await;

            ctx
        }
        async fn teardown(self) {}
    }
}
