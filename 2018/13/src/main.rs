use std::io::{self, prelude::*};
use std::str::FromStr;
use std::fmt;
use std::cmp::Ordering;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.to_string();

    let mut system: System = input.parse()?;
    // for i in 0..20 {
    loop {
        let res = system.tick();
        // writeln!(io::stdout(), "Tick: {}", i)?;
        // writeln!(io::stdout(), "{}", system)?;
        match res {
            Ok(()) => {}
            Err(collisions) => {
                for (pos, carts) in &collisions {
                    writeln!(io::stdout(), "Collsion at: {:?}, {:?}", pos, carts)?;
                    for cart in carts {
                        system.remove_cart(*cart);
                    }
                }
            }
        }
        if system.count_carts() == 1 {
            writeln!(io::stdout(), "One cart left at: {:?}", system.carts[0].pos)?;
            break;
        }
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => {
                *self = match *self {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            },
            Turn::Right => {
                *self = match *self {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right,
}

type Pos = (u32, u32);
type CartId = u32;

#[derive(Debug)]
struct Cart {
    id: CartId,
    pos: Pos,
    last_turn: Turn,
    direction: Direction
}

impl Cart {
    fn get_next_turn(&self) -> Turn {
        match self.last_turn {
            Turn::Right => Turn::Left,
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.pos.1 == other.pos.1 {
            self.pos.0.cmp(&other.pos.0)
        } else {
            self.pos.1.cmp(&other.pos.1)
        }
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.pos == other.pos && self.direction == other.direction
    }
}

impl Eq for Cart {}

struct System {
    grid: Grid,
    carts: Vec<Cart>
}

impl System {
    fn get_cart_at(&self, pos: Pos) -> Option<&Cart> {
        self.carts.iter().find(|c| c.pos == pos)
    }

    fn tick(&mut self) -> Result<(), Vec<(Pos, Vec<CartId>)>>{
        self.carts.sort();
        let mut colliding_carts = vec![];
        for i in 0..self.carts.len() {
            self.move_cart(i);
            let colliding = self.carts
                .iter()
                .filter_map(|c| {
                    if c.pos == self.carts[i].pos {
                        Some(c.id)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();
            if colliding.len() > 1 {
                colliding_carts.push((self.carts[i].pos, colliding));
            }
        }
        if !colliding_carts.is_empty() {
            Err(colliding_carts)
        } else {
            Ok(())
        }
    }

    fn remove_cart(&mut self, id: CartId) {
        self.carts.retain(|c| c.id != id)
    }

    fn count_carts(&self) -> usize {
        self.carts.len()
    }

    fn move_cart(&mut self, i: usize) {
        let mut cart = &mut self.carts[i];
        match cart.direction {
            Direction::Right => (*cart).pos.0 += 1,
            Direction::Left => (*cart).pos.0 -= 1,
            Direction::Up => (*cart).pos.1 -= 1,
            Direction::Down => (*cart).pos.1 += 1
        }
        match self.grid.get(cart.pos.0, cart.pos.1) {
            Track::RightDown => {
                cart.direction = match cart.direction {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Up => Direction::Left,
                }
            },
            Track::LeftUp => {
                cart.direction = match cart.direction {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Up => Direction::Right,
                }
            },
            Track::Intersection => {
                let turn = cart.get_next_turn();
                cart.direction.turn(turn);
                cart.last_turn = turn;
            },
            _ => {}
        }
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if let Some(cart) = self.get_cart_at((x, y)) {
                    match cart.direction {
                        Direction::Up => buf.push_str("^"),
                        Direction::Down => buf.push_str("v"),
                        Direction::Right => buf.push_str(">"),
                        Direction::Left => buf.push_str("<"),
                    }
                } else {
                    match self.grid.get(x, y) {
                        Track::Empty => buf.push_str(" "),
                        Track::Vertical => buf.push_str("|"),
                        Track::Horizontal => buf.push_str("-"),
                        Track::LeftUp => buf.push_str("/"),
                        Track::RightDown => buf.push_str("\\"),
                        Track::Intersection => buf.push_str("+"),
                    }
                }
            }
            buf.push_str("\n");
        }
        write!(f, "{}", buf)

    }
}


impl FromStr for System {
    type Err = Error;

    fn from_str(line: &str) -> Result<System, Error> {
        let mut grid = vec![];
        let mut carts = vec![];

        line.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    ' ' => grid.push(Track::Empty),
                    '|' => grid.push(Track::Vertical),
                    'v' => {
                        grid.push(Track::Vertical);
                        carts.push(Cart {
                            id: carts.len() as u32,
                            pos: (x as u32, y as u32),
                            last_turn: Turn::Right,
                            direction: Direction::Down
                        });
                    }
                    '^' => {
                        grid.push(Track::Vertical);
                        carts.push(Cart {
                            id: carts.len() as u32,
                            pos: (x as u32, y as u32),
                            last_turn: Turn::Right,
                            direction: Direction::Up
                        });
                    }
                    '-' => grid.push(Track::Horizontal),
                    '<' => {
                        grid.push(Track::Horizontal);
                        carts.push(Cart {
                            id: carts.len() as u32,
                            pos: (x as u32, y as u32),
                            last_turn: Turn::Right,
                            direction: Direction::Left
                        });
                    },
                    '>' => {
                        grid.push(Track::Horizontal);
                        carts.push(Cart {
                            id: carts.len() as u32,
                            pos: (x as u32, y as u32),
                            last_turn: Turn::Right,
                            direction: Direction::Right
                        });
                    },
                    '/' => grid.push(Track::LeftUp),
                    '\\' => grid.push(Track::RightDown),
                    '+' => grid.push(Track::Intersection),
                    _ => {}
                }
            });
        });

        let lines = line.lines().collect::<Vec<_>>();
        let width = lines[0].len() as u32;
        let height = lines.len() as u32;

        let grid = Grid {
            grid,
            width,
            height
        };

        Ok(System {
            grid,
            carts,
        })
    }
}


#[derive(Debug, Copy, Clone)]
enum Track {
    Empty,
    Intersection,
    Horizontal,
    Vertical,
    RightDown, // '\'
    LeftUp, // '/;
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Track>,
    width: u32,
    height: u32
}

impl Grid {
    fn get(&self, x: u32, y: u32) -> Track {
        self.grid[(x + (y * self.width)) as usize]
    }
}
