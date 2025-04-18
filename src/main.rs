use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use listagram::handlers::{add, get, feed};
use listagram::handlers::build_appdata;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let vars: HashMap<String, String> = env::vars().collect();
    let ip = match vars.get("ACTIX_IP") {
        Some(p) => p.as_str(),
        None => "127.0.0.1",
    };
    
    // Note: web::Data created _outside_ HttpServer::new closure
    let appdata = web::Data::new(
            build_appdata(vars.get("TIMEZONE").map(|v| v.parse::<i32>().unwrap()))
    );

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone()) // <- register the created data
            .service(add::addtext)
            .service(get::index)
            .service(get::now)
            .service(get::next)
            .service(get::now_and_next)
            .service(get::now_and_soon)
            .service(feed::feed)
    })
    .bind((ip, 8080))?
    .run()
    .await
}
