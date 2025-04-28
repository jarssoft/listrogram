use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Local, Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use super::middleware;
use super::current_datetime;

fn format_error(msg:&str, oline: Option<(usize, &str)>) -> Result<Vec::<(NaiveDateTime, String)>, String> {
    if let Some(line) = oline {
        Err(format!("Error in line {} ('{}'): {}", line.0+1, line.1, msg))
    }else{
        Err(format!("Error: {}", msg))
    }
}

fn parse_from_text(date: NaiveDate, req_body: String) -> Result<Vec::<(NaiveDateTime, String)>, String> {
    let mut progs: Vec::<(NaiveDateTime, String)> = Vec::new();
    let mut lines = req_body.lines().enumerate();

    while let Some(line1) = lines.next() {    
        let time = NaiveTime::parse_from_str(line1.1, "%0H:%0M");
        
        if time.is_err() {
            return format_error("Expected time string (%H:%M).", Some(line1));
        }
        let datetime = NaiveDateTime::new(date, time.unwrap());

        if progs.last().is_some() && datetime < progs.last().unwrap().0 {            
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

        progs.push((datetime, title.to_string().clone()));
    }

    if progs.is_empty() {
        return format_error("No programs added.", None);
    }

    Ok(progs)   

}

#[post("/addtext")]
async fn addtext(data: web::Data<super::AppState>, req_body: String) -> impl Responder {
    let (mut progs, time) = middleware(&data);

    let res: Result<Vec<(NaiveDateTime, String)>, String> = parse_from_text(time.date(), req_body);
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