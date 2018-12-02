use std::io::{self, prelude::*};
use itertools::*;
use std::collections::{HashMap};

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (doubles, triples) = get_double_triple_count(&input);
    writeln!(io::stdout(), "Checksum: {}", doubles * triples)?;

    let common = get_common_string(&input).ok_or("")?;
    writeln!(io::stdout(), "Common: {}", common)?;

    Ok(())
}

fn get_common_string(input: &str) -> Option<String> {
    input.lines().tuple_combinations().find_map(|(line, other)| {
        match get_str_diff(line, other).as_slice() {
            &[idx] => Some(line.chars().take(idx).chain(line.chars().skip(idx+1)).collect()),
            _ => None
        }
    })
}

fn get_str_diff(a: &str, b: &str) -> Vec<usize> {
    a.chars().zip(b.chars()).enumerate().fold(vec![], |mut diffs, (i, (a, b))| {
        if a != b {
            diffs.push(i);
        }
        diffs
    })
}

fn get_double_triple_count(input: &str) -> (usize, usize) {
    let mut chars: HashMap<char, usize> = HashMap::new();
    input
        .lines()
        .fold((0, 0), |(doubles, triples), line| {
            chars.clear();
            let mut has_double = false;
            let mut has_triple = false;
            line.chars().for_each(|c| {
                *chars.entry(c).or_default() += 1
            });
            chars.values().for_each(|&n| {
                if n == 2 {
                    has_double = true;
                } else if n == 3 {
                    has_triple = true;
                }
            });
            (
                doubles + has_double as usize,
                triples + has_triple as usize
            )
        })
}

/*
 *  A clean solution using itertool's group_by
 *  Implemented the other solution since this has to loop the whole input twice, once to sort and
 *  once to create the groups
 */
/*
fn clean_get_double_triple_count(input: &str) -> (usize, usize) {
    input
        .lines()
        .fold((0, 0), |(mut doubles, mut triples), line| {
            let mut has_double = false;
            let mut has_triple = false;
            line
                .bytes()
                .sorted()
                .into_iter()
                .group_by(|e| *e)
                .into_iter()
                .any(|(_, e)| {
                    match e.count() {
                        2 if !has_double => {
                            has_double = true;
                            doubles += 1;
                        }
                        3 if !has_triple => {
                            has_triple = true;
                            triples += 1;
                        }
                        _ => {}
                    }
                    has_double && has_triple
                });
            (doubles, triples)
        })
}
*/
