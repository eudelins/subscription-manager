#[macro_use]
extern crate rocket;

use rocket_db_pools::{sqlx, Database};

use fairings::cors::CORS;
use routes::{
    brand_routes::*, category_routes::*, dataviz_routes::*, healthcheck::healthcheck,
    subscription_routes::*,
};
use utils::get_config;

pub mod dao_entities;
pub mod dto_entities;
pub mod fairings;
pub mod repositories;
pub mod routes;
pub mod services;
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
        .mount(
            "/subscriptions/",
            routes![
                archive_subscription,
                activate_subscription,
                create_subscription,
                delete_subscription_by_id,
                find_all_subscriptions,
                find_all_subscriptions_with_categories,
                find_subscription_by_id,
                update_subscription
            ],
        )
        .mount(
            "/brands/",
            routes![
                create_brand,
                delete_brand_by_id,
                find_all_brands,
                find_brand_by_id
            ],
        )
        .mount(
            "/categories/",
            routes![
                create_category,
                delete_category_by_id,
                find_all_categories,
                find_category_by_id
            ],
        )
        .mount("/dataviz/", routes![get_statistics])
}
