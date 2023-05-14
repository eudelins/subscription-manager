use std::{collections::HashMap};
use std::env;
use rocket::{figment::Figment, Config};

pub fn get_config() -> Figment {
    let mut database_config = HashMap::<&str, &str>::new();
    let mut databases = HashMap::new();
    
    let base_db_url = "postgresql://subscriptions_db_user";
    let (db_host, db_name): (&str, &str);
    
    if cfg!(test) {
        db_name = "subscriptions_db_test";
    } else {
        db_name = "subscriptions_db";
    }
    
    if cfg!(debug_assertions) {
        db_host = "localhost";
        std::env::set_var(
            "PGPASSFILE",
            format!("{}/database/pass.pgpass", env!("CARGO_MANIFEST_DIR"))
        );
    } else {
        db_host = "db";
    }

    let db_url = format!("{}@{}/{}", base_db_url, db_host, db_name);
    database_config.insert(
        "url",
        &db_url
    );

    databases.insert("subscriptions_db", database_config);
    
    Config::figment()
        .merge(("databases", databases))
}


#[cfg(test)]
pub mod test {
    use crate::rocket;

    use once_cell::sync::OnceCell;
    use rocket::local::blocking::Client;
    use std::sync::Mutex;
    
    pub fn get_test_client () -> &'static Mutex<Client> {
        static CLIENT_INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        CLIENT_INSTANCE.get_or_init(|| {
            Mutex::from(Client::tracked(rocket()).expect("valid rocket instance"))
        })
    }
}