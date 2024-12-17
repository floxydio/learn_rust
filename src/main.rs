mod database;
mod entities;

use actix_web::{get, HttpResponse, Responder, HttpServer, App, web, post, put};
use std::io;
use sqlx::{query, query_as, Error, FromRow, MySqlPool};
use crate::database::connect_db;
use crate::entities::{ResponseWhenBlob, ResponseWhenError, ResponseWhenSuccess, ResponseWhenSuccessDetail, StoreModel, StorePost, AuthPost, AuthFindByProfile, ResponseErrorAuth, ResponseSignUp, AuthSignIn, AuthTokenSignIn, ErrorValidation, FieldError};
use bcrypt::{DEFAULT_COST, hash, verify};
use validator::{Validate, ValidationErrors};

#[get("/store")]
async fn get_store(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result: Result<Vec<StoreModel>, Error> = query_as::<_, StoreModel>("SELECT * FROM store")
        .fetch_all(db_pool.get_ref())
        .await;    match result {
        Ok(data) => {
                let data_json: ResponseWhenSuccess = ResponseWhenSuccess {
                status: 200,
                error: false,
                data,
                message: "Successfully Get Data Store".parse().unwrap()
            };
            HttpResponse::Ok().json(data_json)
        }
        Err(error) => {
            let error_json: ResponseWhenError = ResponseWhenError {
                status: 400,
                error: true,
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(error_json)
        }
    }
}

#[post("/store")]
async fn create_store(body: web::Json<StorePost>,db_pool: web::Data<MySqlPool>) -> impl Responder {
    println!("Data {}", body.name);
    let query_post = query("INSERT INTO store (name, location, status) VALUES (?, ?, ?)").bind(&body.name).bind(&body.location).bind(&body.status).execute(db_pool.get_ref()).await;
    match query_post {
        Ok(_) => {
            HttpResponse::Created().json(ResponseWhenBlob {
                message:"Successfully Insert to DB".parse().unwrap(),
                status: 201,
                error: false,
            })
        }
        Err(error) => {
            let error_json: ResponseWhenError = ResponseWhenError {
                status: 400,
                error: true,
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(error_json)
        }
    }

}

#[put("/store/{store_id}")]
async fn update_store(path: web::Path<(u32)>, body: web::Json<StorePost>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let store_id = path.into_inner();
    let query_result_update= query("UPDATE store SET name = ?, location = ?, status = ? WHERE store_id = ?").bind(&body.name).bind(&body.location).bind(&body.status).bind(store_id).execute(db_pool.get_ref()).await;
    match query_result_update {
        Ok(_) => {
            HttpResponse::Ok().json(ResponseWhenBlob {
                status: 200,
                message: "Success Update".parse().unwrap(),
                error: false,
            })
        }
        Err(error) => {
            let error_json: ResponseWhenError = ResponseWhenError {
                status: 400,
                error: true,
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(error_json)
        }
    }
}

#[get("/store-detail/{store_id}")]
async fn get_store_detail(path: web::Path<u32>,db_pool: web::Data<MySqlPool>) -> impl Responder {
    let store_id = path.into_inner();
    let result_data =  query_as::<_, StoreModel>("SELECT * FROM store WHERE store_id = ?").bind(store_id).fetch_one(db_pool.get_ref()).await;

    match result_data {
        Ok(data) => {
            HttpResponse::Ok().json(ResponseWhenSuccessDetail {
                status: 200,
                message: "Successfully Get Detail Store".parse().unwrap(),
                error: false,
                data
            })
        }
        Err(error) => {
            let error_json: ResponseWhenError = ResponseWhenError {
                status: 400,
                error: true,
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(error_json)
        }
    }
}

#[post("/sign-up")]
async fn sign_up_user(body: web::Json<AuthPost>,db_pool: web::Data<MySqlPool>) -> impl Responder {
    if let Err(errors) = body.validate() {
        let structured_errors = format_validation_errors(errors);

        let error_json: ErrorValidation = ErrorValidation {
            status: 400,
            error: true,
            validate: structured_errors,
            message: "Missing Validation".to_string()
        };
        return HttpResponse::BadRequest().json(error_json);
    }
    let hashed = hash(&body.password, DEFAULT_COST);
    let query_insert = query("INSERT INTO users(name,password,email) VALUES (?,?,?)").bind(&body.name).bind(hashed.unwrap()).bind(&body.email).execute(db_pool.get_ref()).await;

    match query_insert {
        Ok(_) => {
            HttpResponse::Ok().json(ResponseSignUp {
                error: false,
                message: "Successfully Register".parse().unwrap(),
                status: 200
            })
        }
        Err(error) => {
            let error_json: ResponseErrorAuth = ResponseErrorAuth {
                status: 400,
                error: true,
                message: error.to_string(),
            };
            HttpResponse::InternalServerError().json(error_json)
        }
    }
}
fn format_validation_errors(errors: ValidationErrors) -> Vec<FieldError> {
    let mut field_errors = Vec::new();

    for (field, errors) in errors.field_errors() {
        for error in errors {
            field_errors.push(FieldError {
                field: field.to_string(),
                message: error.message.clone().unwrap_or_default().to_string(),
            });
        }
    }

    field_errors
}
#[post("/sign-in")]
async fn sign_in_user(body: web::Json<AuthSignIn>,db_pool: web::Data<MySqlPool> ) -> impl Responder {
    println!("{}", body.email);
    let query_find_user= query_as::<_, AuthFindByProfile>("SELECT u.users_id,u.name,u.email,u.password FROM users u WHERE u.email = ?").bind(&body.email).fetch_one(db_pool.get_ref()).await;
    if query_find_user.is_ok() {
        let password : String = query_find_user.unwrap().password;
        match verify(&body.password, &password) {
            Ok(true)=>{
            HttpResponse::Ok().json(AuthTokenSignIn {
            status: 200,
            error: false,
            token: "".to_string(),
            message: "Successfully Login".parse().unwrap()
            })
            }
            Ok(false) => {
                HttpResponse::BadRequest().json(ResponseErrorAuth {
                    message: "Wrong Password".to_string(),
                    error: true,
                    status: 400
                })
            }
            Err(error)=>{
                HttpResponse::BadRequest().json(ResponseErrorAuth {
                    message: error.to_string(),
                    error: true,
                    status: 400
                })
            }
        }

    } else {
        HttpResponse::NotFound().json(ResponseErrorAuth {
            message: "Username not found".to_string(),
            error: true,
            status: 404
        })
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let db_connect: MySqlPool = connect_db().await;
    HttpServer::new(move || App::new().app_data(web::Data::new(db_connect.clone())).service(get_store).service(create_store).service(update_store).service(get_store_detail).service(sign_up_user).service(sign_in_user).route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}