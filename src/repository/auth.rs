use core::fmt;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use sea_orm::{ActiveValue, ColumnTrait, DbConn, EntityTrait, InsertResult, QueryFilter};

use crate::{model::{self, users::{self, LoginUserSchema, RegisterUserSchema, TokenClaims}}, response::Error, JWTConfig};


pub enum AuthError {
    RegisterEmailAlreadyExist,
    PasswordHashingFailed,
    DatabaseError(String),
    IncorrectLogin,
    TokenEncodingError
}

impl Error for AuthError {
    fn get_business_code(&self) -> i32 {
        match &self {
            AuthError::RegisterEmailAlreadyExist => 4009,
            AuthError::IncorrectLogin => 4010,

            AuthError::DatabaseError(_) => 9000,
            AuthError::PasswordHashingFailed => 9001,
            AuthError::TokenEncodingError => 9002
        }
    }
    
    fn get_error_details(&self) -> Option<Vec<&str>> {
        match &self {
            AuthError::DatabaseError(e) => Some(vec![e.as_str()]),
            _ => None
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AuthError::RegisterEmailAlreadyExist => write!(f, "Email already exist"),
            AuthError::PasswordHashingFailed => write!(f, "Password Hashing Error"),
            AuthError::DatabaseError(_) => write!(f, "Database Error"),
            AuthError::IncorrectLogin => write!(f, "Incorrect Login information"),
            AuthError::TokenEncodingError => write!(f, "Token Encoding Error")
        }
    }
}

pub struct AuthRepository<'a> {
    db: DbConn,
    jwt_config: &'a JWTConfig
}

impl<'a> AuthRepository <'a>{
    pub fn new(db: DbConn, jwt_config: &'a JWTConfig) -> Self {
        Self {db, jwt_config}
    }

    pub async fn register_new_user(
        &self,
        register_schema: RegisterUserSchema,
    ) -> Result<InsertResult<users::ActiveModel>, AuthError> {
        // Extract user info from schema struct
        let reg_name = register_schema.name.unwrap();
        let reg_email = register_schema.email.unwrap();
        let reg_password = register_schema.password.unwrap();
        let reg_photo = register_schema.photo.unwrap_or_default();

        // Check Email duplication b4 create new account
        let duplicate_email = users::Entity::find()
            .filter(users::Column::Email.eq(&reg_email))
            .one(&self.db).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        if !duplicate_email.is_none() {
            return Err(AuthError::RegisterEmailAlreadyExist);
        }

        let salt = SaltString::generate(&mut OsRng);

        // Method .hash_password only available while `use argon2::password_hash::PasswordHasher`
        let hashed_password = Argon2::default()
            .hash_password(reg_password.as_bytes(), &salt)
            .map_err(|_| AuthError::PasswordHashingFailed)?.to_string();

        let new_user = users::ActiveModel {
            id: ActiveValue::set(uuid::Uuid::new_v4()),
            name: ActiveValue::set(reg_name),
            email: ActiveValue::set(reg_email),
            photo: ActiveValue::set(reg_photo),
            verified: ActiveValue::set(false),
            password: ActiveValue::set(hashed_password),
            ..Default::default()
        };
        model::prelude::Users::insert(new_user).exec(&self.db).await.map_err(|e| AuthError::DatabaseError(e.to_string()))
    }

    pub async fn login_user(&self, login_schema: LoginUserSchema) -> Result<String, AuthError> {
        // Extract data from schema
        let login_email = login_schema.email.unwrap();
        let login_password = login_schema.password.unwrap();
        
        // Looking for a user that match the entered Email
        let user_may_none = model::users::Entity::find()
            .filter(users::Column::Email.eq(&login_email))
            .one(&self.db).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        if user_may_none.is_none() {
            return Err(AuthError::IncorrectLogin);
        }

        let user = user_may_none.unwrap();

        // Compare the hased password
        let hased_password = PasswordHash::new(&user.password).map_err(|_| AuthError::PasswordHashingFailed)?;
        // If there was an error, thats mean the password is incorrect!
        Argon2::default().verify_password(login_password.as_bytes(), &hased_password).map_err(|_| AuthError::IncorrectLogin)?;

        // If everything is fine, then build the token
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        // TODO: How about read from env?
        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
        let claims = TokenClaims {
            sub: user.id.to_string(),
            exp,
            iat
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.jwt_config.jwt_secret.as_bytes()))
            .map_err(|e| {
                eprintln!("<X>: JWT Token Generation Error {}", e.to_string());
                AuthError::TokenEncodingError
            })?;

        Ok(token)
    }
}
