use crate::{repositories::subscription_repository::SubscriptionRepository, dao_entities::subscription_dao::SubscriptionDAO};
use crate::dto_entities::subscription_dto::{SubscriptionDTO, CreateSubscriptionDTO};
use crate::SubscriptionsDb;

use rocket_db_pools::{Connection};

pub struct SubscriptionService {
    subscription_repository: SubscriptionRepository
}

impl SubscriptionService {
    pub async fn find_subscription_by_id(&self, db: Connection<SubscriptionsDb>, id: i32) -> Option<SubscriptionDTO> {
        self.subscription_repository.find_subscription_by_id(db, id).await.map(SubscriptionDTO::from)
    }

    pub async fn find_all_subscriptions(&self, db: Connection<SubscriptionsDb>) -> Vec<SubscriptionDTO> {
        self.subscription_repository.find_all_subscriptions(db).await
            .into_iter()
            .map(SubscriptionDTO::from)
            .collect()
    }

    pub async fn create_subscription(
        &self, db: Connection<SubscriptionsDb>,
        new_sub_dto: CreateSubscriptionDTO
    ) -> Option<SubscriptionDTO> {
        let new_sub = SubscriptionDAO::from(new_sub_dto);
        let result_dto = self.subscription_repository.create_subscription(db, new_sub).await.map(SubscriptionDTO::from);
        // self.category_repository.add_subscription_to_categories(new_sub, new_sub_dto.categories_id);
        result_dto
    }

    pub async fn delete_subscription_by_id(&self, db: Connection<SubscriptionsDb>, id: i32) {
        self.subscription_repository.delete_subscription_by_id(db, id).await
    }

    pub fn build_subscription_service() -> SubscriptionService {
        SubscriptionService {
            subscription_repository: SubscriptionRepository{}
        }
    }
}
