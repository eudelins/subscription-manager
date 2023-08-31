use crate::dto_entities::brand_dto::BrandDTO;
use crate::SubscriptionsDb;
use crate::{dao_entities::brand_dao::BrandDAO, repositories::brand_repository};

use rocket_db_pools::Connection;

pub async fn find_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<BrandDTO> {
    brand_repository::find_brand_by_id(db, id)
        .await
        .map(BrandDTO::from)
}

pub async fn find_all_brands(db: Connection<SubscriptionsDb>) -> Vec<BrandDTO> {
    brand_repository::find_all_brands(db)
        .await
        .into_iter()
        .map(BrandDTO::from)
        .collect()
}

pub async fn create_or_update_brand(
    db: Connection<SubscriptionsDb>,
    new_brand_dto: BrandDTO,
    is_update: bool,
) -> Option<BrandDTO> {
    let new_brand = BrandDAO::from(new_brand_dto);
    brand_repository::create_or_update_brand(db, new_brand, is_update)
        .await
        .map(BrandDTO::from)
}

pub async fn delete_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    brand_repository::delete_brand_by_id(db, id).await
}

// pub async fn update_brand_logo(
//     db: Connection<SubscriptionsDb>,
//     id: i32,
//     logo_path: Option<&str>,
// ) -> Option<()> {
//     brand_repository::update_brand_logo(db, id, logo_path).await
// }
