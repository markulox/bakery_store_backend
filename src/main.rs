use std::{env, str::FromStr};

use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DbConn};
use service::{get_route_config, health_check::health_check_handler};

mod middleware;
mod model;
mod repository;
mod response;
mod service;

struct BakeryAppState {
    db_conn: DbConn,
    conf: Config,
}

#[derive(Clone)]
pub struct JWTConfig {
    jwt_secret: String,
    jwt_expire_in: String,
    jwt_maxage: i32,
}

#[derive(Clone)]
struct Config {
    jwt_conf: JWTConfig,
}

fn try_load_env<T: FromStr>(var_name: &str) -> Result<T, (&str, &str)> {
    match env::var(var_name) {
        Ok(s) => s.parse::<T>().map_err(|_| (var_name, "is invalid")),
        Err(_) => Err((var_name, "is_missing")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Preparing Bakery Store Backend Server...");
    print!(" -> Reading config from .env file\t\t\t");
    dotenv::dotenv().ok();
    let mut is_env_setup_failed = false;
    let mut error_env_list: Vec<(&str, &str)> = Vec::new();

    let host = try_load_env::<String>("HOST").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        String::new()
    });
    let port = try_load_env::<u16>("PORT").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        0
    });
    let db_url = try_load_env::<String>("DATABASE_URL").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        String::new()
    });
    let jwt_secret = try_load_env::<String>("JWT_SECRET").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        String::new()
    });
    let jwt_expire_in = try_load_env::<String>("JWT_EXPIRED_IN").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        String::new()
    });
    let jwt_maxage = try_load_env::<i32>("JWT_MAXAGE").unwrap_or_else(|e| {
        error_env_list.push(e);
        is_env_setup_failed = true;
        0
    });

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    if is_env_setup_failed {
        println!("[FAILED]");
        for (err_env_name, err_reason) in error_env_list {
            println!("   - \"{err_env_name}\" {err_reason}");
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Improper .env configuration",
        ));
    } else {
        println!("[OK]");
    }

    print!(" -> Connecting to Bakery Store DB Server\t\t");
    let db_conn = match Database::connect(&db_url).await {
        Ok(db_conn) => {
            println!("[OK]");
            db_conn
        }
        Err(e) => {
            println!("[FAILED]");
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                e.to_string(),
            ));
        }
    };
    let app_state = web::Data::new(BakeryAppState {
        db_conn: db_conn,
        conf: Config {
            jwt_conf: JWTConfig {
                jwt_secret,
                jwt_expire_in,
                jwt_maxage,
            },
        },
    });
    println!("Starting Bakery Store Backend Server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(health_check_handler)
            .configure(get_route_config)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
