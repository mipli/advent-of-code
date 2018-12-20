use std::io::{self, prelude::*};

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let truncated = chain_reaction(input.clone().into_bytes());
    println!("Truncated polymer length: {:?}", truncated.len());

    let min_length = (b'a'..b'z').fold(truncated.len(), |acc, letter| {
        let polymer = input
            .bytes()
            .filter_map(|b| {
                if b != letter && b != (letter - 32) {
                    Some(b)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let length = chain_reaction(polymer).len();
        if acc < length {
            acc
        } else {
            length
        }
    });
    println!("Min length: {}", min_length);


    Ok(())
}

fn chain_reaction(input: Vec<u8>) -> Vec<u8> {
    let mut input = input.to_vec();
    loop {
        let truncated = collapse(&input);
        if truncated.len() == input.len() {
            return truncated;
        } else {
            input = truncated;
        }
    }
}

fn collapse(polymer: &[u8]) -> Vec<u8> {
    let (mut truncated, rem) = polymer.iter().fold((vec![], None), |trunc, c| {
        let (mut truncated, previous) = trunc;
        if previous.is_none() {
            return (truncated, Some(*c));
        }
        let previous = previous.unwrap();
        if (c - 32 == previous) || (c + 32 == previous) {
            (truncated, None)
        } else {
            truncated.push(previous);
            (truncated, Some(*c))
        }
    });
    truncated.push(rem.unwrap());
    truncated
}
