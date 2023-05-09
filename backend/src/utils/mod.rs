

#[cfg(test)]
pub mod test {
    use crate::rocket;

    use once_cell::sync::OnceCell;
    use rocket::local::blocking::Client;
    use std::sync::Mutex;
    
    use rocket::{Config, figment::Profile};
    
    pub fn get_test_client () -> &'static Mutex<Client> {
        static CLIENT_INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        CLIENT_INSTANCE.get_or_init(|| {
            let rocket = rocket().configure(
                Config::figment().select(Profile::const_new("test"))
            );
            Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
        })
    }
}