use actix_web::{post, web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use super::middleware;
use crate::utils::parser::parse_from_text;

fn add(progs:&mut Vec<(NaiveDateTime, String)>, date: &NaiveDate, req_body: &String) -> impl Responder {
    let res: Result<Vec<(NaiveDateTime, String)>, String> = parse_from_text(*date, req_body);
    match res {
        Ok(mut newprogs) => {
            let valuecopy=Vec::from(newprogs.clone());
            if !(*progs).is_empty() {                
                if (*progs).last().unwrap().0 > newprogs.first().unwrap().0 {
                    return HttpResponse::BadRequest().body("Error. Days most be addded by order.");
                }
            }
            (*progs).append(&mut newprogs); 

            let json = web::Json(valuecopy);
            HttpResponse::Ok().json(json)
        },
        Err(error) => HttpResponse::BadRequest().body(format!{"{error:?}"}),
    }
}

#[post("/addtext")]
async fn addtext(data: web::Data<super::AppState>, req_body: String) -> impl Responder {
    let (mut progs, datetime) = middleware(&data);
    add(&mut progs, &datetime.date(), &req_body)
}

#[post("/addtextdate/{date}")]
async fn addtextdate(path: web::Path<String>, data: web::Data<super::AppState>, req_body: String) -> impl Responder {
    let (mut progs, _) = middleware(&data);
    let date = NaiveDate::parse_from_str(path.into_inner().as_str(), "%Y-%m-%d").unwrap();
    add(&mut progs, &date, &req_body)
}
