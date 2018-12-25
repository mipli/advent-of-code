#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap};
use std::io::{self, prelude::*};

use regex::Regex;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let mut steps: HashMap<char, Step> = HashMap::default();

    for line in input.lines() {
        let (req, step) = parse_line(line)?;
        steps.entry(step).or_insert(Step::new(step)).parents.push(req);
        steps.entry(req).or_insert(Step::new(req)).children.push(step);
    }

    let mut time = 0;
    let mut workers = vec![Worker {
        id: 1,
        ready_at: 0,
        working_on: None
    }, Worker {
        id: 2,
        ready_at: 0,
        working_on: None
    }, Worker {
        id: 3,
        ready_at: 0,
        working_on: None
    }, Worker {
        id: 4,
        ready_at: 0,
        working_on: None
    }, Worker {
        id: 5,
        ready_at: 0,
        working_on: None
    }];
    let mut done = vec![];
    while !steps.is_empty() {
        println!("Time: {}", time);
        let mut done_something = false;
        match get_available_step(&mut steps) {
            Some(available_step) => {
                match get_available_worker(&mut workers) {
                    Some(mut worker) => {
                        println!("Time: {}, Worker: {:?}", time, worker);
                        println!("Starting work on: {}", available_step.name);
                        worker.working_on = Some(available_step.name);
                        worker.ready_at =  time + available_step.time();
                        workers.push(worker);
                        done_something = true;
                    },
                    None => {
                        steps.insert(available_step.name, available_step);
                    }
                }
            },
            None => {
            }
        }

        if !done_something {
            if let Some((new_time, step)) = perform_work(&mut workers) {
                done.push(step);
                perform_step(step, &mut steps);
                time = new_time;
            }
        }
    }
    while let Some((new_time, step)) = perform_work(&mut workers) {
        done.push(step);
        time = new_time;
    } 

    println!("Steps: {:?}", done);
    println!("Time: {}", time);


    Ok(())
}

fn perform_work(workers: &mut Vec<Worker>) -> Option<(i32, char)> {
    workers.sort_by_key(|w| w.ready_at);
    println!("Workers: {:?}", workers);
    let index = workers.iter().position(|w| w.working_on.is_some())?;
    let mut worker = workers.remove(index);
    if worker.working_on.is_some() {
        println!("Performed work: {:?}", worker);
        let work = (worker.ready_at, worker.working_on.expect("Worker has no work to perform!"));
        worker.working_on = None;
        workers.push(worker);
        Some(work)
    } else {
        println!("no work to perform");
        workers.push(worker);
        None
    }
}

fn get_available_step(steps: &mut HashMap<char, Step>) -> Option<Step> {
    let (&c, _) = steps.iter().find(|(_, s)| s.parents.is_empty())?;
    steps.remove(&c)
}

fn get_available_worker(workers: &mut Vec<Worker>) -> Option<Worker> {
    let index = workers.iter().position(|w| w.working_on.is_none())?;
    Some(workers.remove(index))
}

fn perform_step(name: char, steps: &mut HashMap<char, Step>) {
    steps.remove(&name);
    steps.iter_mut().for_each(|(_, v)| {
        v.parents.retain(|&c| c != name);
        v.children.retain(|&c| c != name);
    });
}

#[derive(Debug, Clone)]
struct Worker {
    id: usize,
    ready_at: i32,
    working_on: Option<char>
}

#[derive(Debug, Clone)]
struct Step {
    name: char,
    parents: Vec<char>,
    children: Vec<char>,
}

impl Step {
    fn new(name: char) -> Step {
        Step {
            name,
            parents: vec![],
            children: vec![]
        }
    }

    fn time(&self) -> i32 {
       (self.name as i32) - 64 + 60
    }
}

fn parse_line(line: &str) -> Result<(char, char), Error> {
    lazy_static! {
        static ref LINE_PARSER_RE: Regex = Regex::new(r"^Step (?P<req>.) must be finished before step (?P<step>.) can begin.$").unwrap();
    }

    match LINE_PARSER_RE.captures(line) {
        None => return Err(Box::<std::error::Error>::from("Could not parse coordinate")),
        Some(captures) => {
            let a: char = captures["req"].parse()?;
            let b: char = captures["step"].parse()?;
            Ok((a, b))
        }
    }

}
