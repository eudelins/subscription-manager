use crate::repositories::subscription_repository; 
use crate::dao_entities::subscription_dao::SubscriptionDAO;
use crate::dto_entities::subscription_dto::{SubscriptionDTO, CreateSubscriptionDTO, EntireSubscriptionDTO};
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;

use super::category_service;


pub async fn find_subscription_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<EntireSubscriptionDTO> {
    let result_dto = subscription_repository::find_subscription_by_id(
        &mut db,
        id
    ).await.map(EntireSubscriptionDTO::from);
    
    if let Some(mut res_dto_inner) = result_dto {
        res_dto_inner.categories_id = subscription_repository::fetch_subscription_categories_id(
            &mut db,
            id
        ).await;
        Some(res_dto_inner)
    } else {
        Option::None
    }
}

pub async fn find_all_subscriptions(mut db: Connection<SubscriptionsDb>) -> Vec<SubscriptionDTO> {
    subscription_repository::find_all_subscriptions(&mut db).await
        .into_iter()
        .map(SubscriptionDTO::from)
        .collect()
}

pub async fn create_subscription(
    mut db: Connection<SubscriptionsDb>,
    new_sub_dto: CreateSubscriptionDTO
) -> Option<SubscriptionDTO> {
    let categories_id = new_sub_dto.categories_id.clone();
    let new_sub = SubscriptionDAO::from(new_sub_dto);
    let result_dto = subscription_repository::create_subscription(
        &mut db,
        new_sub
    ).await.map(SubscriptionDTO::from);

    if let Some(res_dto_inner) = result_dto {
        category_service::add_subscription_to_categories(
            db,
            res_dto_inner.id,
            categories_id
        ).await;
        Some(res_dto_inner)
    } else {
        Option::None
    }
}

pub async fn delete_subscription_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    subscription_repository::delete_subscription_by_id(&mut db, id).await
}

pub async fn fetch_subscription_categories_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Vec<i32> {
    subscription_repository::fetch_subscription_categories_id(&mut db, id).await
}
