// src/main.rs

use actix_web::{web, App, HttpServer};
// use crate::config::database::AppState;
mod config;
mod controllers;
mod models;
mod routes;
use crate::config::database::{init_mongo_db, AppState};
// Import the configure function from user_routes
use crate::routes::user_routes::{configure,getUserList};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init_mongo_db().await;
    let app_state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure)
            .configure(getUserList)  // Use the imported function directly
    })
    .bind("127.0.0.1:8181")?
    .run()
    .await
}
