use serde::{Deserialize, Serialize};

use crate::dao_entities::category_dao::CategoryDAO;

#[derive(Serialize, Deserialize)]
pub struct CategoryDTO {
    pub id: i32,
    pub name: String,
}

impl From<CategoryDAO> for CategoryDTO {
    fn from(category_dao: CategoryDAO) -> Self {
        CategoryDTO {
            id: category_dao.id,
            name: category_dao.name,
        }
    }
}
