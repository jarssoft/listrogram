use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
pub mod handlers;
use handlers::{add, get};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct AppState {
    pub progs: Mutex<Vec::<(NaiveTime, String)>>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Note: web::Data created _outside_ HttpServer::new closure
    let progs = web::Data::new(crate::AppState {
        progs: Mutex::new(Vec::new()),
    });

    let vars: HashMap<String, String> = env::vars().collect();
    let ip = match vars.get("ACTIX_IP") {
        Some(p) => p.as_str(),
        None => "127.0.0.1",
    };

    HttpServer::new(move || {
        App::new()
            .app_data(progs.clone()) // <- register the created data
            .service(hello)
            .service(add::addtext)
            .service(get::index)
            .service(get::now)
            .service(get::next3)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, 8080))?
    .run()
    .await
}