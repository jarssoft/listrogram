use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/addtext")]
async fn echo(req_body: String) -> impl Responder {
    let lines = req_body.lines();
    let mut progs: Vec::<(NaiveTime, String)> = Vec::new();
    let mut time: Option<NaiveTime> = Option::None;
    lines.for_each(|line| {
        if line.len()==5 {
            let time_only: Result<NaiveTime, ParseError> = NaiveTime::parse_from_str(line, "%H:%M");
            time = Option::Some(time_only.unwrap());
        }else{
            if time != Option::None {
                progs.push((time.unwrap(), line.to_string().clone()));
                time = Option::None;
            }
        }
    });
    print!{"{progs:?}"}

    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    print!("line");

    let vars: HashMap<String, String> = env::vars().collect();
    let ip = match vars.get("ACTIX_IP") {
        Some(p) => p.as_str(),
        None => "127.0.0.1",
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, 8080))?
    .run()
    .await
}
