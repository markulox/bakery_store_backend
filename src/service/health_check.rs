use actix_web::{get, Responder};
use serde::Serialize;

use crate::response::APIResponse;

#[get("/api/health-check")]
async fn health_check_handler() -> impl Responder {
    APIResponse::new(
        true, 
        1000,
        "I'm alive!",
        None,
        None::<()>
    )
}