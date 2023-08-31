use sqlx;

use crate::dto_entities::category_dto::CategoryDTO;

#[derive(sqlx::FromRow)]
pub struct CategoryDAO {
    pub id: i32,
    pub name: String,
}

impl From<CategoryDTO> for CategoryDAO {
    fn from(category_dto: CategoryDTO) -> Self {
        CategoryDAO {
            id: category_dto.id,
            name: category_dto.name,
        }
    }
}
