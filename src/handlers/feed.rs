use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike, Utc};
use std::ops::Range;
use crate::utils::progs::{middleware, progs_after, progs_by_time, current_dateime, progs_in_time};

pub const DAYPARTS: &[(&str, Range<u32>)] = &[
    ("Yö", 0..6), 
    ("Aamu", 6..12), 
    ("Päivä", 12..17), 
    ("Ilta", 17..21), 
    ("Myöhäis-ilta", 21..0)];

#[get("/rss")]
async fn feed(data: web::Data<crate::AppState>) -> impl Responder  {
    let (progs, time) = middleware(&data);    
    let currentpart = DAYPARTS.iter().find(|x| x.1.contains(&time.hour())).unwrap();   
    let response = progs_in_time(
            &progs, 
            NaiveTime::from_hms_opt(currentpart.1.start, 0, 0).unwrap()..
            NaiveTime::from_hms_opt(currentpart.1.end, 0, 0).unwrap()
        );    
    
    let start_of_part = NaiveTime::from_hms_opt(currentpart.1.start, 0, 0).unwrap();
    let pubtime = time.format("%Y-%m-%dT").to_string()+&start_of_part.format("%H:%M:%SZ").to_string();

    let haiku = format!("\
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <feed xmlns=\"http://www.w3.org/2005/Atom\">
        <id>urn:uuid:553e4df4-6e6c-11da-be05-000461723b33</id>
        <title>Tv-ohjelmat</title>
        <updated>{}</updated>
        <link rel=\"self\" href=\"http://example.org/blog/feed.atom\"/>
        <entry>
            <id>{}</id>
            <title>Ohjelmat:{} {}</title>
            <updated>{}</updated>
            <content type='html'>{}</content>
            <link href=\"http://example.org/blog/{}\"/>
        </entry>
        </feed>",
        pubtime,
        pubtime, 
        time.date().to_string(),
        currentpart.0,      
        pubtime,   
        format!("{}", response.iter().map(|p|format!("&lt;p&gt;{} {}&lt;/p&gt;", p.0, p.1.replace("&", "&amp;"))).collect::<Vec<String>>().join("")),
        pubtime);

    HttpResponse::Ok().content_type(header::ContentType::xml()).body(haiku)
}