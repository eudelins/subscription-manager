use crate::dao_entities::subscription_dao::SubscriptionDAO;
use crate::SubscriptionsDb;

use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use sqlx::PgExecutor;
use sqlx::Row;

pub async fn find_all_subscriptions(db: &mut Connection<SubscriptionsDb>) -> Vec<SubscriptionDAO> {
    sqlx::query("SELECT * FROM Subscriptions")
        .fetch_all(&mut **db)
        .await
        .unwrap_or_else(|_| vec![])
        .iter()
        .filter_map(|row| SubscriptionDAO::from_row(row).ok())
        .collect()
}

pub async fn find_subscription_by_id(
    db: &mut Connection<SubscriptionsDb>,
    id: i32,
) -> Option<SubscriptionDAO> {
    sqlx::query("SELECT * FROM Subscriptions WHERE id = $1")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| SubscriptionDAO::from_row(&r))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn create_or_update_subscription<'a>(
    db: impl PgExecutor<'a>,
    new_sub: SubscriptionDAO,
    is_update: bool,
) -> Option<SubscriptionDAO> {
    let query = if is_update {
        sqlx::query(
            "UPDATE Subscriptions SET brand_id=$2, name=$3, price=$4, status=$5 WHERE id=$1 RETURNING *;"
        ).bind(new_sub.id)
    } else {
        sqlx::query(
            "INSERT INTO Subscriptions (brand_id, name, price, status) VALUES ($1, $2, $3, $4) RETURNING *;"
        )
    };

    query
        .bind(new_sub.brand_id)
        .bind(new_sub.name)
        .bind(new_sub.price)
        .bind(new_sub.status)
        .fetch_one(db)
        .await
        .and_then(|res| SubscriptionDAO::from_row(&res))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn update_subscription_status<'a>(
    db: impl PgExecutor<'a>,
    sub_id: i32,
    new_status: bool,
) -> Option<SubscriptionDAO> {
    sqlx::query("UPDATE Subscriptions SET status=$2 WHERE id=$1 RETURNING *;")
        .bind(sub_id)
        .bind(new_status)
        .fetch_one(db)
        .await
        .and_then(|res| SubscriptionDAO::from_row(&res))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn delete_subscription_by_id(
    db: &mut Connection<SubscriptionsDb>,
    id: i32,
) -> Option<()> {
    sqlx::query("DELETE FROM Subscriptions WHERE id = $1;")
        .bind(id)
        .execute(&mut **db)
        .await
        .map_err(|e| println!("Error while deleting subscription: {:?}", e))
        .ok()
        .filter(|res| res.rows_affected() == 1)
        .map(|_| ())
}

pub async fn fetch_subscription_categories_id(
    db: &mut Connection<SubscriptionsDb>,
    id: i32,
) -> Vec<i32> {
    sqlx::query("SELECT category_id FROM Belongs_To_Categories WHERE subscription_id = $1;")
        .bind(id)
        .fetch_all(&mut **db)
        .await
        .unwrap_or_else(|_| vec![])
        .iter()
        .filter_map(|row| row.try_get(0).ok())
        .collect()
}

pub async fn remove_subscription_from_all_categories(
    db: impl PgExecutor<'_>,
    id: i32,
) -> Option<()> {
    sqlx::query("DELETE FROM Belongs_To_Categories WHERE subscription_id = $1;")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| println!("Error while removing sub from categories: {:?}", e))
        .ok()
        .map(|_| ())
}
