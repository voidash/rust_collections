extern crate chrono;

use chrono::{Date, Duration, TimeZone, Utc, Datelike};
use std::error::Error;
use std::fs::File;
use std::io::{Write,};

use std::mem;

struct DateRange(Date<Utc> , Date<Utc>);

impl Iterator for DateRange {
    type Item = Date<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        }else {
            None
        }
    }
}

fn create_and_write_file(name: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut output =  File::create(name).unwrap();
    write!(output, "{}", content);
    Ok(())
}

fn main() {
    let months = ["january", "february", "march", "april","may", "june", "july", "august", "september", "october", "november", "december" ];
    let dt1 = Utc.ymd(2022,1,1);
    let dt2 = Utc.ymd(2023,1,1);

    for dt in DateRange(dt1, dt2) {
        let filename = format!("./2022/{}-{}.md", months[dt.month0() as usize], dt.day());
        let content = format!("---\ntitle:\"{}\"\ndate:{}\ndraft: false\n---", filename, dt);

        create_and_write_file(filename.as_str(), content.as_str());
    }


    
}
