use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

fn format_error(msg:&str, oline: Option<(usize, &str)>) -> Result<Vec::<(NaiveDateTime, String)>, String> {
    if let Some(line) = oline {
        Err(format!("Error in line {} ('{}'): {}", line.0+1, line.1, msg))
    }else{
        Err(format!("Error: {}", msg))
    }
}

/// Parsii ohjelmatiedot tekstistä.   
///   
/// %Y-%m-%d        (Päiväys, voi jättää pois, jolloin käytetään aiempaa päiväystä.)  
/// %H:%M           (Alkamisaika)  
/// Ohjelman nimi  
pub fn parse_from_text(mut date: NaiveDate, req_body: &String) -> Result<Vec::<(NaiveDateTime, String)>, String> {
    let mut progs: Vec::<(NaiveDateTime, String)> = Vec::new();
    let mut lines = req_body.lines().enumerate();

    while let Some(line1) = lines.next() {

        if line1.1.is_empty() {
            continue;
        }

        if line1.1.len()==10 {
            let date_result = NaiveDate::parse_from_str(line1.1, "%Y-%m-%d");
            if date_result.is_err(){
                return format_error("Expected time or date string (%H:%M).", Some(line1));
            }
            date=date_result.unwrap();
            continue;
        }

        let time = NaiveTime::parse_from_str(line1.1, "%0H:%0M");
        
        if time.is_err() {
            return format_error("Expected time or date string (%H:%M).", Some(line1));
        }
        let datetime = NaiveDateTime::new(date, time.unwrap());

        if progs.last().is_some() && datetime < progs.last().unwrap().0 {            
            return format_error("Added time was before last time.", Some(line1));
        }

        let nextline = lines.next();
        if nextline.is_none(){
            return format_error("Found end of file, expected program title.", nextline);
        }

        let title = nextline.unwrap().1;           
        if title.is_empty() {
            return format_error("Program title must be longer than 0.", nextline);
        }

        progs.push((datetime, title.to_string().clone()));
    }

    if progs.is_empty() {
        return format_error("No programs added.", None);
    }

    Ok(progs)   

}
