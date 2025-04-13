use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike, Utc};
use std::ops::Range;
use crate::utils::progs::{middleware, progs_after, progs_by_time, current_dateime, progs_in_time};

#[get("/list")]
async fn index(data: web::Data<crate::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

#[get("/now")]
async fn now(data: web::Data<crate::AppState>) -> impl Responder {
    let (progs, time) = middleware(&data);
    web::Json(progs_by_time(&progs, time.time())) 
}

#[get("/next/{max}")]
async fn next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    web::Json(progs_after(&progs, time.time(), max)) 
}

#[get("/now-and-next/{max}")]
async fn now_and_next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    let mut response = progs_by_time(&progs, time.time());
    response.append(&mut progs_after(&progs, time.time(), max-1));
    web::Json(response) 
}

#[get("/now-and-soon/{minutes}")]
async fn now_and_soon(path: web::Path<i64>, data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let minutes = path.into_inner();   
    let mut response = progs_by_time(&progs, time.time()); 
    response.append(&mut progs_in_time(&progs, time.time()..time.time() + TimeDelta::try_minutes(minutes).unwrap()));
    web::Json(response) 
}

