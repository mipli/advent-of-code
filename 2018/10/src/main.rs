#[macro_use]
extern crate lazy_static;

use std::io::{self, prelude::*};
use std::str::FromStr;

use regex::Regex;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let mut points = vec![];
    for line in input.lines() {
        let point: Point = line.parse()?;
        points.push(point);
    }

    let mut grid = Grid::new(points);

    for _ in 0..20_000 {
        grid.step();
        let bounds = grid.bounds();
        if bounds.width() < 100 && bounds.height() < 100 {
            writeln!(io::stdout(), "{}", grid)?;
            writeln!(io::stdout(), "{}", "=".repeat(80))?;
        }
    }

    Ok(())
}

struct Grid {
    points: Vec<Point>,
    step: u32
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        Grid { 
            points,
            step: 0,
        }
    }

    fn step(&mut self) {
        self.step += 1;
        for p in &mut self.points {
            p.x += p.vx;
            p.y += p.vy;
        }
    }

    fn bounds(&self) -> Bounds {
        Bounds::create(&self.points)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bounds = self.bounds();
        let mut buf = vec![vec![b'.'; bounds.width() as usize]; bounds.height() as usize];

        self.points.iter().for_each(|p| {
            let x = p.x - bounds.min_x;
            let y = p.y - bounds.min_y;
            buf[y as usize][x as usize] = b'#';
        });
        let mut out = String::new();
        for row in buf {
            out.push_str(std::str::from_utf8(&row).unwrap());
            out.push('\n');
        }
        write!(f, "Step: {}\n{}", self.step, out)
    }
}

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(line: &str) -> Result<Point, Error> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^position=<\s*(?P<px>-?\d+),\s*(?P<py>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>.*").unwrap();
        }

        match LINE_RE.captures(line) {
            Some(captures) => {
                let x: i32 = captures["px"].parse()?;
                let y: i32 = captures["py"].parse()?;
                let vx: i32 = captures["vx"].parse()?;
                let vy: i32 = captures["vy"].parse()?;
                Ok(Point { x, y, vx, vy })
            },
            None => return Err(Box::<std::error::Error>::from("Could not parse point information"))
        }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos: {}, {} - vel: {}, {}", self.x, self.y, self.vx, self.vy)
    }
}

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

impl Bounds {
    fn create(points: &[Point]) -> Bounds {
        points.iter().fold(Bounds {
            min_x: points[0].x,
            max_x: points[0].x,
            min_y: points[0].y,
            max_y: points[0].y,
        }, |bounds, point| {
            Bounds {
                min_x: std::cmp::min(bounds.min_x, point.x),
                max_x: std::cmp::max(bounds.max_x, point.x),
                min_y: std::cmp::min(bounds.min_y, point.y),
                max_y: std::cmp::max(bounds.max_y, point.y),
            }
        })
    }

    fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }
}
