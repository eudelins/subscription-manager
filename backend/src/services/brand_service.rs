use crate::{repositories::brand_repository, dao_entities::brand_dao::BrandDAO};
use crate::dto_entities::brand_dto::BrandDTO;
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;


pub async fn find_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<BrandDTO> {
    brand_repository::find_brand_by_id(db, id).await.map(BrandDTO::from)
}

pub async fn find_all_brands(db: Connection<SubscriptionsDb>) -> Vec<BrandDTO> {
    brand_repository::find_all_brands(db).await
        .into_iter()
        .map(BrandDTO::from)
        .collect()
}

pub async fn create_brand(db: Connection<SubscriptionsDb>, new_brand_dto: BrandDTO) -> Option<BrandDTO> {
    let new_brand = BrandDAO::from(new_brand_dto);
    brand_repository::create_brand(db, new_brand).await.map(BrandDTO::from)
}

pub async fn delete_brand_by_id(db: Connection<SubscriptionsDb>, id: i32) {
    brand_repository::delete_brand_by_id(db, id).await
}