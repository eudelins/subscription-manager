use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryDTO {
    pub name: String,
}