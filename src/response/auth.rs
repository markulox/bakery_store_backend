use chrono::prelude::*;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct FilteredUser{
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser
}

#[derive(Serialize, Debug)]
pub struct UserResponse{
    pub status: String,
    pub data: UserData
}

#[derive(Serialize, Debug)]
pub struct RegistrationSuccessResponse{
    pub account_id: uuid::Uuid
}

#[derive(Serialize, Debug)]
pub struct LoginSuccessResponse{
    pub token: String
}