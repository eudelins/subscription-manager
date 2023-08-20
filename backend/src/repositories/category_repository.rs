use crate::dao_entities::category_dao::CategoryDAO;
use crate::SubscriptionsDb;

use rocket_db_pools::{sqlx, Connection};
use sqlx::FromRow;
use sqlx::PgExecutor;

pub async fn find_all_categories(mut db: Connection<SubscriptionsDb>) -> Vec<CategoryDAO> {
    sqlx::query("SELECT * FROM Categories;")
        .fetch_all(&mut *db)
        .await
        .unwrap_or_else(|_| vec![])
        .iter()
        .filter_map(|row| CategoryDAO::from_row(row).ok())
        .collect()
}

pub async fn find_category_by_id(
    mut db: Connection<SubscriptionsDb>,
    id: i32,
) -> Option<CategoryDAO> {
    sqlx::query("SELECT * FROM Categories WHERE id = $1;")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| CategoryDAO::from_row(&r))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn create_or_update_category<'a>(
    mut db: Connection<SubscriptionsDb>,
    new_category: CategoryDAO,
    is_update: bool,
) -> Option<CategoryDAO> {
    let query = if is_update {
        sqlx::query("UPDATE Categories SET name=$2, icon=$3 WHERE id=$1 RETURNING *;")
            .bind(new_category.id)
    } else {
        sqlx::query("INSERT INTO Categories (name, icon) VALUES ($1, $2) RETURNING *;")
    };

    query
        .bind(new_category.name)
        .bind(new_category.icon)
        .fetch_one(&mut *db)
        .await
        .and_then(|res| CategoryDAO::from_row(&res))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn delete_category_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    sqlx::query("DELETE FROM Categories WHERE id = $1;")
        .bind(id)
        .execute(&mut *db)
        .await
        .map_err(|e| println!("Error while deleting category: {:?}", e))
        .ok()
        .filter(|res| res.rows_affected() == 1)
        .map(|_| ())
}

pub async fn add_subscription_to_category(
    db_conn: impl PgExecutor<'_>,
    sub_id: i32,
    category_id: i32,
) -> Option<()> {
    sqlx::query("INSERT INTO Belongs_To_Categories (subscription_id, category_id) VALUES ($1, $2);")
        .bind(sub_id)
        .bind(category_id)
        .execute(db_conn)
        .await
        .map_err(|e| println!("Error while adding sub to category: {:?}", e))
        .ok()
        .filter(|res| res.rows_affected() == 1)
        .map(|_| ())
}

pub async fn update_category_icon<'a>(
    mut db: Connection<SubscriptionsDb>,
    id: i32,
    icon_path: Option<&str>,
) -> Option<()> {
    let query = sqlx::query("UPDATE Categories SET icon=$2 WHERE id=$1 RETURNING *;");
    query
        .bind(id)
        .bind(icon_path)
        .fetch_one(&mut *db)
        .await
        .map(|_| ())
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}
