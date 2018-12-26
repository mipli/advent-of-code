use std::io::{self, prelude::*};

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let mut digits = input.split(" ").map(|c| {
        match c.parse() {
            Ok(d) => d,
            Err(_) => 2
        }
    }).collect::<Vec<i32>>();

    digits.reverse();

    let root = parse_node(&mut digits).expect("Could not create nodes");

    println!("Sum part 1: {:?}", sum_meta(&root));
    println!("Sum part 2: {:?}", sum_child_based(&root));

    Ok(())
}

fn parse_node(digits: &mut Vec<i32>) -> Option<Node> {
    let child_count = digits.pop()? as usize;
    let meta_count = digits.pop()? as usize;
    let mut children = vec![];
    let mut meta = vec![];

    for _ in 0..child_count {
        children.push(parse_node(digits)?);
    }

    for _ in 0..meta_count {
        meta.push(digits.pop()?);
    }
    Some(Node {
        child_count,
        meta_count,
        children,
        meta
    })
}

fn sum_meta(root: &Node) -> i32 {
    root.meta.iter().sum::<i32>() + root.children.iter().fold(0, |acc, r| acc + sum_meta(r))
}

fn sum_child_based(node: &Node) -> i32 {
    if node.child_count == 0 {
        sum_meta(node)
    } else {
        node.meta.iter().fold(0, |acc, &idx| {
            acc + match node.children.get((idx - 1) as usize) {
                Some(child) => sum_child_based(child),
                None => 0
            }
        })
    }
}

#[derive(Debug)]
struct Node {
    child_count: usize,
    meta_count: usize,
    children: Vec<Node>,
    meta: Vec<i32>
}
