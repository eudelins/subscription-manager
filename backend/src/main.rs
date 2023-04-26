#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row, Execute};

#[derive(Database)]
#[database("subscriptions_db")]
struct SubscriptionsDb(sqlx::PgPool);

#[get("/")]
fn index() -> &'static str {
    "Hello Simsim!"
}

#[get("/<id>")]
async fn read(mut db: Connection<SubscriptionsDb>, id: i64) -> Option<String> {
    println!("Query: {:#?}", sqlx::query("SELECT name FROM Subscriptions WHERE id = $1").bind(id).sql());
   sqlx::query("SELECT name FROM Subscriptions WHERE id = $1").bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .map_err(|e| println!("Error: {:?}", e))
       .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(SubscriptionsDb::init())
        .mount("/", routes![index])
        .mount("/db/", routes![read])
}
