use actix_web::web;
use auth::{login, register};
use bakery::{create_bakery, list_bakery};

pub mod health_check;
mod bakery;
mod auth;

pub fn get_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/bakery")
            .route("", web::post().to(create_bakery))
            .route("", web::get().to(list_bakery))
    );

    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    );
}