use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};

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
async fn addtext(data: web::Data<crate::AppState>, req_body: String) -> impl Responder {

    let mut progs = data.progs.lock().unwrap(); // <- get counter's MutexGuard
    
    let res: Result<Vec<(NaiveTime, String)>, String> = parse_from_text(req_body);
    match res {
        Ok(value) => {
            *progs=value.clone(); 
            //HttpResponse::Ok().body(format!{"{value:?}"})
            //web::Json(value)
            let json = web::Json(value);
            HttpResponse::Ok().json(json)
        },
        Err(error) => HttpResponse::BadRequest().body(format!{"{error:?}"}),
    }
}