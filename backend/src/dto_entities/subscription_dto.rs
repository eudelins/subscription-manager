use serde::{Deserialize, Serialize};

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
            status: sub_dao.status,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EntireSubscriptionDTO {
    pub id: i32,
    pub brand_id: i32,
    pub name: String,
    pub price: f32,
    pub status: bool,
    pub categories_id: Vec<i32>,
}

impl From<SubscriptionDAO> for EntireSubscriptionDTO {
    fn from(sub_dao: SubscriptionDAO) -> Self {
        EntireSubscriptionDTO {
            id: sub_dao.id,
            brand_id: sub_dao.brand_id,
            name: sub_dao.name,
            price: sub_dao.price,
            status: sub_dao.status,
            categories_id: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrUpdateSubscriptionDTO {
    pub id: Option<i32>,
    pub name: String,
    pub brand_id: i32,
    pub price: f32,
    pub status: bool,
    pub categories_id: Vec<i32>,
}
