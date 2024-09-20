use chrono::prelude::*;

enum Event {
    Sleep,
    Wake,
    Shift(String),
}

fn main() {
    let input: Vec<&str> = include_str!("../../data/day4_short.txt").lines().collect();

    let mut chronology: Vec<NaiveDateTime> = vec![];

    for item in input {
        let query: Vec<&str> = item.split_whitespace().map(|part| part.trim()).collect();
        let date = &query[0][1..];
        let hour = &query[1].strip_suffix("]").unwrap();
        let event = match &query[2] {
            n if n.starts_with("falls") => Event::Sleep, 
            n if n.starts_with("wakes") => Event::Wake, 
            n if n.starts_with("Guard") => {
                let id = &query[3][1..];
                Event::Shift(id.to_owned())
            },
            _ => unreachable!("nice input guy"),
        };

        let time = parse_datetime(date, hour);
        chronology.push(time.unwrap());
    }

    print_chronology(&chronology);
    chronology.sort();
    print_chronology(&chronology);
}

fn parse_datetime(date_str: &str, time_str: &str) -> Option<NaiveDateTime> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    
    let time = NaiveTime::parse_from_str(&format!("{}:00", time_str), "%H:%M:%S").ok()?;
    
    Some(date.and_time(time))
}

fn print_chronology(chrono: &Vec<NaiveDateTime>) {
    for item in chrono {
        println!("{item:?}");
    }
    println!();
}
