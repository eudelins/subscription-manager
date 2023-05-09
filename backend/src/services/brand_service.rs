use crate::{repositories::brand_repository::BrandRepository, dao_entities::brand_dao::BrandDAO};
use crate::dto_entities::brand_dto::BrandDTO;
use crate::SubscriptionsDb;

use rocket_db_pools::{Connection};

pub struct BrandService {
    brand_repository: BrandRepository
}

impl BrandService {
    pub async fn find_brand_by_id(&self, db: Connection<SubscriptionsDb>, id: i32) -> Option<BrandDTO> {
        self.brand_repository.find_brand_by_id(db, id).await.map(BrandDTO::build_from_dao)
    }

    pub async fn find_all_brands(&self, db: Connection<SubscriptionsDb>) -> Vec<BrandDTO> {
        self.brand_repository.find_all_brands(db).await
            .into_iter()
            .map(BrandDTO::build_from_dao)
            .collect()
    }

    pub async fn create_brand(&self, db: Connection<SubscriptionsDb>, new_sub_dto: BrandDTO) -> Option<BrandDTO> {
        let new_sub = BrandDAO::build_from_dto(new_sub_dto);
        self.brand_repository.create_brand(db, new_sub).await.map(BrandDTO::build_from_dao)
    }

    pub async fn delete_brand_by_id(&self, db: Connection<SubscriptionsDb>, id: i32) {
        self.brand_repository.delete_brand_by_id(db, id).await
    }

    pub fn build_brand_service() -> BrandService {
        BrandService {
            brand_repository: BrandRepository{}
        }
    }
}
