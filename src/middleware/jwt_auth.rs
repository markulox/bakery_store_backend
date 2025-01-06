use actix_web::{error::ErrorUnauthorized, http, web, FromRequest, HttpMessage};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::ready;

use crate::{
    model::users::TokenClaims,
    response::{self, APIResponse},
    BakeryAppState,
};

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let data = req.app_data::<web::Data<BakeryAppState>>().unwrap();
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(
                ErrorUnauthorized(response::APIResponse::<()>::unauthorized()),
            ));
        }

        let key = &DecodingKey::from_secret(data.conf.jwt_conf.jwt_secret.as_bytes());
        let validation = &Validation::default();

        let claims = match decode::<TokenClaims>(&token.unwrap(), &key, validation) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(APIResponse::<()>::unauthorized())));
            }
        };

        let user_id = match uuid::Uuid::parse_str(&claims.sub) {
            Ok(uuid) => uuid,
            Err(e) => {
                return ready(Err(ErrorUnauthorized(APIResponse::new(
                    false,
                    9999,
                    &e.to_string(),
                    None,
                    None::<()>,
                ))));
            }
        };
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());
        ready(Ok(JwtMiddleware { user_id }))
    }
}
