use serde::{Deserialize, Serialize};

use crate::dao_entities::brand_dao::BrandDAO;

#[derive(Serialize, Deserialize, Debug)]
pub struct BrandDTO {
    pub id: i32,
    pub name: String,
}

impl From<BrandDAO> for BrandDTO {
    fn from(brand_dao: BrandDAO) -> Self {
        BrandDTO {
            id: brand_dao.id,
            name: brand_dao.name,
        }
    }
}
