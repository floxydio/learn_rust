use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ResponseWhenSuccess {
    pub status: i32,
    pub error: bool,
    pub data: Vec<StoreModel>,
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseWhenSuccessDetail {
    pub status: i32,
    pub error: bool,
    pub data: StoreModel,
    pub message: String,
}

#[derive(Serialize)]
pub struct ResponseWhenError {
    pub status: i32,
    pub error: bool,
    pub message: String,
}


#[derive(Serialize)]
pub struct ResponseWhenBlob {
    pub status: i32,
    pub error: bool,
    pub message: String
}

#[derive(Serialize, FromRow)]
pub struct StoreModel {
    pub store_id: i32,
    pub name: String,
    pub location: String,
    pub status: i32,
    pub store_founder_id: Option<i32>,
    pub created_at:Option<NaiveDateTime>
}

#[derive(Deserialize)]
pub struct StorePost {
    pub name: String,
    pub location: String,
    pub status: i32,
}