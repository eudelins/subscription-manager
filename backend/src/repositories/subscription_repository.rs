use crate::SubscriptionsDb;
use crate::dao_entities::subscription_dao::SubscriptionDAO;

use rocket_db_pools::{Connection};
use rocket_db_pools::sqlx::{self};
use sqlx::FromRow;

pub struct SubscriptionRepository {
}

impl SubscriptionRepository {
    pub async fn find_all_subscription(&self, mut db: Connection<SubscriptionsDb>) -> Vec<SubscriptionDAO> {
        sqlx::query("SELECT * FROM Subscriptions")
            .fetch_all(&mut *db).await
            .unwrap_or_else(|_| vec![])
            .iter()
            .filter_map(|row| SubscriptionDAO::from_row(row).ok())
            .collect()
    }

    pub async fn find_subscription(&self, mut db: Connection<SubscriptionsDb>, id: i32) -> Option<SubscriptionDAO> {
        sqlx::query("SELECT * FROM Subscriptions WHERE id = $1").bind(id)
            .fetch_one(&mut *db).await
            .and_then(|r| SubscriptionDAO::from_row(&r))
            .map_err(|e| println!("Error: {:?}", e))
            .ok()
    }
}
