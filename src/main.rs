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
    
    let mut progs: Vec::<(NaiveTime, String)> = Vec::new();
    let mut lines = req_body.lines();

    while let Some(time) = lines.next() {        
        let time_only = NaiveTime::parse_from_str(time, "%H:%M");
        let title = lines.next().unwrap();

        progs.push((time_only.unwrap(), title.to_string().clone()));
    }

    println!{"{progs:?}"}
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
