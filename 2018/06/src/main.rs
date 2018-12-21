#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::io::{self, prelude::*};

use regex::Regex;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let points = parse_input(&input)?;

    let width = points.iter().fold(0, |mut max, p| {
        if p.x > max {
            max = p.x
        }
        if p.y > max {
            max = p.y
        }
        max
    });

    let mut plane = Plane::new(width + 2);
    plane.fill(&points);
    let largest = plane.get_largest(points.len() as u32);

    println!("Largest area: {}", largest); // 3290
    println!("Distance limited: {}", plane.get_distance_limited_region(10000, points)); // 45602

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Point>, Error> {
    let mut points = vec![];
    for line in input.lines() {
        let point: Point = line.parse().or_else(|err| {
            Err(Box::<std::error::Error>::from(format!("Failed to parse '{}': {}", line, err)))
        })?;
        points.push(point);
    }
    Ok(points)
}

#[derive(Debug, Clone)]
struct Plane {
    width: u32,
    inner: Vec<Option<u32>>
}

impl Plane {
    fn new(width: u32) -> Self {
        Plane {
            width,
            inner: vec![None; (width*width) as usize]
        }
    }

    fn get_largest(&self, max: u32) -> u32 {
        let mut areas = vec![];
        for i in 0..max {
            if !self.is_infinite(i as u32) {
                areas.push(self.get_area(i as u32));
            }
        }
        areas.iter().fold(0, |acc, &n| {
            if n >  acc {
                n
            } else {
                acc
            }
        })
    }

    fn fill(&mut self, points: &Vec<Point>) {
        for x in 0..self.width {
            for y in 0..self.width {
                let coord = Point { x, y };
                let val = points.iter().enumerate().fold((None, self.width * self.width), |cur, (index, point)| {
                    let distance = point.distance(&coord);
                    if distance < cur.1 {
                        (Some(index as u32), distance)
                    } else if distance == cur.1 {
                        (None, distance)
                    } else {
                        cur
                    }
                });
                match val.0 {
                    None => {},
                    Some(val) => self.set(coord, val)
                }
            }
        }
    }

    fn get(&self, point: Point) -> Option<u32> {
        self.inner[(point.x + (point.y * self.width)) as usize]
    }

    fn set(&mut self, point: Point, val: u32) {
        self.inner[(point.x + (point.y * self.width)) as usize] = Some(val);
    }

    fn get_area(&self, index: u32) -> u32 {
        self.inner.iter().filter(|n| *n == &Some(index)).count() as u32
    }

    fn get_distance_limited_region(&self, limit: u32, points: Vec<Point>) -> u32 {
        let mut area = 0;
        for x in 0..self.width {
            for y in 0..self.width {
                let coord = Point{x, y};
                let distance: u32 = points.iter().map(|p| {
                    p.distance(&coord)
                }).sum();
                if distance < limit {
                    area += 1;
                }
            }
        }
        area
    }

    fn is_infinite(&self, index: u32) -> bool {
        for x in 0..self.width {
            for y in 0..self.width {
                if self.get(Point{x, y}) == Some(index) {
                    if x == 0 || x == (self.width - 1) || y == 0 || y == (self.width - 1) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        let dx = (self.x as i32) - (other.x as i32);
        let dy = (self.y as i32) - (other.y as i32);
        (dx.abs() + dy.abs()) as u32
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Point, Error> {
        lazy_static! {
            static ref COORDINATE_RE: Regex = Regex::new(r"^(?P<x>\d+), (?P<y>\d+)$").unwrap();
        }
        match COORDINATE_RE.captures(s) {
            None => return Err(Box::<std::error::Error>::from("Could not parse coordinate")),
            Some(captures) => {
                let x: u32 = captures["x"].parse()?;
                let y: u32 = captures["y"].parse()?;
                Ok(Point { x, y })
            }
        }
    }
}
