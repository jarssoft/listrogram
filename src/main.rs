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
    let mut lines = req_body.lines().enumerate();

    while let Some(time) = lines.next() {    
        let time_only = NaiveTime::parse_from_str(time.1, "%H:%M");
        assert!(time_only.is_ok(), "Error in line {}. Expected time (%H:%M), found {}.", time.0+1, time.1);

        let title = lines.next();       
        assert!(title.is_some(), "Expected program title, found end of file.");
        assert!(title.unwrap().1.len()>0, "Error in line {}. Expected program title (a string longer than 0).", title.unwrap().0+1);

        progs.push((time_only.unwrap(), title.unwrap().1.to_string().clone()));
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
