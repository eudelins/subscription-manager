use sqlx;

use crate::dto_entities::subscription_dto::{CreateOrUpdateSubscriptionDTO, SubscriptionDTO};

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

impl From<SubscriptionDTO> for SubscriptionDAO {
    fn from(sub_dto: SubscriptionDTO) -> Self {
        SubscriptionDAO {
            id: sub_dto.id,
            brand_id: 1, // TO DO
            name: sub_dto.name,
            price: sub_dto.price,
            status: sub_dto.status,
            logo: Option::None,
            subscribe_link: Option::None,
            unsubscribe_link: Option::None,
            start_date: Option::None,
            end_date: Option::None,
        }
    }
}

impl From<CreateOrUpdateSubscriptionDTO> for SubscriptionDAO {
    fn from(new_sub_dto: CreateOrUpdateSubscriptionDTO) -> Self {
        SubscriptionDAO {
            id: new_sub_dto.id.unwrap_or(-1),
            brand_id: new_sub_dto.brand_id,
            name: new_sub_dto.name,
            price: new_sub_dto.price,
            status: new_sub_dto.status,
            logo: Option::None,
            subscribe_link: Option::None,
            unsubscribe_link: Option::None,
            start_date: Option::None,
            end_date: Option::None,
        }
    }
}
