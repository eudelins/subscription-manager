use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://127.0.0.1:1420"));
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:1420"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE, PATCH, OPTIONS",
        ));
    }
}