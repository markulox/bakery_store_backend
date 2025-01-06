use actix_web::{cookie::Cookie, web, Responder};
use chrono::Utc;
use validator::Validate;

use crate::{
    middleware::jwt_auth, model::{
        self,
        users::{LoginUserSchema, RegisterUserSchema},
    }, repository::auth::AuthRepository, response::{
        auth::{FilteredUser, LoginSuccessResponse, RegistrationSuccessResponse},
        APIResponse, Error,
    }, BakeryAppState
};

fn filter_user_record(user: &model::users::Model) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        role: user.role.to_owned(),
        photo: user.photo.to_owned(),
        verified: user.verified,
        created_at: chrono::DateTime::<Utc>::from_naive_utc_and_offset(user.created_at, Utc),
        updated_at: chrono::DateTime::<Utc>::from_naive_utc_and_offset(user.updated_at, Utc),
    }
}

pub async fn register(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<BakeryAppState>,
) -> impl Responder {
    let auth_repo = AuthRepository::new(data.db_conn.clone(), &data.conf.jwt_conf);
    let register_schema = body.into_inner();
    if let Err(errs) = register_schema.validate() {
        let err_details: Vec<String> = errs
            .into_errors()
            .iter()
            .map(|(msg, _)| format!("{}", msg))
            .collect();
        return APIResponse::<RegistrationSuccessResponse>::new(
            false,
            4010,
            "Invalid Parameter entered",
            Some(err_details.iter().map(|s| s.as_str()).collect()),
            None,
        );
    };

    let reg_res = auth_repo.register_new_user(register_schema).await;
    match reg_res {
        Ok(t) => APIResponse::<RegistrationSuccessResponse>::new(
            true,
            1001,
            "User registration successful",
            None,
            Some(RegistrationSuccessResponse {
                account_id: t.last_insert_id,
            }),
        ),
        Err(e) => APIResponse::<RegistrationSuccessResponse>::new(
            false,
            e.get_business_code(),
            e.to_string().as_str(),
            e.get_error_details(),
            None,
        ),
    }
}

pub async fn login(
    body: web::Json<LoginUserSchema>,
    data: web::Data<BakeryAppState>,
) -> impl Responder {
    let auth_repo = AuthRepository::new(data.db_conn.clone(), &data.conf.jwt_conf);

    let login_schema = body.into_inner();
    if let Err(errs) = login_schema.validate() {
        return APIResponse::<LoginSuccessResponse>::validation_error(errs);
    };
    match auth_repo.login_user(login_schema).await {
        Ok(tk) => APIResponse::<LoginSuccessResponse>::new(
            true,
            2000,
            "Login success",
            None,
            Some(LoginSuccessResponse { token: tk.clone() }),
        )
        .with_cookie(
            Cookie::build("token", tk.clone())
                .path("/")
                .max_age(actix_web::cookie::time::Duration::new(60 * 60, 0))
                .http_only(true)
                .finish(),
        ),
        Err(e) => APIResponse::<LoginSuccessResponse>::new(
            false,
            e.get_business_code(),
            e.to_string().as_str(),
            e.get_error_details(),
            None,
        ),
    }
}

pub async fn logout(_: jwt_auth::JwtMiddleware) -> impl Responder {
    APIResponse::new(true, 2000, "Logout complete", None, None::<()>).with_cookie(
        Cookie::build("token", "")
            .path("/")
            .max_age(actix_web::cookie::time::Duration::new(-1, 0))
            .http_only(true)
            .finish()
    )
}
