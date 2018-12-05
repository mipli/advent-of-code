#[macro_use]
extern crate lazy_static;

use itertools;
use std::str::FromStr;
use std::io::{self, prelude::*};
use regex::Regex;
use std::collections::{HashMap};

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let log_lines = parse_input(&input)?;

    let guard_log = GuardLog::build(log_lines);

    println!("Most sleepy guard checksum: {}", most_sleepy_guard_solution(&guard_log)); //99759
    println!("Most predictable guard checksum: {}", most_predictable_guard_solution(&guard_log)); //97884

    Ok(())
}

fn most_predictable_guard_solution(guard_log: &GuardLog) -> u32 {
    struct FrequentGuard {
        id: u32,
        time: u32,
        frequency: u32
    }
    let fg = guard_log.minutes.keys().fold(FrequentGuard {id: 0, time: 0, frequency: 0}, |acc, &id| {
        let (time, frequency) = guard_log.get_prefered_sleep_minute(id).expect("Tried to get prefered sleep time for unknown guard");

        if frequency > acc.frequency {
            FrequentGuard {
                id,
                time,
                frequency
            }
        } else {
            acc
        }
    });
    fg.id * fg.time
}

fn most_sleepy_guard_solution(guard_log: &GuardLog) -> u32 {
    let (id, _) = guard_log.log.keys().fold((0, 0), |mut acc, &id| {
        let time = guard_log.get_total_sleep_time(id).expect("Tried to get sleep time for unknown guard");
        if time > acc.1 {
            acc = (id, time);
        }
        acc
    });

    let (time, _) = guard_log.get_prefered_sleep_minute(id).expect("Tried to get prefered sleep time for unknown guard");
    id * time
}

fn parse_input(input: &str) -> Result<Vec<LogLine>, Error> {
    let mut log_lines = vec![];
    for line in input.lines() {
        let log: LogLine = line.parse().or_else(|err| {
            Err(Box::<std::error::Error>::from(format!("Failed to parse '{}': {}", line, err)))
        })?;
        log_lines.push(log);
    }
    log_lines.sort_by(|a, b| a.time.cmp(&b.time));
    Ok(log_lines)
}

struct GuardLog {
    log: HashMap<u32, Vec<LogLine>>,
    minutes: HashMap<u32, [u32; 60]>
}

impl GuardLog {
    fn build(log: Vec<LogLine>) -> Self {
        let mut lines: HashMap<u32, Vec<LogLine>> = HashMap::new();
        let mut minutes: HashMap<u32, [u32; 60]> = HashMap::new();

        let mut guard_id = None;
        for line in log.into_iter() {
            guard_id = match line.get_guard_id() {
                Some(id) => Some(id),
                None => guard_id
            };
            match guard_id {
                Some(id) => {
                    (*lines.entry(id).or_default()).push(line.clone());
                },
                None => {}
            }

        }

        lines.iter().for_each(|(&id, lines)| {
            let mut guard_minutes = [0; 60];
            let mut sleep_time = 0;
            for log in lines {
                if log.is_fall_asleep() {
                    sleep_time = log.minute;
                }
                if log.is_wakes_up() {
                    for i in sleep_time..log.minute {
                        guard_minutes[i as usize] += 1;
                    }
                }
            }
            minutes.insert(id, guard_minutes);
        });

        GuardLog {
            log: lines,
            minutes
        }
    }

    fn get_prefered_sleep_minute(&self, id: u32) -> Option<(u32, u32)> {
        let minutes = self.minutes.get(&id)?;
        let (id, freq) = minutes.iter().enumerate().fold((0, 0), |acc, (id, &freq)| {
            if acc.1 < freq {
                (id, freq)
            } else {
                acc
            }
        });
        Some((id as u32, freq))
    }

    fn get_total_sleep_time(&self, id: u32) -> Option<u64> {
        let logs = self.log.get(&id)?;
        let mut awake = true;
        let mut total = 0;
        let mut last_time = 0;
        for log in logs {
            if awake {
                if log.is_fall_asleep() {
                    awake = false;
                }
            } else {
                if log.is_wakes_up() {
                    awake = true;
                    total += log.time - last_time;
                }
            }
            last_time = log.time;
        }
        Some(total)
    }
}

#[derive(Debug, Clone)]
struct LogLine {
    time: u64,
    minute: u32,
    log: String,
    original: String
}

impl LogLine {
    fn get_guard_id(&self) -> Option<u32> {
        lazy_static! {
            static ref GUARD_ID_RE: Regex = Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
        }
        match GUARD_ID_RE.captures(&self.log) {
            None => None,
            Some(captures) => {
                captures["id"].parse().ok()
            }
        }
    }

    fn is_fall_asleep(&self) -> bool {
        self.log == "falls asleep"
    }

    fn is_wakes_up(&self) -> bool {
        self.log == "wakes up"
    }
}

impl FromStr for LogLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<LogLine, Error> {
        lazy_static! {
            static ref LOG_LINE_RE: Regex = Regex::new(r"\[(?P<Y>\d{4})-(?P<M>\d{2})-(?P<D>\d{2}) (?P<hour>\d{2}):(?P<min>\d{2})] (?P<log>.*)").unwrap();
        }

        let cap = match LOG_LINE_RE.captures(s) {
            None => return Err(Box::<std::error::Error>::from("Could not parse log line")),
            Some(captures) => captures,
        };

        let y: u64 = cap["Y"].parse()?;
        let m: u64 = cap["M"].parse()?;
        let d: u64 = cap["D"].parse()?;
        let hour: u64 = cap["hour"].parse()?;
        let min: u64 = cap["min"].parse()?;

        Ok(LogLine {
            time: min + (hour * 60) + (d * 24 * 60) + (m * 24 * 60 * 31) + (y * 24 * 60 * 31 * 365),
            minute: min as u32,
            log: cap["log"].to_string(),
            original: s.to_string()
        })
    }
}
