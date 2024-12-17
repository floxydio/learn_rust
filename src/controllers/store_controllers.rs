use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::{query, query_as, Error, MySqlPool};
use crate::entities::{ResponseWhenBlob, ResponseWhenError, ResponseWhenSuccess, ResponseWhenSuccessDetail, StoreModel, StorePost};

#[get("/store")]
pub async fn get_store(db_pool: web::Data<MySqlPool>) -> impl Responder {
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
pub async fn create_store(body: web::Json<StorePost>,db_pool: web::Data<MySqlPool>) -> impl Responder {
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
pub async fn update_store(path: web::Path<u32>, body: web::Json<StorePost>, db_pool: web::Data<MySqlPool>) -> impl Responder {
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
pub async fn get_store_detail(path: web::Path<u32>,db_pool: web::Data<MySqlPool>) -> impl Responder {
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
