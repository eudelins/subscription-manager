use sqlx;

use crate::dto_entities::subscription_dto::SubscriptionDTO;

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

impl SubscriptionDAO {
    pub fn build_from_dto(sub_dto: SubscriptionDTO) -> SubscriptionDAO {
        SubscriptionDAO {
            id: sub_dto.id,
            brand_id: 1,  // TO DO
            name: sub_dto.name,
            price: sub_dto.price,
            status: sub_dto.status,
            logo: Option::None,
            subscribe_link: Option::None,
            unsubscribe_link: Option::None,
            start_date: Option::None,
            end_date: Option::None
        }
    }
}