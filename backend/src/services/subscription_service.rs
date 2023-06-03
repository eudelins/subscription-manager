use crate::dao_entities::subscription_dao::SubscriptionDAO;
use crate::dto_entities::subscription_dto::{
    CreateOrUpdateSubscriptionDTO, EntireSubscriptionDTO, SubscriptionDTO,
};
use crate::repositories::subscription_repository;
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;
use sqlx::{Acquire, Postgres, Transaction};

use super::category_service;

pub async fn find_subscription_by_id(
    mut db: Connection<SubscriptionsDb>,
    id: i32,
) -> Option<EntireSubscriptionDTO> {
    let result_dto = subscription_repository::find_subscription_by_id(&mut db, id)
        .await
        .map(EntireSubscriptionDTO::from);

    if let Some(mut res_dto_inner) = result_dto {
        res_dto_inner.categories_id =
            subscription_repository::fetch_subscription_categories_id(&mut db, id).await;
        Some(res_dto_inner)
    } else {
        Option::None
    }
}

pub async fn find_all_subscriptions(mut db: Connection<SubscriptionsDb>) -> Vec<SubscriptionDTO> {
    subscription_repository::find_all_subscriptions(&mut db)
        .await
        .into_iter()
        .map(SubscriptionDTO::from)
        .collect()
}

pub async fn find_all_subscriptions_with_categories(
    mut db: Connection<SubscriptionsDb>,
) -> Vec<EntireSubscriptionDTO> {
    let mut subs_dto = subscription_repository::find_all_subscriptions(&mut db)
        .await
        .into_iter()
        .map(EntireSubscriptionDTO::from)
        .collect::<Vec<EntireSubscriptionDTO>>();
    for mut sub in subs_dto.iter_mut() {
        sub.categories_id =
            subscription_repository::fetch_subscription_categories_id(&mut db, sub.id).await;
    }
    subs_dto
}

pub async fn create_or_update_subscription(
    mut db: Connection<SubscriptionsDb>,
    new_sub_dto: CreateOrUpdateSubscriptionDTO,
    is_updated: bool,
) -> Option<SubscriptionDTO> {
    let mut transaction = match db.begin().await {
        Ok(tr) => tr,
        Err(_) => return Option::None,
    };

    let categories_id = new_sub_dto.categories_id.clone();
    let new_sub = SubscriptionDAO::from(new_sub_dto);
    let result_dto = subscription_repository::create_or_update_subscription(
        &mut transaction,
        new_sub,
        is_updated,
    )
    .await
    .map(SubscriptionDTO::from);

    if let Some(res_dto_inner) = result_dto {
        if is_updated {
            let successful_remove =
                subscription_repository::remove_subscription_from_all_categories(
                    &mut transaction,
                    res_dto_inner.id,
                )
                .await
                .is_some();
            if !successful_remove {
                return rollback_and_return_none(transaction).await;
            }
        }

        let successful_remove = category_service::add_subscription_to_categories(
            &mut transaction,
            res_dto_inner.id,
            categories_id,
        )
        .await
        .is_some();
        if !successful_remove {
            return rollback_and_return_none(transaction).await;
        }
        if let Err(err) = transaction.commit().await {
            println!("Commit error : {:?}", err);
            return Option::None;
        };
        Some(res_dto_inner)
    } else {
        rollback_and_return_none(transaction).await
    }
}

pub async fn delete_subscription_by_id(mut db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    subscription_repository::delete_subscription_by_id(&mut db, id).await
}

pub async fn fetch_subscription_categories_id(
    mut db: Connection<SubscriptionsDb>,
    id: i32,
) -> Vec<i32> {
    subscription_repository::fetch_subscription_categories_id(&mut db, id).await
}

async fn rollback_and_return_none<T>(transaction: Transaction<'_, Postgres>) -> Option<T> {
    let _ = transaction
        .rollback()
        .await
        .map_err(|err| println!("Rollback error : {:?}", err));
    Option::None
}
