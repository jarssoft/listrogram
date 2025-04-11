use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};

#[get("/list")]
async fn index(data: web::Data<crate::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

fn getnow(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>) -> Vec<(NaiveTime, String)>{
    let naive_time = Local::now().naive_local().time();
    
    let now2 = progs
        .iter()
        .reduce(|x,y|{
            if y.0 < naive_time {y} else {x}
        }); 

    match now2 {
        Some(prog) => vec![(*prog).clone()],
        None => vec![]        
    }
}

#[get("/now")]
async fn now(data: web::Data<crate::AppState>) -> impl Responder {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
    web::Json(getnow(&progs)) 
}

fn getnext(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, max:usize) -> Vec<(NaiveTime, String)>{   
    let naive_time = Local::now().naive_local().time();
    let mut count=0;

    progs
        .iter()
        .filter(|x| {            
            if x.0 > naive_time {
                count += 1;
                count <= max
            } else {false}})
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>()
}

#[get("/next/{max}")]
async fn next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
    let max = path.into_inner();
    web::Json(getnext(&progs, max)) 
}

#[get("/now-and-next/{max}")]
async fn now_and_next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let progs: std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> = data.progs.lock().unwrap();
    let max = path.into_inner();
    let mut response = getnow(&progs);
    response.append(&mut getnext(&progs, max-1));
    web::Json(response) 
}
