use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct AuthPost {
   pub name: String,
   pub email: String,
   pub password: String,
}

#[derive(Serialize)]
pub struct AuthTokenSignIn {
    pub status: i32,
    pub error: bool,
    pub token: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseSignUp {
    pub status: i32,
    pub error: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseErrorAuth {
    pub status: i32,
    pub error: bool,
    pub message: String,
}


#[derive(Deserialize)]
pub struct AuthSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize,FromRow)]
pub struct AuthFindByProfile {
    pub users_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
