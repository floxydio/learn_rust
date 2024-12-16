use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ResponseWhenSuccess {
    pub(crate) status: i32,
    pub(crate) error: bool,
    pub(crate)data: Vec<StoreModel>,
    pub(crate) message: String,
}

#[derive(Serialize)]
pub struct ResponseWhenSuccessDetail {
    pub(crate) status: i32,
    pub(crate) error: bool,
    pub(crate)data: StoreModel,
    pub(crate) message: String,
}

#[derive(Serialize)]
pub struct ResponseWhenError {
    pub(crate) status: i32,
    pub(crate) error: bool,
    pub(crate) message: String,
}


#[derive(Serialize)]
pub struct ResponseWhenBlob {
    pub(crate) status: i32,
    pub(crate) error: bool,
    pub(crate) message: String
}

#[derive(Serialize, FromRow)]
pub struct StoreModel {
    pub(crate) store_id: i32,
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) status: i32,
    pub(crate) store_founder_id: Option<i32>,
    pub(crate) created_at:Option<NaiveDateTime>
}

#[derive(Deserialize)]
pub struct StorePost {
    pub(crate) name: String,
    pub(crate) location: String,
    pub(crate) status: i32,
}