use std::ops::Range;
use chrono::{NaiveDateTime, NaiveTime, TimeDelta, Timelike};

pub const DAYPARTS: &[Range<u32>] = &[1..6, 6..12, 12..17, 17..21, 21..25];

pub fn  progs_in_current_part(progs: &std::sync::MutexGuard<'_, Vec<(NaiveDateTime, String)>>, datetime: &NaiveDateTime) -> 
    Vec<(Range<NaiveDateTime>, Vec<(NaiveDateTime, String)>)> {
        
    //jos kello on vähemmän kuin ensimmäinen jakso, lisätään 24
    let movinghours = if datetime.hour() >= DAYPARTS[0].start {0} else {24};
    
    let currentpart = DAYPARTS.iter().find(|x: &&Range<u32>| x.contains(&(datetime.hour() + movinghours))).unwrap(); 
    let startpart = DAYPARTS.first().unwrap();
    let endpart = DAYPARTS.last().unwrap();

    [currentpart]
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