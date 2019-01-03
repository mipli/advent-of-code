use std::io::{self, prelude::*};
use std::str::FromStr;
use std::collections::{VecDeque, BTreeMap, BTreeSet};
use std::cmp::Ordering;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let system: System = input.parse()?;
    let checksum = system.clone().run();
    println!("Checksum: {}", checksum);

    let mut power = 4;
    loop {
        let mut sys = system.clone();
        sys.set_elf_power(power);
        let pre_elves = sys.count_elves();
        let checksum = sys.run();
        if pre_elves == sys.count_elves() {
            println!("Checksum({}): {}", power, checksum);
            break;
        }
        power += 1;
    }

    Ok(())
}

#[derive(Clone)]
struct System {
    next_id: ActorId,
    actors: Vec<Actor>,
    map: Map,
    tick: u32
}

impl System {
    fn checksum(&self) -> i32 {
        let hp: i32 = self.get_actors().iter().map(|(_, a)| {
            self.actors[*a as usize].hp
        }).sum();
        hp * self.tick as i32
    }

    fn set_elf_power(&mut self, power: i32) {
        for actor in &mut self.actors {
            if actor.species == Species::Elf {
                (*actor).attack_power = power;
            }
        }
    }

    fn run(&mut self) -> i32 {
        while self.count_goblins() > 0 && self.count_elves() > 0 {
            self.tick();
        }
        self.checksum()
    }

    fn tick(&mut self) {
        let actors = self.get_actors();
        for &(position, actor) in &actors {
            if self.count_goblins() == 0 || self.count_elves() == 0 {
                return;
            }
            if self.actors[actor as usize].hp > 0 {
                if let Some(position) = self.perform_move(actor, position) {
                    self.perform_attack(actor, position);
                } else {
                    self.perform_attack(actor, position);
                }
            }
        }
        self.tick += 1;
    }

    fn count_goblins(&self) -> i32 {
        self.actors.iter().filter(|a| a.hp > 0 && a.species == Species::Goblin).count() as i32
    }

    fn count_elves(&self) -> i32 {
        self.actors.iter().filter(|a| a.hp > 0 && a.species == Species::Elf).count() as i32
    }

    fn get_actor(&self, id: &ActorId) -> &Actor {
        &self.actors[*id as usize]
    }

    fn perform_move(&mut self, actor: ActorId, position: Position) -> Option<Position> {
        let pos = self.get_walk_target(actor, position)?;
        if pos == position {
            Some(position)
        } else {
            let distances = self.map.distances(pos);
            let pos = position.get_neighbours().iter()
                .filter_map(|c| distances.get(&c).map(|dist| (c, dist)))
                .min_by_key(|&(_, dist)| dist)
                .map(|(c, _)| c.clone());
            if let Some(step) = pos {
                self.map.set(&position, Tile::Empty);
                self.map.set(&step, Tile::Actor(actor));
                return Some(step);
            }
            Some(position)
        }
    }

    fn get_walk_target(&self, actor: ActorId, position: Position) -> Option<Position> {
        let actor = self.actors[actor as usize];
        let closest = self.get_actors().iter().filter_map(|(coord, act)| {
            if *act == actor.id || self.actors[*act as usize].species == actor.species {
                None
            } else {
                Some(position.distance(coord))
            }
        }).min();
        if closest == Some(1) {
            return Some(position);
        }
        let distances = self.map.distances(position);
        self.get_actors()
            .iter()
            .filter_map(|(coord, act)| {
                if *act == actor.id || self.actors[*act as usize].species == actor.species {
                    None
                } else {
                    Some(coord.get_neighbours()
                        .iter()
                        .filter_map(|c| {
                            if self.map.get(c) != Some(Tile::Empty) && self.map.get(c) != Some(Tile::Actor(actor.id)) {
                                None
                            } else {
                                Some(*c)
                            }
                        })
                        .collect::<Vec<Position>>())
                }
            })
            .flatten()
            .filter_map(|pos| distances.get(&pos).map(|dist| (pos, dist)))
            .min_by_key(|&(_, dist)| dist)
            .map(|(pos, _)| pos)
    }

    fn perform_attack(&mut self, actor: ActorId, position: Position) {
        let actor = self.actors[actor as usize];
        let target = position.get_neighbours().iter()
            .filter_map(|pos| {
                match self.map.get(pos) {
                    Some(Tile::Actor(id)) => {
                        let t = self.actors[id as usize];
                        if t.species == actor.species {
                            None
                        } else {
                            Some((t.hp, t.id, pos.clone()))
                        }
                    },
                    _ => None
                }
            })
            .min_by_key(|t| t.0);

        if let Some((_, target, pos)) = target {
            let hp = self.attack(&actor.id, &target);
            if hp <= 0 {
                self.map.set(&pos, Tile::Empty);
            }
        }
    }

