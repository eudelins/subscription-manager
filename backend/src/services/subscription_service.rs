use crate::repositories::subscription_repository; 
use crate::dao_entities::subscription_dao::SubscriptionDAO;
use crate::dto_entities::subscription_dto::{SubscriptionDTO, CreateSubscriptionDTO};
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;


pub async fn find_subscription_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<SubscriptionDTO> {
    subscription_repository::find_subscription_by_id(db, id).await.map(SubscriptionDTO::from)
}

pub async fn find_all_subscriptions(db: Connection<SubscriptionsDb>) -> Vec<SubscriptionDTO> {
    subscription_repository::find_all_subscriptions(db).await
        .into_iter()
        .map(SubscriptionDTO::from)
        .collect()
}

pub async fn create_subscription(
    db: Connection<SubscriptionsDb>,
    new_sub_dto: CreateSubscriptionDTO
) -> Option<SubscriptionDTO> {
    let new_sub = SubscriptionDAO::from(new_sub_dto);
    let result_dto = subscription_repository::create_subscription(db, new_sub).await.map(SubscriptionDTO::from);
    // self.category_service.add_subscription_to_categories(new_sub, new_sub_dto.categories_id);
    result_dto
}

pub async fn delete_subscription_by_id(db: Connection<SubscriptionsDb>, id: i32) {
    subscription_repository::delete_subscription_by_id(db, id).await
}
