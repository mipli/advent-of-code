use std::io::{self, prelude::*};
use regex::Regex;

type Error = Box<std::error::Error>;

#[derive(Debug)]
struct Claim {
    id: u32,
    top: u32,
    left: u32,
    width: u32,
    height: u32
}

impl Claim {
    fn intersects(&self, other: &Claim) -> bool {
        (self.left <= other.width + other.left) && (self.left + self.width >= other.left) && (self.top <= other.height + other.top) && (self.height + self.top >= other.top)
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let claims = parse_input(&input)?;
    let area = get_overlap_area(&claims);
    writeln!(io::stdout(), "Overlap area: {}", area)?;

    if let Some(claim) = get_unique_claim(&claims) {
        writeln!(io::stdout(), "Unique claim: {}", claim.id)?;
    } else {
        writeln!(io::stdout(), "No unique claim found")?;
    }

    Ok(())
}

fn get_unique_claim<'a>(claims: &'a [Claim]) -> Option<&'a Claim> {
    claims.iter().find_map(|claim| {
        let has_overlap = claims.iter().any(|other| {
            if other.id == claim.id {
                false
            } else {
                claim.intersects(other)
            }
        });
        if has_overlap {
            None
        } else {
            Some(claim)
        }
    })
}

fn get_overlap_area(claims: &[Claim]) -> usize {
    let mut fabric = vec![0; 1000*1000];
    claims.iter().for_each(|claim| {
        for y in claim.top..claim.top+claim.height {
            for x in claim.left..claim.left+claim.width {
                fabric[(x + (y * 1000)) as usize] += 1;
            }
        }
    });
    fabric.iter().fold(0, |acc, &overlap| {
        if overlap > 1 {
            acc + 1
        } else {
            acc
        }
    })
}

fn parse_input(input: &str) -> Result<Vec<Claim>, Error> {
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let mut claims = vec![];
    for cap in re.captures_iter(input) {
        let id = cap.name("id").ok_or(ParseError)?.as_str().parse()?;
        let top = cap.name("top").ok_or(ParseError)?.as_str().parse()?;
        let left = cap.name("left").ok_or(ParseError)?.as_str().parse()?;
        let width = cap.name("width").ok_or(ParseError)?.as_str().parse()?;
        let height = cap.name("height").ok_or(ParseError)?.as_str().parse()?;
        claims.push(Claim {
            id,
            top,
            left,
            width,
            height
        });
    }
    Ok(claims)
}

#[derive(Debug)]
struct ParseError;

impl std::error::Error for ParseError {
    fn description(&self) -> &str { &"" }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing Error")
    }
}
