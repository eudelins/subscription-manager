use crate::repositories::subscription_repository::SubscriptionRepository;
use crate::dto_entities::subscription_dto::SimpleSubscriptionDTO;
use crate::SubscriptionsDb;

use rocket_db_pools::{Connection};

pub struct SubscriptionService {
    subscription_repository: SubscriptionRepository
}

impl SubscriptionService {
    pub async fn find_subscription(&self, db: Connection<SubscriptionsDb>, id: i32) -> Option<SimpleSubscriptionDTO> {
        match self.subscription_repository.find_subscription(db, id).await {
            Some(subscription) => Some(SimpleSubscriptionDTO {
                name: subscription.name,
                price: subscription.price,
                status: subscription.status,
            }),
            None => None
        }
    }

    pub async fn find_all_subscription(&self, db: Connection<SubscriptionsDb>) -> Vec<SimpleSubscriptionDTO> {
        self.subscription_repository.find_all_subscription(db).await
            .iter()
            .map(|subscription| SimpleSubscriptionDTO {
                name: subscription.name.clone(),
                price: subscription.price,
                status: subscription.status,
            })
            .collect()
    }

    pub fn build_subscription_service() -> SubscriptionService {
        SubscriptionService {
            subscription_repository: SubscriptionRepository{}
        }
    }
}
