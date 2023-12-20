// src/routes/item_routes.rs

use actix_web::web;
use crate::controllers::user_controller;



pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/items", web::post().to(user_controller::createUser));
    // Configure other routes
}

pub fn getUserList(cfg: &mut web::ServiceConfig) {
    cfg.route("/getUserList", web::post().to(user_controller::getUser));
    // Configure other routes
}
