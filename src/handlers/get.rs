use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

#[get("/list")]
async fn index(data: web::Data<crate::AppState>) -> impl Responder {
    let progs = data.progs.lock().unwrap();
    web::Json((*progs).clone())
}

fn current_time(timezoneopt: Option<i32>) -> NaiveTime {
    match timezoneopt {
        Some(timezone) => {
            let tz_offset = FixedOffset::east_opt(timezone).unwrap();
            tz_offset.from_utc_datetime(&Utc::now().naive_utc()).naive_local().time()}
        None => Local::now().naive_local().time()
    }
}

fn progs_by_time(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, time:NaiveTime) -> Vec<(NaiveTime, String)>{
    
    let now2 = progs
        .iter()
        .reduce(|x,y|{
            if y.0 < time {y} else {x}
        }); 

    match now2 {
        Some(prog) => vec![(*prog).clone()],
        None => vec![]        
    }
}

fn middleware(data: &web::Data<crate::AppState>) -> (std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> , NaiveTime) {
    (data.progs.lock().unwrap(), current_time(data.timezone))
}

#[get("/now")]
async fn now(data: web::Data<crate::AppState>) -> impl Responder {
    let (progs, time) = middleware(&data);
    web::Json(progs_by_time(&progs, time)) 
}

fn progs_after(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, time:NaiveTime, max:usize) -> Vec<(NaiveTime, String)>{   
    let mut count=0;

    progs
        .iter()
        .filter(|x| {            
            if x.0 > time {
                count += 1;
                count <= max
            } else {false}})
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>()
}

#[get("/next/{max}")]
async fn next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    web::Json(progs_after(&progs, time, max)) 
}

#[get("/now-and-next/{max}")]
async fn now_and_next(path: web::Path<usize>, data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);
    let max = path.into_inner();
    let mut response = progs_by_time(&progs, time);
    response.append(&mut progs_after(&progs, time, max-1));
    web::Json(response) 
}