    fn attack(&mut self, attacker: &ActorId, target: &ActorId) -> i32 {
        let power = self.actors[*attacker as usize].attack_power;
        self.actors[*target as usize].hp -= power;
        self.actors[*target as usize].hp
    }

    fn get_actors(&self) -> Vec<(Position, ActorId)> {
        let mut actors = vec![];
        self.map.grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                match tile {
                    Tile::Actor(id) => {
                        if self.actors[*id as usize].hp > 0 {
                            actors.push(((x as i32, y as i32).into(), *id))
                        }
                    },
                    _ => {}
                }
            });
        });
        actors
    }

    fn create_actor(&mut self, species: Species) -> ActorId {
        self.actors.push(Actor {
            id: self.next_id,
            hp: 200,
            attack_power: 3,
            species
        });
        self.next_id += 1;
        self.next_id - 1
    }
}

impl FromStr for System {
    type Err = Error;

    fn from_str(input: &str) -> Result<System, Error> {
        let mut system = System {
            tick: 0,
            next_id: 0,
            actors: vec![],
            map: Map {
                grid: vec![],
                width: 1,
                height: 1
            }
        };
        let grid = input.lines().fold(vec![], |mut grid, row| {
            let grid_row = row.bytes().map(|b| {
                match b {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Empty,
                    b'G' => {
                        let actor = system.create_actor(Species::Goblin);
                        Tile::Actor(actor)
                    },
                    b'E' => {
                        let actor = system.create_actor(Species::Elf);
                        Tile::Actor(actor)
                    },
                    _ => unreachable!("Input contained invalid character"),
                }
            }).collect::<Vec<_>>();
            grid.push(grid_row);
            grid
        });
        system.map = Map { 
            height: grid.len() as i32,
            width: grid[0].len() as i32,
            grid,
        };
        Ok(system)
    }
}
impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = String::new();

        self.map.grid.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Actor(id) => {
                        let actor = self.get_actor(id);
                        match actor.species {
                            Species::Elf => 'E',
                            Species::Goblin => 'G',
                        }
                    }
                };
                buf.push(c);
            });
            buf.push('\n');
        });
        write!(f, "{}", buf)
    }
}


type ActorId = u32;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Actor {
    id: ActorId,
    hp: i32,
    attack_power: i32,
    species: Species
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Species {
    Elf,
    Goblin

}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Actor(ActorId),
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Map {
    fn set(&mut self, position: &Position, tile: Tile) {
        self.grid[position.y as usize][position.x as usize] = tile
    }
    fn get(&self, position: &Position) -> Option<Tile> {
        if self.is_invalid(position) {
            None
        } else {
            Some(self.grid[position.y as usize][position.x as usize])
        }
    }

    fn is_invalid(&self, position: &Position) -> bool {
        position.x < 0 || position.x >= self.width || position.y < 0 || position.y >= self.height
    }

    fn distances(&self, origin: Position) -> BTreeMap<Position, usize> {
        let mut distances = BTreeMap::default();
        distances.insert(origin, 0);

        let mut todo = VecDeque::new();
        todo.push_front(origin);
        let mut visisted = BTreeSet::new();
        let mut todo_set = BTreeSet::new();

        while let Some(node) = todo.pop_front() {
            visisted.insert(node);
            todo_set.remove(&node);
            let neighbours = node.get_neighbours();
            for neighbour in &neighbours {
                if visisted.contains(neighbour) {
                    continue;
                }
                if self.get(neighbour) != Some(Tile::Empty) {
                    visisted.insert(*neighbour);
                    continue;
                }
                if !todo_set.contains(neighbour) {
                    todo.push_back(neighbour.clone());
                    todo_set.insert(neighbour.clone());
                }

                let dist = 1 + *distances.get(&node).unwrap_or(&0);
                if !distances.contains_key(neighbour) || dist < distances[neighbour] {
                    distances.insert(*neighbour, dist);
                }
            }
        }

        distances
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl From<(i32, i32)> for Position {
    fn from(p: (i32, i32)) -> Self {
        Position {
            x: p.0,
            y: p.1,
        }
    }
}


impl Position {
    fn distance(&self, other:  &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_neighbours(&self) -> [Position; 4] {
        [
            (self.x, self.y - 1).into(),
            (self.x - 1, self.y).into(),
            (self.x + 1, self.y).into(),
            (self.x, self.y + 1).into(),
        ]
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

#[derive(Debug, Clone, Copy, Hash)]
struct PathPosition {
    position: Position,
    distance: i32
}

impl PartialOrd for PathPosition {
    fn partial_cmp(&self, other: &PathPosition) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathPosition {
    fn cmp(&self, other: &PathPosition) -> Ordering {
        let ord = if self.distance == other.distance {
            self.position.cmp(&other.position)
        } else {
            self.distance.cmp(&other.distance)
        };
        ord.reverse()
    }
}

impl PartialEq for PathPosition {
    fn eq(&self, other: &PathPosition) -> bool {
        self.position == other.position
    }
}

impl Eq for PathPosition {}
