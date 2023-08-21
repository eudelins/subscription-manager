use sqlx;

use crate::dto_entities::brand_dto::BrandDTO;

#[derive(sqlx::FromRow)]
pub struct BrandDAO {
    pub id: i32,
    pub name: String,
}

impl From<BrandDTO> for BrandDAO {
    fn from(brand_dto: BrandDTO) -> Self {
        BrandDAO {
            id: brand_dto.id,
            name: brand_dto.name,
        }
    }
}
