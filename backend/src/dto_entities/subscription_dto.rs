use serde::{Serialize, Deserialize};

use crate::dao_entities::subscription_dao::SubscriptionDAO;

#[derive(Serialize, Deserialize)]
pub struct SubscriptionDTO {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub status: bool,
}


impl SubscriptionDTO {
    pub fn build_from_dao(sub_dao: SubscriptionDAO) -> SubscriptionDTO {
        SubscriptionDTO {
            id: sub_dao.id,
            name: sub_dao.name,
            price: sub_dao.price,
            status: sub_dao.status
        }
    }
}