use dotenvy::dotenv;
use std::env;
use sqlx::{MySqlPool};

pub async fn connect_db() -> MySqlPool {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MySqlPool::connect(&database_url).await.expect("Failed to connect database")
}