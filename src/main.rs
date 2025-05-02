use std::env;
use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use listagram::handlers::{add, feed, get};
use listagram::handlers::build_appdata;
use listagram::utils::progs::TimePolicy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let vars: HashMap<String, String> = env::vars().collect();
    let ip = match vars.get("ACTIX_IP") {
        Some(p) => p.as_str(),
        None => "127.0.0.1",
    };
    
    // Use environment variable TIMEZONE=10800 to set fixed timezone in Finland
    let timeformat = match vars.get("TIMEZONE") {
        Some(p) => TimePolicy::Timezone(p.parse::<i32>().unwrap()),
        None => TimePolicy::Naive(),
    };

    let appdata = web::Data::new(
            build_appdata(timeformat)
    );

    HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone()) // <- register the created data
            .service(add::addtext)
            .service(add::addtextdate)
            .service(get::list)
            .service(get::now)
            .service(get::next)
            .service(get::now_and_next)
            .service(get::now_and_soon)
            .service(get::today)  
            .service(get::day)            
            .service(feed::feed)
            //.service(feed::day)
    })
    .bind((ip, 8080))?
    .run()
    .await
}
