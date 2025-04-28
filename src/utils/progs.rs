use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Timelike};
use std::ops::Range;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub enum TimePolicy {
    ///Use localtime
    Naive(),
    ///Specify a fixed timezone
    Timezone(i32),
    ///Use fixed time for testing
    FixedTime(i8, i8),
}

pub fn current_datetime(timepolicy: &TimePolicy) -> NaiveDateTime {
    match timepolicy {
        TimePolicy::Naive() => {
            let utc = Utc::now();
            //let local = Local::now();
            let converted: DateTime<Local> = DateTime::from(utc);
            print!("Local is {}",converted.naive_local());
            converted.naive_local()
            //Local::now().naive_local()
        }
        TimePolicy::Timezone(timezone) => {
            let tz_offset = FixedOffset::east_opt(*timezone).unwrap();
            tz_offset.from_utc_datetime(&Utc::now().naive_utc()).naive_local()
        }
        TimePolicy::FixedTime(hour, min) => {
            let date  = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
            let time = NaiveTime::from_hms_opt((*hour).try_into().unwrap(), (*min).try_into().unwrap(), 0).unwrap();
            NaiveDateTime::new(date, time)
            
        }
    }
}

pub fn current_time(timeformat: &TimePolicy) -> NaiveTime {
    current_datetime(timeformat).time()
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


