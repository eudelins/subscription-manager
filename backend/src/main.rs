#[macro_use] extern crate rocket;

use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::{self};

use services::subscription_service::SubscriptionService;
use routes::subscription_routes::*;
use routes::healthcheck::healthcheck;

pub mod routes;
pub mod repositories;
pub mod services;
pub mod dao_entities;
pub mod dto_entities;

#[derive(Database)]
#[database("subscriptions_db")]
pub struct SubscriptionsDb(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(SubscriptionsDb::init())
        .manage(SubscriptionService::build_subscription_service())
        .mount("/", routes![healthcheck])
        .mount("/subscriptions/", routes![
            find_subscription,
            find_all_subscription
        ])
}
