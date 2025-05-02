use std::ops::Range;
use chrono::{NaiveDateTime, NaiveTime, TimeDelta, Timelike};

pub const DAYPARTS: &[Range<u32>] = &[1..6, 6..12, 12..17, 17..21, 21..25];

pub fn  prog_in_dayparts(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, datetime: &NaiveDateTime, parts:&[Range<u32>]) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
    let movinghours = if datetime.hour() >= DAYPARTS[0].start {0} else {24};

    parts
        .iter()
        .map(|part| {
            
            let start = NaiveDateTime::new(
                datetime.date() - TimeDelta::hours(movinghours as i64),
                NaiveTime::from_hms_opt(part.start, 0, 0).unwrap()
            );        
            let end = start + TimeDelta::hours((part.end - part.start) as i64);
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

pub fn  progs_in_day_part(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, datetime: &NaiveDateTime) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
    prog_in_dayparts(progs, datetime, DAYPARTS)
}

pub fn  progs_in_current_part(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, datetime: &NaiveDateTime) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
        
    //jos kello on vähemmän kuin ensimmäinen jakso, lisätään 24
    let movinghours = if datetime.hour() >= DAYPARTS[0].start {0} else {24};   
    let currentpart = DAYPARTS.iter().find(|x: &&Range<u32>| x.contains(&(datetime.hour() + movinghours))).unwrap(); 

    prog_in_dayparts(progs, datetime, &[currentpart.clone()])
}