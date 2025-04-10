//use crate::db_connection::{DbPool, MyPooledConnection};
//use actix_web::{web, HttpResponse};

pub mod add;
pub mod get;

/*
pub fn pool_handler(pool: web::Data<DbPool>) -> Result<MyPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
 */