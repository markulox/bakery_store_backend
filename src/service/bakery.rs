use actix_web::Responder;

use crate::response::APIResponse;

pub async fn create_bakery() -> impl Responder {
    APIResponse::<()>::ok()
}

pub async fn list_bakery() -> impl Responder {
    APIResponse::<()>::unknown_internal_error()
}