use crate::services::subscription_service::{SubscriptionService};
use crate::dto_entities::subscription_dto::SimpleSubscriptionDTO;
use crate::SubscriptionsDb;

use rocket::State;
use rocket_db_pools::{Connection};
use rocket::serde::json::Json;


#[get("/<id>")]
pub async fn find_subscription(
    subscription_service: &State<SubscriptionService>,
    db: Connection<SubscriptionsDb>,
    id: i32
) -> Option<Json<SimpleSubscriptionDTO>> {
    match subscription_service.find_subscription(db, id).await {
        Some(subscription) => Some(Json(subscription)),
        None => None,
    }
}

#[get("/")]
pub async fn find_all_subscription(
    subscription_service: &State<SubscriptionService>,
    db: Connection<SubscriptionsDb>
) -> Json<Vec<SimpleSubscriptionDTO>> {
    Json(subscription_service.find_all_subscription(db).await)
}