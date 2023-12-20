// src/controllers/item_controller.rs

use actix_web::{ 
    web,HttpResponse, Responder};
use crate::config::database::AppState;
use crate::models::user::User;
// use crate::responses::success_response;

// use mongodb::bson::Document;
use mongodb::bson::{doc, Document};
use mongodb::Cursor;
// use std::sync::{Arc, Mutex};
use serde_json::json;

// pub fn configure(cfg:&mut web::ServiceConfig){
//     cfg.route('/getUser',web::get().to(get_all_items));
//     cfg.route('/createUser',web::post().to(create));
// }



pub async fn createUser(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let collection = data.db.collection::<Document>("people");
    let new_item = doc! {
        "name": &user.name,
        "age": &user.age,
    };
    match collection.insert_one(new_item.clone(), None).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({
                "success": true,
                "data": new_item,
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "error": format!("Failed to insert item into the database: {}", e),
            }))
        }
    }

}

pub async fn getUser(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {

    let getUser = data.db.collection::<User>("people");

    // println!(getUser);
    // let docs = vec![ doc! { "name":  &user.name, "age": &user.age }];
    // getUser.find().await;
    

    HttpResponse::Ok().json("items")
}

// #[post("/user")]
// pub async fn create_user(db: Data<AppState>, new_user: Json<User>) -> HttpResponse {
//     let data = User {
//         id: None,
//         name: new_user.name.to_owned(),
//         age: new_user.age.to_owned(),
//     };

//     let user_detail = db.insert_one(data).await;

//     match user_detail {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }