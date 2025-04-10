use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn formatline(line: (usize, &str)) -> String {
    format!("line {} '{}'", line.0+1, line.1)
}

fn parse_from_text(req_body: String) -> Result<Vec::<(NaiveTime, String)>, String> {
    let mut progs: Vec::<(NaiveTime, String)> = Vec::new();
    let mut lines = req_body.lines().enumerate();

    while let Some(line1) = lines.next() {    
        let time = NaiveTime::parse_from_str(line1.1, "%H:%M");
        if time.is_err() {
            return Err(format!("Error in {}, expected time (%H:%M).", formatline(line1)));
        }        
        if progs.last().is_some() && time.unwrap() < progs.last().unwrap().0 {
            return Err(format!("Error in {}. Added time was before last time.", formatline(line1)));
        }

        let nextline = lines.next();
        if nextline.is_none(){
            return Err(format!("Found end of file, expected program title."));
        }
        let line2 = nextline.unwrap();
           
        if line2.1.is_empty() {
            return Err(format!("Error in {}, expected program title (a string longer than 0).", formatline(line2)));
        }

        progs.push((time.unwrap(), line2.1.to_string().clone()));
    }
    Ok(progs)   

}

#[post("/addtext")]
async fn echo(req_body: String) -> impl Responder {
    let res: Result<Vec<(NaiveTime, String)>, String> = parse_from_text(req_body);
    match res {
        Ok(value) => HttpResponse::Ok().body(format!{"{value:?}"}),
        Err(error) => HttpResponse::BadRequest().body(format!{"{error:?}"}),
    }
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
