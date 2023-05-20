use crate::dao_entities::category_dao::CategoryDAO;
use crate::dto_entities::category_dto::CategoryDTO;
use crate::repositories::category_repository;
use crate::SubscriptionsDb;

use rocket_db_pools::Connection;
use sqlx::{Postgres, Transaction};

pub async fn find_category_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<CategoryDTO> {
    category_repository::find_category_by_id(db, id)
        .await
        .map(CategoryDTO::from)
}

pub async fn find_all_categories(db: Connection<SubscriptionsDb>) -> Vec<CategoryDTO> {
    category_repository::find_all_categories(db)
        .await
        .into_iter()
        .map(CategoryDTO::from)
        .collect()
}

pub async fn create_category(
    db: Connection<SubscriptionsDb>,
    new_cat_dto: CategoryDTO,
) -> Option<CategoryDTO> {
    let new_cat = CategoryDAO::from(new_cat_dto);
    category_repository::create_category(db, new_cat)
        .await
        .map(CategoryDTO::from)
}

pub async fn delete_category_by_id(db: Connection<SubscriptionsDb>, id: i32) -> Option<()> {
    category_repository::delete_category_by_id(db, id).await
}

pub async fn add_subscription_to_categories(
    db_conn: &mut Transaction<'_, Postgres>,
    sub_id: i32,
    categories_id: Vec<i32>,
) -> Option<()> {
    for cat_id in categories_id.iter() {
        category_repository::add_subscription_to_category(&mut *db_conn, sub_id, *cat_id).await?;
    }
    Some(())
}
