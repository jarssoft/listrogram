use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/addtext")]
async fn echo(req_body: String) -> impl Responder {
    let lines = req_body.lines();
    let mut progs: Vec::<(String, String)> = Vec::new();
    let mut time: &str = "";
    lines.for_each(|line| {
        if line.len()==5 {
            time = line;
        }else{
            progs.push((time.to_string().clone(), line.to_string().clone()));
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
