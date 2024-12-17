use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{query, query_as, MySqlPool};
use validator::{Validate};
use crate::entities::{AuthFindByProfile, AuthPost, AuthSignIn, AuthTokenSignIn, ErrorValidation, ResponseErrorAuth, ResponseSignUp};
use crate::helper::format_validation_errors;

#[post("/sign-up")]
pub async fn sign_up_user(body: web::Json<AuthPost>,db_pool: web::Data<MySqlPool>) -> impl Responder {
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

#[post("/sign-in")]
pub async fn sign_in_user(body: web::Json<AuthSignIn>,db_pool: web::Data<MySqlPool> ) -> impl Responder {
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
