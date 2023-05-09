use crate::SubscriptionsDb;
use crate::dao_entities::brand_dao::BrandDAO;

use rocket_db_pools::{Connection};
use rocket_db_pools::sqlx::{self};
use sqlx::FromRow;

pub struct BrandRepository {
}

impl BrandRepository {
    pub async fn find_all_brands(&self, mut db: Connection<SubscriptionsDb>) -> Vec<BrandDAO> {
        sqlx::query("SELECT * FROM Brands;")
            .fetch_all(&mut *db).await
            .unwrap_or_else(|_| vec![])
            .iter()
            .filter_map(|row| BrandDAO::from_row(row).ok())
            .collect()
    }

    pub async fn find_brand_by_id(&self, mut db: Connection<SubscriptionsDb>, id: i32) -> Option<BrandDAO> {
        sqlx::query("SELECT * FROM Brands WHERE id = $1;").bind(id)
            .fetch_one(&mut *db).await
            .and_then(|r| BrandDAO::from_row(&r))
            .map_err(|e| println!("Error: {:?}", e))
            .ok()
    }

    pub async fn create_brand(&self, mut db: Connection<SubscriptionsDb>, new_brand: BrandDAO) -> Option<BrandDAO> {
        sqlx::query(
            "INSERT INTO Brands (name, logo) VALUES ($1, $2) RETURNING *;"
        )
        .bind(new_brand.name)
        .bind(new_brand.logo)
        .fetch_one(&mut *db).await
        .and_then(|res| BrandDAO::from_row(&res))
        .map_err(|e| println!("Error: {:?}", e))
        .ok()
    }

    pub async fn delete_brand_by_id(&self, mut db: Connection<SubscriptionsDb>, id: i32) {
        sqlx::query(
            "DELETE FROM Brands WHERE id = $1;"
        )
        .bind(id)
        .execute(&mut *db).await.map(|res| {
            println!("Successful delete: {} row affected", res.rows_affected())
        })
        .map_err(|e| println!("Error: {:?}", e))
        .ok();
    }
}
