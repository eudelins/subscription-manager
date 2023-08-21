use sqlx;

use crate::dto_entities::subscription_dto::{CreateOrUpdateSubscriptionDTO, SubscriptionDTO};

#[derive(sqlx::FromRow)]
pub struct SubscriptionDAO {
    pub id: i32,
    pub brand_id: i32,
    pub name: String,
    pub price: f32,
    pub status: bool,
}

impl From<SubscriptionDTO> for SubscriptionDAO {
    fn from(sub_dto: SubscriptionDTO) -> Self {
        SubscriptionDAO {
            id: sub_dto.id,
            brand_id: 1, // TO DO
            name: sub_dto.name,
            price: sub_dto.price,
            status: sub_dto.status,
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
        }
    }
}
