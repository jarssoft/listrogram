use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{NaiveDate, NaiveDateTime, TimeDelta};
use crate::utils::{dayparts::to_date, progs::{progs_after, progs_by_time, progs_in_time}};
use super::middleware;
use crate::utils::dayparts::{progs_in_day_part};

#[get("/list")]
async fn list(data: web::Data<super::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

#[get("/now")]
async fn now(data: web::Data<super::AppState>) -> impl Responder {
    let (progs, datetime) = middleware(&data);
    web::Json(progs_by_time(&progs, datetime)) 
}

#[get("/next/{max}")]
async fn next(path: web::Path<usize>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, datetime) = middleware(&data);
    let max = path.into_inner();
    web::Json(progs_after(&progs, datetime, max)) 
}

#[get("/now-and-next/{max}")]
async fn now_and_next(path: web::Path<usize>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, datetime) = middleware(&data);
    let max = path.into_inner();
    let mut response = progs_by_time(&progs, datetime);
    response.append(&mut progs_after(&progs, datetime, max-1));
    web::Json(response) 
}

#[get("/now-and-soon/{minutes}")]
async fn now_and_soon(path: web::Path<i64>, data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, datetime) = middleware(&data);
    let minutes = path.into_inner();   
    let mut response = progs_by_time(&progs, datetime); 
    response.append(&mut progs_in_time(&progs, datetime..datetime + TimeDelta::try_minutes(minutes).unwrap()));
    web::Json(response) 
}

#[get("/today")]
async fn today(data: web::Data<super::AppState>) -> impl Responder {
    let (progs, datetime) = middleware(&data); 
    let result = progs_in_day_part(&progs, &to_date(&datetime));  
    web::Json(result.clone())
}

#[get("/day/{date}")]
async fn day(path: web::Path<NaiveDate>, data: web::Data<super::AppState>) -> impl Responder {
    let (progs, _) = middleware(&data); 
    let date = path.into_inner();    
    let result = progs_in_day_part(&progs, &date);  
    web::Json(result.clone())
}
