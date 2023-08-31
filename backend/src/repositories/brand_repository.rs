use crate::dao_entities::brand_dao::BrandDAO;
use crate::SubscriptionsDb;

use rocket_db_pools::{sqlx, Connection};
use sqlx::FromRow;

pub async fn find_all_brands(mut db: Connection<SubscriptionsDb>) -> Vec<BrandDAO> {
    sqlx::query("SELECT * FROM Brands;")
        .fetch_all(&mut *db)
        .await
        .unwrap_or_else(|_| vec![])
        .iter()
        .filter_map(|row| BrandDAO::from_row(row).ok())
        .collect()
}

pub async fn find_brand_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<BrandDAO> {
    sqlx::query("SELECT * FROM Brands WHERE id = $1;")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| BrandDAO::from_row(&r))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn create_or_update_brand<'a>(
    mut db: Connection<SubscriptionsDb>,
    new_brand: BrandDAO,
    is_update: bool,
) -> Option<BrandDAO> {
    let query = if is_update {
        sqlx::query("UPDATE Brands SET name=$2 WHERE id=$1 RETURNING *;").bind(new_brand.id)
    } else {
        sqlx::query("INSERT INTO Brands (name) VALUES ($1) RETURNING *;")
    };

    query
        .bind(new_brand.name)
        .fetch_one(&mut *db)
        .await
        .and_then(|res| BrandDAO::from_row(&res))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
}

pub async fn delete_brand_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    sqlx::query("DELETE FROM Brands WHERE id = $1;")
        .bind(id)
        .execute(&mut *db)
        .await
        .map_err(|e| println!("Error while deleting brand: {:?}", e))
        .ok()
        .filter(|res| res.rows_affected() == 1)
        .map(|_| ())
}

// pub async fn update_brand_logo<'a>(
//     mut db: Connection<SubscriptionsDb>,
//     id: i32,
//     logo_path: Option<&str>,
// ) -> Option<()> {
//     let query = sqlx::query("UPDATE Brands SET logo=$2 WHERE id=$1 RETURNING *;");
//     query
//         .bind(id)
//         .bind(logo_path)
//         .fetch_one(&mut *db)
//         .await
//         .map(|_| ())
//         .map_err(|e| println!("Error: {:?}", e))
//         .ok()
// }
