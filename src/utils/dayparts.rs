use std::ops::Range;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Timelike};

pub const DAYPARTS: &[Range<u32>] = &[1..6, 6..12, 12..17, 17..21, 21..25];

pub fn to_date(datetime: &NaiveDateTime) -> NaiveDate {
    let movingday = ((datetime.hour() + 24 - DAYPARTS[0].start) as f64 / 24.0).floor() as i64 - 1;
    datetime.date() + TimeDelta::days(movingday)
}

pub fn  prog_in_dayparts(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, date: &NaiveDate, parts:&[Range<u32>]) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {

    parts
        .iter()
        .map(|part| {            
            let midnight = NaiveDateTime::new(
                *date, NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            );        
            let start = midnight + TimeDelta::hours(part.start as i64);
            let end = midnight + TimeDelta::hours(part.end as i64);
            let range = start..end; 

            (range.clone(), progs
                .iter()            
                .filter(|p| range.contains(&p.0))             
                .cloned()            
                .collect::<Vec<(NaiveDateTime, String)>>()
            )
        })
        .collect::<Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)>>()
}

pub fn progs_in_day_part(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, date: &NaiveDate) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
    prog_in_dayparts(progs, date, DAYPARTS)
}

pub fn progs_in_current_part(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, datetime: &NaiveDateTime) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
        
    //jos kello on vähemmän kuin ensimmäinen jakso, lisätään 24
    let hour = (datetime.hour() + 24 - DAYPARTS[0].start) % 24 + DAYPARTS[0].start;
    let currentpart = DAYPARTS.iter().find(|x: &&Range<u32>| x.contains(&hour)).unwrap(); 

    prog_in_dayparts(progs, &to_date(datetime), &[currentpart.clone()])
}