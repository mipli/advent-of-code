use std::io::Write;
use fnv::FnvHashMap;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let serial_number = 6878;

    let mut grid = Grid::new(serial_number);
    grid.generate();

    let mut sum = 0;
    let mut point = (0, 0);
    let mut size = 0;
    for s in 1..16 {
        for x in 1..=(300 - s + 1) {
            for y in 1..=(300 - s + 1) {
                match grid.get_square_level(x, y, s) {
                    Some(level) if level > sum => {
                        sum = level;
                        point = (x, y);
                        size = s;
                    },
                    _ => {}
                }
            }
        }
    }

    writeln!(std::io::stdout(), "{}, {}, {}: {}", point.0, point.1, size, sum)?;

    Ok(())
}

struct Grid {
    width: usize,
    height: usize,
    serial_number: i32,
    values: Vec<i32>,
    cache: FnvHashMap<(i32, i32, i32), i32>,
}

impl Grid {
    fn new(serial_number: i32) -> Grid {
        Grid {
            width: 300,
            height: 300,
            values: vec![0; 300 * 300],
            serial_number,
            cache: FnvHashMap::default()
        }
    }

    fn get_square_level(&mut self, left: i32, top: i32, size: i32) -> Option<i32> {
        if left > self.width as i32 - (size - 1) || top > self.height as i32 - (size - 1) {
            return None;
        }

        let mut sum = 0;
        for x in left..(left + size) {
            sum += self.get_power_level(x, top);
        }
        if size > 1 {
            for y in (top + 1)..(top + size) {
                sum += self.get_power_level(left, y);
            }
            sum += self.cache.get(&(left + 1, top + 1, size - 1)).expect("Smaller square must be calculated first");
        }
        self.cache.insert((left, top, size), sum);
        Some(sum)
    }

    fn get_power_level(&self, x: i32, y: i32) -> i32 {
        self.values[((x -1) + ((y - 1) * self.width as i32)) as usize]
    }

    fn generate(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let power = self.generate_power_level((x + 1) as i32, (y + 1) as i32);
                self.values[(x + (y * self.width)) as usize] = power;
            }
        }
    }

    fn generate_power_level(&self, x: i32, y: i32) -> i32 {
        let rack_id = x + 10;
        let mut power = y * rack_id;
        power += self.serial_number;
        power *= rack_id;
        if power < 100 {
            power = 0;
        } else {
            power = ((power as f32)/ 100f32).floor() as i32;
        }
        power = power % 10;
        power - 5
    }
}

