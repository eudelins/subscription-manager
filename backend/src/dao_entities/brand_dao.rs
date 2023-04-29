use sqlx;

#[derive(sqlx::FromRow)]
pub struct BrandDAO {
    pub id: i32,
    pub name: String,
    pub logo: Option<String>,
}