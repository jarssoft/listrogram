use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use chrono::{NaiveDateTime, NaiveTime, TimeDelta, Timelike};
use std::ops::Range;
use crate::utils::progs::progs_in_time;
use super::middleware;

pub const DAYPARTS: &[Range<u32>] = &[1..6, 6..12, 12..17, 17..21, 21..25];

#[get("/feed")]
async fn feed(data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, datetime) = middleware(&data);    
    
    let hour = if datetime.hour() >= DAYPARTS[0].start {datetime.hour()} else {datetime.hour()+24};
    let currentpart = DAYPARTS.iter().find(|x| x.contains(&hour)).unwrap(); 

    let start = NaiveDateTime::new(datetime.date(),NaiveTime::from_hms_opt(currentpart.start, 0, 0).unwrap());
    let response = progs_in_time(
            &progs, 
            start..start + TimeDelta::hours((currentpart.end - currentpart.start) as i64)
        );    
    
    let start_of_part = NaiveTime::from_hms_opt(currentpart.start, 0, 0).unwrap();
    let pubtime = datetime.format("%Y-%m-%dT").to_string()+&start_of_part.format("%H:%M:%SZ").to_string();

    let haiku = format!("\
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <feed xmlns=\"http://www.w3.org/2005/Atom\">
        <id>urn:uuid:553e4df4-6e6c-11da-be05-000461723b33</id>
        <title>Tv-ohjelmat</title>
        <updated>{pubtime}</updated>
        <link rel=\"self\" href=\"http://example.org/blog/feed.atom\"/>
        <entry>
            <id>{pubtime}</id>
            <title>Ohjelmat: {} {}</title>
            <updated>{pubtime}</updated>
            <content type='html'>{}</content>
            <link href=\"http://example.org/blog/{}\"/>
        </entry>
        </feed>",        
        datetime.date().to_string(),
        format!("klo {}–{}",currentpart.start,currentpart.end),
        format!("{}", response
                .iter()
                .map(|p|format!(
                    "\n\t\t\t&lt;p&gt;{} {}&lt;/p&gt;", 
                    p.0.format("%H:%M"), 
                    p.1.replace("&", "&amp;")))
                .collect::<Vec<String>>()
                .join("")),
        pubtime);

    HttpResponse::Ok().content_type(header::ContentType::xml()).body(haiku)
}