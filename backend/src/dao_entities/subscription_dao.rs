use sqlx;

#[derive(sqlx::FromRow)]
pub struct SubscriptionDAO {
    pub id: i32,
    pub brand_id: i32,
    pub name: String,
    pub price: f32,
    pub status: bool,
    pub logo: Option<String>,
    pub subscribe_link: Option<String>,
    pub unsubscribe_link: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}