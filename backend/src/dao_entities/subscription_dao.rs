use sqlx;

use crate::dto_entities::subscription_dto::{SubscriptionDTO, CreateSubscriptionDTO};

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

impl From<CreateSubscriptionDTO> for SubscriptionDAO {
    fn from(create_sub_dto: CreateSubscriptionDTO) -> Self {
        SubscriptionDAO {
            id: -1,
            brand_id: create_sub_dto.brand_id,
            name: create_sub_dto.name,
            price: create_sub_dto.price,
            status: create_sub_dto.status,
            logo: Option::None,
            subscribe_link: Option::None,
            unsubscribe_link: Option::None,
            start_date: Option::None,
            end_date: Option::None
        }
    }
}