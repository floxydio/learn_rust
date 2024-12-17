mod database;
mod entities;
mod controllers;
mod helper;
use actix_web::{HttpResponse, HttpServer, App, web};
use std::io;
use sqlx::{MySqlPool};
use crate::database::connect_db;
use crate::controllers::{create_store, get_store, get_store_detail, sign_in_user, sign_up_user, update_store};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db_connect: MySqlPool = connect_db().await;
    HttpServer::new(move || App::new().app_data(web::Data::new(db_connect.clone()))
        .service(
            web::scope("/auth").service(sign_in_user).service(sign_up_user)
        ).service(
            web::scope("/store").service(get_store).service(update_store).service(create_store).service(get_store_detail)
    )
        .route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}