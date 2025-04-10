use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
//use chrono::format::ParseError;
use std::sync::Mutex;

#[get("/list")]
async fn index(data: web::Data<crate::AppState>) -> String {
    let progs = data.progs.lock().unwrap();
    format!("Programs: {progs:?}")
}

#[get("/now")]
async fn now(data: web::Data<crate::AppState>) -> String {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
  
    let naive_time = Local::now().naive_local().time();
    
    let now = progs
        .iter()
        .reduce(|x,y|{
            if y.0 < naive_time {y} else {x}
        }); 

    format!("Programs: {now:?}")
}

#[get("/next3")]
async fn next3(data: web::Data<crate::AppState>) -> String {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
  
    let naive_time = Local::now().naive_local().time();
    let mut count=0;

    let nexts  = progs
        .iter()
        .filter(|x| {            
            if x.0 > naive_time {
                count += 1;
                count <= 3
            } else {false}})
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>();

    format!("Programs: {nexts:?}")
}