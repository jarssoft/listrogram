use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike, Utc};
use crate::utils::progs::{progs_after, progs_by_time, current_dateime, progs_in_time};
use super::middleware;

#[get("/list")]
async fn list(data: web::Data<super::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

#[get("/now")]
async fn now(data: web::Data<super::AppState>) -> impl Responder {
    let (progs, time) = middleware(&data);
    web::Json(progs_by_time(&progs, time.time())) 
}

#[get("/next/{max}")]
async fn next(path: web::Path<usize>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    web::Json(progs_after(&progs, time.time(), max)) 
}

#[get("/now-and-next/{max}")]
async fn now_and_next(path: web::Path<usize>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    let mut response = progs_by_time(&progs, time.time());
    response.append(&mut progs_after(&progs, time.time(), max-1));
    web::Json(response) 
}

#[get("/now-and-soon/{minutes}")]
async fn now_and_soon(path: web::Path<i64>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let minutes = path.into_inner();   
    let mut response = progs_by_time(&progs, time.time()); 
    response.append(&mut progs_in_time(&progs, time.time()..time.time() + TimeDelta::try_minutes(minutes).unwrap()));
    web::Json(response) 
}

