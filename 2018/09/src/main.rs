#[macro_use]
extern crate lazy_static;

use std::io::{self, prelude::*};

use regex::Regex;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    for line in input.lines() {
        let (players, max_marble) = parse_line(line)?;
        let highscore = play(players, max_marble);
        println!("Highscore ({}, {}): {}", players, max_marble, highscore);

        let max_marble = max_marble * 100;
        let highscore = play(players, max_marble);
        println!("Highscore part 2 ({}, {}): {}", players, max_marble, highscore);
    }


    Ok(())
}

fn play(players: u32, max_marble: u32) -> u32 {
    let mut circle = Circle::new();
    let mut scores = vec![0; players as usize];

    (0..players).cycle().zip(1..=max_marble).for_each(|(p, m)| {
        match circle.play(m) {
            Some(score) => {
                scores[p as usize] += score;
            },
            None => {}
        }
    });

    *scores.iter().max().unwrap()
}

fn parse_line(line: &str) -> Result<(u32, u32), Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^(?P<players>\d+) players; last marble is worth (?P<max>\d+) points.*").unwrap();
    }

    match LINE_RE.captures(line) {
        Some(captures) => {
            let players: u32 = captures["players"].parse()?;
            let max: u32 = captures["max"].parse()?;
            Ok((players, max))
        },
        None => return Err(Box::<std::error::Error>::from("Could not parse play information"))
    }
}

type MarbleId = usize;
type MarbleValue = u32;

struct Circle {
    current: MarbleId,
    marbles: Vec<Marble>
}

impl Circle {
    fn new() -> Circle {
        let marble = Marble::create_empty(0);
        Circle {
            current: 0,
            marbles: vec![marble]
        }
    }

    fn play(&mut self, value: MarbleValue) -> Option<u32> {
        let marble = self.add_marble(value);
        if value % 23 == 0 {
            let remove_id = self.counter_clockwise(7);
            self.remove(remove_id);
            self.current = self.counter_clockwise(6);
            Some(value + self.marbles[remove_id].value)
        } else {
            let pos = self.clockwise(1);
            self.insert_after(pos, marble);
            self.current = marble;
            None
        }
    }

    fn remove(&mut self, index: MarbleId) {
        let (prev, next) = (self.marbles[index].prev, self.marbles[index].next);
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
    }

    fn insert_after(&mut self, index: MarbleId, marble: MarbleId) {
        let old = self.marbles[index].next;
        self.marbles[index].next = marble;
        self.marbles[old].prev = marble;
        self.marbles[marble].prev = index;
        self.marbles[marble].next = old;
    }

    fn add_marble(&mut self, value: MarbleValue) -> MarbleId {
        let id = self.marbles.len();
        self.marbles.push(Marble::create_empty(value));
        id
    }

    fn clockwise(&mut self, mut count: i32) -> MarbleId {
        let mut id = self.current;
        while count > 0 {
            id = self.marbles[id].next;
            count -= 1;
        }
        id
    }

    fn counter_clockwise(&mut self, mut count: i32) -> MarbleId {
        let mut id = self.current;
        while count > 0 {
            id = self.marbles[id].prev;
            count -= 1;
        }
        id
    }
}

struct Marble {
    value: MarbleValue,
    next: MarbleId,
    prev: MarbleId
}

impl Marble {
    fn create_empty(value: MarbleValue) -> Marble {
        Marble {
            value,
            next: 0,
            prev: 0
        }
    }
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut id = self.current;
        loop {
            let marble = &self.marbles[id];
            write!(f, "{}", marble.value)?;
            id = marble.next;
            if id == self.current {
                break;
            }
        }
        Ok(())
    }
}

