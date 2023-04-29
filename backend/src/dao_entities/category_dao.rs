use sqlx;

#[derive(sqlx::FromRow)]
pub struct CategoryDAO {
    pub id: i32,
    pub name: String,
}