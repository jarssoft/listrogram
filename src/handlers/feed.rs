use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use chrono::{NaiveDateTime, NaiveTime, TimeDelta, Timelike};

use crate::utils::dayparts::progs_in_current_part;

use super::middleware;

#[get("/feed")]
async fn feed(data: web::Data<super::AppState>) -> impl Responder  {
    let (progs, datetime) = middleware(&data);    
 
    let (range, progs) 
            = progs_in_current_part(&progs, &datetime).get(0).unwrap().clone();    
    
    let pubtime = range.start.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let xml = format!("\
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <feed xmlns=\"http://www.w3.org/2005/Atom\">
        <id >urn:uuid:553e4df4-6e6c-11da-be05-000461723b33</id>
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
        range.start.date().to_string(),
        format!("klo {:0>2}–{:0>2}", range.start.hour(), range.end.hour()),
        format!("{}", progs
                .iter()
                .map(|p|format!(
                    "\n\t\t\t&lt;p&gt;{} {}&lt;/p&gt;", 
                    p.0.format("%H:%M"), 
                    p.1.replace("&", "&amp;")))
                .collect::<Vec<String>>()
                .join("")),
        pubtime);

    HttpResponse::Ok().content_type(header::ContentType::xml()).body(xml)
}
