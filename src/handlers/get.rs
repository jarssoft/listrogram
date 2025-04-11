use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};

#[get("/list")]
async fn index(data: web::Data<crate::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

#[get("/now")]
async fn now(data: web::Data<crate::AppState>) -> impl Responder {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
  
    let naive_time = Local::now().naive_local().time();
    
    let now = progs
        .iter()
        .reduce(|x,y|{
            if y.0 < naive_time {y} else {x}
        }); 

    let nows = match now {
        Some(x) => vec![(*now.unwrap()).clone()],
        None => vec![]        
    };

    web::Json(nows)
}

#[get("/next/{max}")]
async fn next3(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
  
    let naive_time = Local::now().naive_local().time();
    let mut count=0;
    let max = path.into_inner();

    let nexts  = progs
        .iter()
        .filter(|x| {            
            if x.0 > naive_time {
                count += 1;
                count <= max
            } else {false}})
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>();

    web::Json(nexts)
}