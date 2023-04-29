#[get("/")]
pub fn healthcheck() -> &'static str {
    "Health is OK !"
}


#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn healthcheck_test() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::healthcheck)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Health is OK !");
    }
}
