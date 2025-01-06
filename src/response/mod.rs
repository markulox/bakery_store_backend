use core::fmt;
use std::{fmt::write, result};

use actix_web::{body::BoxBody, cookie::Cookie, HttpResponse, Responder};
use serde::Serialize;
use validator::ValidationErrors;

use crate::repository;

pub mod auth;

// pub enum BusinessCode {
//     ObjectCreated,
//     Ok,
//     Unauthorized,
// }

pub trait Error {
    fn get_business_code(&self) -> i32;
    fn get_error_details(&self) -> Option<Vec<&str>>;
}

#[derive(Debug, Serialize)]
pub struct APIResponse<'a, T> where T:Serialize{
    success: bool,
    business_code: i32,
    message: String,
    error_details: Option<Vec<String>>,
    results: Option<T>,
    #[serde(skip_serializing)]
    cookies: Option<Cookie<'a>>
}

impl<'a, T> Responder for APIResponse<'a, T> where T:Serialize{
    type Body = BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let struct_obj = self;
        let mut response = match &struct_obj.business_code {
            1000..=2999 => HttpResponse::Ok(),
            4001 => HttpResponse::Unauthorized(),
            4009 => HttpResponse::Conflict(),
            4010 => HttpResponse::BadRequest(),
            8000..9000 => HttpResponse::BadRequest(), // Invalid parameters
            9000..=9999 | _ => HttpResponse::InternalServerError(), // Internal server error
        };
        if let Some(c) = &struct_obj.cookies {
            response.cookie(c.to_owned());
        }
        response.json(struct_obj)
    }
}

impl<'a, T> fmt::Display for APIResponse<'a, T> where T:Serialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.business_code {
            1000..2000 => write!(f, "OK"),
            4001 => write!(f, "User Fuck-up!"),
            4009 => write!(f, "Conflict Naja!"),
            4010 => write!(f, "Login with invalid email or password"),
            8000..9000 => write!(f, "User input Fuck-up!"), // Invalid parameters
            9000..=9999 | _ => write!(f, "OK... I fuckup this time"), // Internal server error
        }
    }
}

impl <'a, T> APIResponse<'a, T> where T:Serialize {
    pub fn new(
        success: bool,
        business_code: i32,
        msg: &str,
        err_details: Option<Vec<&str>>,
        results: Option<T>
    ) -> Self {
        Self {
            success: success,
            business_code: business_code,
            message: msg.to_string(),
            error_details: err_details.map_or(None, 
                |v| Some(v.iter().map(|s| s.to_string()).collect())
            ),
            results: results,
            cookies: None
        }
    }

    pub fn ok() -> Self {
        Self {
            success: true,
            business_code: 1000,
            message: "Everything is okay".to_string(),
            error_details: None,
            results: None::<T>,
            cookies: None
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            success: false,
            business_code: 4001,
            message: "Unauthorized Access".to_string(),
            error_details: None,
            results: None::<T>,
            cookies: None
        }
    }

    pub fn unknown_internal_error() -> Self {
        Self {
            success: false,
            business_code: 9999,
            message: "An unknown cause internal error has occured!".to_string(),
            error_details: None,
            results: None::<T>,
            cookies: None
        }
    }

    pub fn validation_error(errs: ValidationErrors) -> Self {
        Self {
            success: false,
            business_code: 4010,
            message: "Invalid parameters entered".to_string(),
            error_details: Some(errs.into_errors().iter().map(|(s, _)| format!("{}", s)).collect()),
            results: None::<T>,
            cookies: None
        }
    }

    pub fn with_cookie(mut self, c:Cookie<'a>) -> Self{
        self.cookies = Some(c);
        self
    }
}
