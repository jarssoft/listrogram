use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
        let time = NaiveTime::parse_from_str(line1.1, "%H:%M");
        if time.is_err() {
            return format_error("Expected time (%H:%M).", Some(line1));
        }        
        if progs.last().is_some() && time.unwrap() < progs.last().unwrap().0 {            
            return format_error("Added time was before last time.", Some(line1));
        }

        let nextline = lines.next();
        if nextline.is_none(){
            return format_error("Found end of file, expected program title.", nextline);
        }
        let line2 = nextline.unwrap();
           
        if line2.1.is_empty() {
            return format_error("Program title must be longer than 0.", nextline);
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
