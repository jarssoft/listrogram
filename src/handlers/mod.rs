use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::utils::progs::{progs_after, progs_by_time, current_datetime, progs_in_time};
use chrono::NaiveDateTime;
use serde::Serialize;
use std::sync::Mutex;
use crate::utils::progs::TimePolicy;

#[derive(Serialize)]
pub struct AppState {
    pub progs: Mutex<Vec::<(NaiveDateTime, String)>>, // <- Mutex is necessary to mutate safely across threads
    pub timeformat: TimePolicy,
}

// Note: web::Data created _outside_ HttpServer::new closure
pub fn build_appdata(timeformat: TimePolicy) -> AppState{
    AppState {
        progs: Mutex::new(Vec::new()),
        timeformat
    }
}

pub fn middleware(data: &web::Data<AppState>) -> (std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>> , NaiveDateTime) {
    (data.progs.lock().unwrap(), current_datetime(&data.timeformat))
}

pub mod add;
pub mod get;
pub mod feed;

/*
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize,Debug)] 
*/