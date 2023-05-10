use sqlx;

use crate::dto_entities::brand_dto::BrandDTO;

#[derive(sqlx::FromRow)]
pub struct BrandDAO {
    pub id: i32,
    pub name: String,
    pub logo: Option<String>,
}

impl BrandDAO {
    pub fn build_from_dto(brand_dto: BrandDTO) -> BrandDAO {
        BrandDAO {
            id: -1,
            name: brand_dto.name,
            logo: brand_dto.logo
        }
    }
}