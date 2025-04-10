use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
//use chrono::format::ParseError;
use std::sync::Mutex;

struct AppState {
    progs: Mutex<Vec::<(NaiveTime, String)>>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/list")]
async fn index(data: web::Data<AppState>) -> String {
    let progs = data.progs.lock().unwrap();
    format!("Programs: {progs:?}")
}

#[get("/now")]
async fn now(data: web::Data<AppState>) -> String {
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
async fn next3(data: web::Data<AppState>) -> String {
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

fn format_error(msg:&str, oline: Option<(usize, &str)>) -> Result<Vec::<(NaiveTime, String)>, String> {
    if let Some(line) = oline {
        Err(format!("Error in line {} ('{}'): {}", line.0+1, line.1, msg))
    }else{
        Err(format!("Error: {}", msg))
    }
}

fn parse_from_text(req_body: String) -> Result<Vec::<(NaiveTime, String)>, String> {
    let mut progs: Vec::<(NaiveTime, String)> = Vec::new();
    let mut lines = req_body.lines().enumerate();

    while let Some(line1) = lines.next() {    
        let time = NaiveTime::parse_from_str(line1.1, "%0H:%0M");
        if time.is_err() {
            return format_error("Expected time string (%H:%M).", Some(line1));
        }        
        if progs.last().is_some() && time.unwrap() < progs.last().unwrap().0 {            
            return format_error("Added time was before last time.", Some(line1));
        }

        let nextline = lines.next();
        if nextline.is_none(){
            return format_error("Found end of file, expected program title.", nextline);
        }

        let title = nextline.unwrap().1;           
        if title.is_empty() {
            return format_error("Program title must be longer than 0.", nextline);
        }

        progs.push((time.unwrap(), title.to_string().clone()));
    }
    Ok(progs)   

}

#[post("/addtext")]
async fn echo(data: web::Data<AppState>, req_body: String) -> impl Responder {

    let mut progs = data.progs.lock().unwrap(); // <- get counter's MutexGuard
    
    let res: Result<Vec<(NaiveTime, String)>, String> = parse_from_text(req_body);
    match res {
        Ok(value) => {
            *progs=value.clone(); 
            HttpResponse::Ok().body(format!{"{value:?}"})
        },
        Err(error) => HttpResponse::BadRequest().body(format!{"{error:?}"}),
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Note: web::Data created _outside_ HttpServer::new closure
    let progs = web::Data::new(AppState {
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
            .service(echo)
            .service(index)
            .service(now)
            .service(next3)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, 8080))?
    .run()
    .await
}