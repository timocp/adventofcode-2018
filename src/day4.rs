use chrono::prelude::*;
use std::collections::HashMap;
use super::{Part,Part::*};
use time::Duration;

pub fn run(part: Part, input: &str) {
    let stats = collect_stats(&parse_input(input));
    let result = match part {
        One => strategy1(&stats),
        Two => strategy2(&stats)
    };
    println!("{}", result.0 * result.1);
}

enum Observation {
    BeginsShift(i32),
    WakesUp,
    FallsAsleep
}

struct Event {
    time: NaiveDateTime,
    observation: Observation
}

fn collect_stats(events: &Vec<Event>) -> HashMap<i32, (i32, [i32; 60])> {
    let mut stats = HashMap::new(); // (totalAsleep, sleepsPerMinute)
    let mut guard = 0;
    let mut slept_at: i32 = 0;
    for e in events {
        match e.observation {
            Observation::BeginsShift(g) => {
                guard = g;
            },
            Observation::WakesUp => {
                // from last event to here-1, guard was asleep
                let entry = stats.entry(guard).or_insert((0, [0; 60]));
                entry.0 += e.time.minute() as i32 - slept_at;
                for m in slept_at..(e.time.minute() as i32) {
                    entry.1[m as usize] += 1;
                }
            },
            Observation::FallsAsleep => {
                slept_at = e.time.minute() as i32;
            }
        }
    }
    stats
}

fn strategy1(stats: &HashMap<i32, (i32, [i32; 60])>) -> (i32, i32) {
    // guard with highest total minutes asleep
    let (sleepy, stat) = stats.iter().max_by_key(|(_k, v)| v.0).unwrap();
    let mut most_sleepy = 0;
    let mut most_sleepy_at = 0;
    for (min, times) in stat.1.iter().enumerate() {
        if *times > most_sleepy {
            most_sleepy = *times;
            most_sleepy_at = min as i32;
        }
    }

    (*sleepy, most_sleepy_at)
}

fn strategy2(stats: &HashMap<i32, (i32, [i32; 60])>) -> (i32, i32) {
    let mut most_sleepy: (i32, i32) = (0, 0); // guard#,  min
    let mut most_times = 0;
    for (guard, stat) in stats.iter() {
        for (min, times) in stat.1.iter().enumerate() {
            if *times > most_times {
                most_sleepy = (*guard, min as i32);
                most_times = *times;
            }
        }
    }
    most_sleepy
}

fn parse_input(input: &str) -> Vec<Event> {
    let mut events = vec![];
    for line in input.lines() {
        let mut dttm = NaiveDateTime::parse_from_str(
            line.chars().skip(1).take(16).collect::<String>().as_str(),
            "%Y-%m-%d %H:%M"
        ).unwrap();
        // assume things at 11pm are guards starting shifts. normalise them
        // to midnight.
        if dttm.hour() > 0 {
            dttm = dttm.with_hour(0).unwrap().with_minute(0).unwrap() + Duration::days(1);
        }
        let event = line.chars().skip(19).collect::<String>();
        if event.starts_with("Guard #") {
            let guard: i32 = event.chars().skip(7).take_while(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
            events.push(Event{time: dttm, observation: Observation::BeginsShift(guard)});
        } else if event == "falls asleep" {
            events.push(Event{time: dttm, observation: Observation::FallsAsleep});
        } else if event == "wakes up" {
            events.push(Event{time: dttm, observation: Observation::WakesUp});
        }
    }
    events.sort_by_key(|e| e.time);
    events
}

#[test]
fn test_run() {
    let test_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
    let stats = collect_stats(&parse_input(test_input));
    assert_eq!((10, 24), strategy1(&stats));
    assert_eq!((99, 45), strategy2(&stats));
}
