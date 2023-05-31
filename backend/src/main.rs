#[macro_use]
extern crate rocket;

use rocket_db_pools::{Database, sqlx};

use services::{
    subscription_service::SubscriptionService,
    brand_service::BrandService
};
use routes::{subscription_routes::*, brand_routes::*, healthcheck::healthcheck};
use fairings::cors::CORS;
use utils::get_config;

pub mod routes;
pub mod repositories;
pub mod services;
pub mod dao_entities;
pub mod dto_entities;
pub mod fairings;
pub mod utils;

#[derive(Database)]
#[database("subscriptions_db")]
pub struct SubscriptionsDb(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::custom(get_config())
        .attach(SubscriptionsDb::init())
        .attach(CORS)
        .manage(SubscriptionService::build_subscription_service())
        .manage(BrandService::build_brand_service())
        .mount("/", routes![healthcheck])
        .mount("/subscriptions/", routes![
            find_subscription_by_id,
            find_all_subscriptions,
            create_subscription,
            delete_subscription_by_id
        ])
        .mount("/brands/", routes![
            find_brand_by_id,
            find_all_brands,
            create_brand,
            delete_brand_by_id
        ])
}
