use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike, Utc};
use std::ops::Range;

pub fn current_dateime(timezoneopt: Option<i32>) -> NaiveDateTime {
    match timezoneopt {
        Some(timezone) => {
            let tz_offset = FixedOffset::east_opt(timezone).unwrap();
            tz_offset.from_utc_datetime(&Utc::now().naive_utc()).naive_local()}
        None => Local::now().naive_local()
    }
}

pub fn current_time(timezoneopt: Option<i32>) -> NaiveTime {
    current_dateime(timezoneopt).time()
}

pub fn progs_by_time(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, time:NaiveTime) -> Vec<(NaiveTime, String)>{
    let now2 = progs
        .iter()
        .reduce(|x,y|{
            if y.0 < time {y} else {x}
        }); 

    match now2 {
        Some(prog) => vec![(*prog).clone()],
        None => vec![]        
    }
}

pub fn progs_after(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, time:NaiveTime, max:usize) -> Vec<(NaiveTime, String)>{   
    progs
        .iter()
        .filter(|x| x.0 > time)
        .take(max)
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>()
}

pub fn progs_in_time(progs: &std::sync::MutexGuard<'_, Vec<(NaiveTime, String)>>, time: Range<NaiveTime>) -> Vec<(NaiveTime, String)>{   
    progs
        .iter()
        .filter(|x| x.0 >= time.start && x.0 <= time.end )
        .cloned()
        .collect::<Vec<(NaiveTime, String)>>()
}


