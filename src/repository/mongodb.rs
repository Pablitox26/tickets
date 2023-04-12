use dotenv::dotenv;

use std::env;

use mongodb::{
    Client, Database,
};

pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let username = match env::var("MONGO_USERNAME") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGO_USERNAME"),
        };

        let password = match env::var("MONGO_PASSWORD") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGO_PASSWORD"),
        };

        let host = match env::var("MONGO_HOST") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGO_HOST"),
        };

        let database = match env::var("MONGO_DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGO_DATABASE"),
        };

        let uri = format!("mongodb://{}:{}@{}:27017", username, password, host);

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&database);
        MongoRepo { db }
    }
}