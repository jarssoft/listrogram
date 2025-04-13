use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
pub mod handlers;
use handlers::{add, get, feed};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct AppState {
    pub progs: Mutex<Vec::<(NaiveTime, String)>>, // <- Mutex is necessary to mutate safely across threads
    pub timezone: Option<i32>,
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

    let vars: HashMap<String, String> = env::vars().collect();
    let ip = match vars.get("ACTIX_IP") {
        Some(p) => p.as_str(),
        None => "127.0.0.1",
    };
    
    // Note: web::Data created _outside_ HttpServer::new closure
    let appdata = web::Data::new(crate::AppState {
        progs: Mutex::new(Vec::new()),
        timezone: vars.get("TIMEZONE").map(|v| v.parse::<i32>().unwrap())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone()) // <- register the created data
            .service(hello)
            .service(add::addtext)
            .service(get::index)
            .service(get::now)
            .service(get::next)
            .service(get::now_and_next)
            .service(get::now_and_soon)
            .service(feed::feed)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, 8080))?
    .run()
    .await
}
