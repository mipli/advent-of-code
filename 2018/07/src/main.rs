#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap};
use std::str::FromStr;
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

    let mut order = String::new();
    while !steps.is_empty() {
        let ready = get_next_step(&steps);
        perform_step(ready.unwrap(), &mut steps);
        order.push(ready.unwrap());
    }
    println!("Order: {}", order); // FMOXCDGJRAUIHKNYZTESWLPBQV


    Ok(())
}

fn perform_step(name: char, steps: &mut HashMap<char, Step>) {
    steps.remove(&name);
    steps.iter_mut().for_each(|(_, v)| {
        v.parents.retain(|&c| c != name);
        v.children.retain(|&c| c != name);
    });
}

fn get_next_step(steps: &HashMap<char, Step>) -> Option<char> {
    let mut ready = steps.iter().filter_map(|(_, v)| {
        if v.parents.is_empty() {
            Some(v.name)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    ready.sort();
    ready.reverse();
    ready.pop()
}

#[derive(Debug, Clone)]
struct Step {
    name: char,
    parents: Vec<char>,
    children: Vec<char>
}

impl Step {
    fn new(name: char) -> Step {
        Step {
            name,
            parents: vec![],
            children: vec![]
        }
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
