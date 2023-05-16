use crate::SubscriptionsDb;
use crate::dao_entities::subscription_dao::SubscriptionDAO;

use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;
use sqlx::FromRow;
use sqlx::Row;


pub async fn find_all_subscriptions(db: &mut Connection<SubscriptionsDb>) -> Vec<SubscriptionDAO> {
    sqlx::query("SELECT * FROM Subscriptions")
        .fetch_all(&mut **db).await
        .unwrap_or_else(|_| vec![])
        .iter()
        .filter_map(|row| SubscriptionDAO::from_row(row).ok())
        .collect()
}

pub async fn find_subscription_by_id(db: &mut Connection<SubscriptionsDb>, id: i32) -> Option<SubscriptionDAO> {
    sqlx::query("SELECT * FROM Subscriptions WHERE id = $1").bind(id)
        .fetch_one(&mut **db).await
        .and_then(|r| SubscriptionDAO::from_row(&r))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn create_subscription(db: &mut Connection<SubscriptionsDb>, new_sub: SubscriptionDAO) -> Option<SubscriptionDAO> {
    sqlx::query(
        "INSERT INTO Subscriptions (brand_id, name, price, status) VALUES ($1, $2, $3, $4) RETURNING *;"
    )
    .bind(new_sub.brand_id)
    .bind(new_sub.name)
    .bind(new_sub.price)
    .bind(new_sub.status)
    .fetch_one(&mut **db).await
    .and_then(|res| SubscriptionDAO::from_row(&res))
    .map_err(|e| println!("Error: {:?}", e))
    .ok()
}

pub async fn delete_subscription_by_id(db: &mut Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    sqlx::query(
        "DELETE FROM Subscriptions WHERE id = $1;"
    )
    .bind(id)
    .execute(&mut **db).await.map(|res| {
        println!("Successful delete: {} row affected", res.rows_affected())
    })
    .map_err(|e| println!("Error: {:?}", e))
    .ok()
}

pub async fn fetch_subscription_categories_id(db: &mut Connection<SubscriptionsDb>, id: i32) -> Vec<i32> {
    sqlx::query(
        "SELECT category_id FROM Belongs_To_Categories WHERE subscription_id = $1;"
    )
    .bind(id)
    .fetch_all(&mut **db).await
    .unwrap_or_else(|_| vec![])
    .iter()
    .filter_map(|row| row.try_get(0).ok())
    .collect()
}