#[macro_use]
extern crate lazy_static;

use std::io::{self, prelude::*};
use std::str::FromStr;
use std::fmt;

use regex::Regex;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let mut lines = input.lines().collect::<Vec<_>>();

    let state: State = lines.remove(0).parse()?;

    let mut patterns: Vec<Pattern> = vec![];

    for line in lines {
        if !line.is_empty() {
            let pattern: Pattern = line.parse()?;
            patterns.push(pattern);
        }
    }

    println!("20: {}", run(state.clone(), 20, &patterns));
    println!("50'000: {}", run(state.clone(), 50_000, &patterns));
    println!("500'000: {}", run(state.clone(), 500_000, &patterns));
    // There is a pattern to the answers here;
    // answer is of form 43x3414, where x is n zeroes, where n is generation_number / 5_000
    // so the answer is 4300000003414


    Ok(())
}

fn run(mut state: State, generations: usize, patterns: &[Pattern]) -> i32 {
    for _ in 0..generations {
        state = state.next_generation(patterns);
    }
    state.count()
}

#[derive(Debug, Copy, Clone)]
struct Pattern {
    pattern: [Pot; 5],
    result: Pot
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(line: &str) -> Result<Pattern, Error> {
        lazy_static! {
            static ref PATTERN_RE: Regex = Regex::new(r"^(?P<pattern>[.#]{5}) => (?P<result>[.#])").unwrap();
        }

        match PATTERN_RE.captures(line) {
            Some(captures) => {
                let pattern: String = captures["pattern"].parse()?;
                let result: Pot = captures["result"].parse()?;
                let mut pat = [Pot::Empty; 5];
                pattern.chars().enumerate().for_each(|(i, c)| {
                    pat[i] = c.to_string().parse().unwrap();
                });
                Ok(Pattern {
                    pattern: pat,
                    result
                })
            },
            None => return Err(Box::<std::error::Error>::from("Could not parse pattern information"))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pot {
    Flower,
    Empty
}

impl FromStr for Pot {
    type Err = Error;

    fn from_str(line: &str) -> Result<Pot, Error> {
        match line {
            "." => Ok(Pot::Empty),
            "#" => Ok(Pot::Flower),
            _ => Err(Box::<std::error::Error>::from("Could not parse plant state"))
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    pots: Vec<Pot>,
    generation: usize,
    left: i32
}

impl State {
    fn new() -> State {
        State {
            pots: vec![],
            generation: 0,
            left: 0
        }
    }

    fn add_pot(&mut self, pot: Pot) {
        self.pots.push(pot);
    }

    fn get(&self, pos: i32) -> Pot {
        if pos < 0 {
            Pot::Empty
        } else {
            match self.pots.get(pos as usize) {
                Some(pot) => *pot,
                None => Pot::Empty
            }
        }
    }

    fn get_five(&self, pos: i32) -> [Pot; 5] {
        let mut pots = [Pot::Empty; 5];
        ((pos - 2)..=(pos + 2)).enumerate().for_each(|(i, p)| {
            pots[i] = self.get(p);
        });
        pots
    }

    fn count(&self) -> i32 {
        self.pots.iter().enumerate().filter_map(|(i, p)| {
            match p {
                Pot::Flower => {
                    Some(i as i32 + self.left)
                },
                Pot::Empty => None
            }
        }).sum()
    }

    fn next_generation(self, patterns: &[Pattern]) -> State {
        let mut state = State::new();
        state.generation = self.generation + 1;
        state.left = self.left - 2;
        let mut id = -2;
        loop {
            let pots = self.get_five(id);
            let mut matched = false;
            for pattern in patterns {
                if pots == pattern.pattern {
                    state.pots.push(pattern.result);
                    matched = true;
                    break;
                }
            }
            if !matched {
                state.pots.push(Pot::Empty);
            }
            id += 1;
            if id > self.pots.len() as i32 + 3 {
                break;
            }
        }
        state.trim_left();
        state.trim_right();
        state
    }

    fn trim_left(&mut self) {
        let pos = self.pots.iter().position(|p| *p == Pot::Flower);
        if let Some(pos) = pos {
            if pos > 3 {
                for _ in 0..(pos - 3) {
                    let _ = self.pots.remove(0);
                    self.left += 1;
                }
            } else if pos < 3 {
                for _ in 0..(3 - pos) {
                    let _ = self.pots.insert(0, Pot::Empty);
                    self.left -= 1;
                }
            }
        }
    }

    fn trim_right(&mut self) {
        let pos = self.pots.iter().rev().position(|p| *p == Pot::Flower);
        if let Some(pos) = pos {
            if pos > 3 {
                for _ in 0..(pos - 3) {
                    let _ = self.pots.pop();
                }
            } else if pos < 3 {
                for _ in 0..(3 - pos) {
                    self.pots.push(Pot::Empty);
                }
            }
        }
    }
}

impl FromStr for State {
    type Err = Error;

    fn from_str(line: &str) -> Result<State, Error> {
        lazy_static! {
            static ref STATE_RE: Regex = Regex::new(r"^initial state: (?P<state>[.#]+).*").unwrap();
        }

        match STATE_RE.captures(line) {
            Some(captures) => {
                let init: String = captures["state"].parse()?;
                let mut state = State::new();
                init.bytes().for_each(|i| {
                    match i {
                        b'.' => {
                            state.add_pot(Pot::Empty);
                        },
                        b'#' => {
                            state.add_pot(Pot::Flower);
                        },
                        _ => unreachable!("This character should not be here")
                    }
                });
                state.trim_left();
                state.trim_right();
                Ok(state)

            },
            None => return Err(Box::<std::error::Error>::from("Could not parse initial state information"))
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = format!("{}: ", self.generation);
        self.pots.iter().for_each(|pot| {
            match pot {
                Pot::Empty => buf.push('.'),
                Pot::Flower => buf.push('#')
            }
        });
        write!(f, "{}", buf)

    }
}
