#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("subscriptions")]
struct Subscriptions(sqlx::SqlitePool);

#[get("/")]
fn index() -> &'static str {
    "Hello Simsim!"
}

#[get("/<id>")]
async fn read(mut db: Connection<Subscriptions>, id: i64) -> Option<String> {
   sqlx::query("SELECT name FROM Subscriptions WHERE id = ?").bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Subscriptions::init())
        .mount("/", routes![index])
        .mount("/db/", routes![read])
}
