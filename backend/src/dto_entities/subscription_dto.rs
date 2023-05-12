use serde::{Serialize, Deserialize};

use crate::dao_entities::subscription_dao::SubscriptionDAO;

#[derive(Serialize, Deserialize)]
pub struct SubscriptionDTO {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub status: bool,
}

impl From<SubscriptionDAO> for SubscriptionDTO {
    fn from(sub_dao: SubscriptionDAO) -> Self {
        SubscriptionDTO {
            id: sub_dao.id,
            name: sub_dao.name,
            price: sub_dao.price,
            status: sub_dao.status
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateSubscriptionDTO {
    pub name: String,
    pub brand_id: i32,
    pub price: f32,
    pub status: bool,
    pub categories_id: Vec<i32>,
}