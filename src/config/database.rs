// src/config/database.rs

use mongodb::{Client, Database};

pub struct AppState {
    pub db: Database,
}

pub async fn init_mongo_db() -> Database {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
    let db = client.database("config");
    db
}
