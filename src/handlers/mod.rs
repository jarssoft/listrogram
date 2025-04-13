use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use listagram::utils::progs::{progs_after, progs_by_time, current_dateime, progs_in_time};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike, Utc};

pub fn middleware(data: &web::Data<crate::AppState>) -> (std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>> , NaiveDateTime) {
    (data.progs.lock().unwrap(), current_dateime(data.timezone))
}

pub mod add;
pub mod get;
pub mod feed;