use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Timelike};
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
    let mut total_sleep_time: HashMap<String, u32> = HashMap::new();

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
                        *total_sleep_time.entry(current_guard.clone()).or_insert(0) += wake_minute - sleep_start_minute;
                    }
                    current_sleep_time = None;
                } else {
                    panic!("Error: Wake event without prior Sleep event.");
                }
            },
            Event::Sleep => {
                current_sleep_time = Some(item.0);
            },
            Event::Shift(guard_id) => {
                current_guard = guard_id;
            },
        }
    }

    // part 1
    let guard_most_asleep = total_sleep_time
        .iter()
        .max_by_key(|&(_, &total_time)| total_time)
        .map(|(guard_id, _)| guard_id.clone())
        .unwrap();

    let minutes = sleep_schedule.get(&guard_most_asleep).unwrap();
    let (most_slept_minute, &most_slept_count) = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, &count)| count)
        .unwrap();

    println!("Part 1:");
    println!(
        "Guard #{} slept the most (total {} minutes). Most frequent minute: {} ({} times)",
        guard_most_asleep, 
        total_sleep_time[&guard_most_asleep],
        most_slept_minute, 
        most_slept_count
    );

    println!(
        "Answer: {}",
        guard_most_asleep.parse::<usize>().unwrap() * most_slept_minute
    );

    // part 2
    let mut max_frequency = 0;
    let mut max_guard = String::new();
    let mut max_minute = 0;

    for (guard_id, minutes) in sleep_schedule.iter() {
        for (minute, &count) in minutes.iter().enumerate() {
            if count > max_frequency {
                max_frequency = count;
                max_guard = guard_id.clone();
                max_minute = minute;
            }
        }
    }

    println!("\nPart 2:");
    println!(
        "Guard #{} was most frequently asleep on minute {} ({} times)",
        max_guard, max_minute, max_frequency
    );

    println!(
        "Answer: {}",
        max_guard.parse::<usize>().unwrap() * max_minute
    );
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
