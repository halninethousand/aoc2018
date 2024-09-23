use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration, Timelike};
use std::collections::HashMap;

#[derive(Debug)]
enum Event {
    Sleep,
    Wake,
    Shift(String),
}

fn main() {
    let input: Vec<&str> = include_str!("../../data/day4.txt").lines().collect();

    let mut chronology = init_and_sort(input);    

    chronology.sort_by_key(|&(datetime, _)| datetime);
    print_chronology(&chronology);
    process_chronology(chronology);
}

fn process_chronology(chronology: Vec<(NaiveDateTime, Event)>) {
    let mut current_guard: String = String::new();
    let mut current_sleep_time: Option<NaiveDateTime> = None;
    let mut sleep_schedule: HashMap<String, [u32; 60]> = HashMap::new();

    for item in chronology {
        match item.1 {
            Event::Wake => {
                if let Some(sleep_time) = current_sleep_time {
                    if !current_guard.is_empty() {
                        let sleep_start_minute = sleep_time.minute();
                        let wake_minute = item.0.minute();

                        let minute_count = sleep_schedule.entry(current_guard.clone()).or_insert([0; 60]);
                        for minute in sleep_start_minute..wake_minute {
                            minute_count[minute as usize] += 1;
                        }
                    }
                    current_sleep_time = None;
                } else {
                    println!("Error: Wake event without prior Sleep event.");
                }
            },
            Event::Sleep => {
                current_sleep_time = Some(item.0);
            },
            Event::Shift(guard_id) => {
                current_guard = guard_id;
            },
            _ => println!("Don't care"),
        }
    }

    // Step 2: Find the guard who slept the most on a single minute
    let mut guard_most_asleep = String::new();
    let mut most_slept_minute = 0;
    let mut most_slept_count = 0;

    for (guard_id, minutes) in sleep_schedule.iter() {
        for (minute, &count) in minutes.iter().enumerate() {
            if count > most_slept_count {
                most_slept_minute = minute;
                most_slept_count = count;
                guard_most_asleep = guard_id.clone();
            }
        }
    }

    if !guard_most_asleep.is_empty() {
        println!(
            "Guard #{} slept the most on minute {} ({} times) {}",
            guard_most_asleep, most_slept_minute, most_slept_count, guard_most_asleep.parse::<usize>().unwrap() * most_slept_minute
        );
    }
}
fn init_and_sort(input: Vec<&str>) -> Vec<(NaiveDateTime, Event)> {
    let mut chronology: Vec<(NaiveDateTime, Event)> = vec![];

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

        println!("date: {}, hour: {}, event: {:?}", date, hour, event);

        let time = parse_datetime(date, hour);
        chronology.push((time.unwrap(), event));
    }
    
    chronology

}

fn parse_datetime(date_str: &str, time_str: &str) -> Option<NaiveDateTime> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    
    let time = NaiveTime::parse_from_str(&format!("{}:00", time_str), "%H:%M:%S").ok()?;
    
    Some(date.and_time(time))
}

fn print_chronology(chronology: &Vec<(NaiveDateTime, Event)>) {
    for item in chronology {
        println!("{item:?}");
    }
    println!();
}
