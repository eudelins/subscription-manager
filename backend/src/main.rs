#[macro_use]
extern crate rocket;

use rocket_db_pools::{Database, sqlx};

use routes::{subscription_routes::*, brand_routes::*, category_routes::*, healthcheck::healthcheck};
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
        .mount("/categories/", routes![
            find_category_by_id,
            find_all_categories,
            create_category,
            delete_category_by_id
        ])
}
