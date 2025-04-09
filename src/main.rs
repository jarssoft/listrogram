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
    format!("line {} \"{}\"", line.0+1, line.1)
}

#[post("/addtext")]
async fn echo(req_body: String) -> impl Responder {
    
    let progs={
        let mut progs: Vec::<(NaiveTime, String)> = Vec::new();
        let mut lines = req_body.lines().enumerate();

        while let Some(line1) = lines.next() {    
            let time = NaiveTime::parse_from_str(line1.1, "%H:%M");
            assert!(time.is_ok(), "Error in {}, expected time (%H:%M).", formatline(line1));

            let line2 = lines.next().expect("Found end of file, expected program title.");       
            assert!(line2.1.len()>0, "Error in {}, expected program title (a string longer than 0).", formatline(line2));

            progs.push((time.unwrap(), line2.1.to_string().clone()));
        }
        progs
    };
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
