use std::io::{self, prelude::*};
use std::collections::{HashSet};

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let frequencies = parse_input(&input)?;

    let freq = get_frequency(&frequencies);
    writeln!(io::stdout(), "Frequency: {}", freq)?;

    let repeat = get_repeat_frequency(&frequencies);
    writeln!(io::stdout(),"Repeat: {}", repeat)?;

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<i32>, Error> {
    input.lines().try_fold(vec![], |mut acc, line| {
        acc.push(line.parse()?);
        Ok(acc)
    })
}

fn get_frequency(frequencies: &[i32]) -> i32 {
    frequencies.iter().sum()
}

fn get_repeat_frequency(frequencies: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut current = 0;

    frequencies.into_iter().cycle().any(|f| {
        current += *f;
        !seen.insert(current)
    });
    current
}
