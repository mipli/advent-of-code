#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;

use strum::{self, IntoEnumIterator};

use std::io::{self, prelude::*};
use std::str::FromStr;
use regex::Regex;


type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.trim().to_string();

    let mut system = System::new();

    let (tests, program) = parse_input(&input)?;

    while system.codes.iter().filter(|c| c.is_none()).count() > 0 {
        tests.iter().for_each(|test| {
            system.find_valid(&test);
        });
    }

    let mut register = Register::new(0, 0, 0, 0);
    for line in &program {
        register = system.execute(&register, line[0], line[1], line[2], line[3]);
    }
    println!("{:?}", register);

    Ok(())
}

fn parse_input(input: &str) -> Result<(Vec<Test>, Vec<[u32; 4]>), Error> {
    lazy_static! {
        static ref BEFORE_RE: Regex = Regex::new(r"^Before:\s+(?P<reg>\[\d+, \d+, \d+, \d+\])").unwrap();
        static ref AFTER_RE: Regex = Regex::new(r"^After:\s+(?P<reg>\[\d+, \d+, \d+, \d+\])").unwrap();
        static ref INPUT_RE: Regex = Regex::new(r"^(?P<a>\d+) (?P<b>\d+) (?P<c>\d+) (?P<d>\d+)").unwrap();
    }

    let mut before: Option<Register> = None;
    let mut inp: Option<[u32; 4]> = None;

    let mut tests = vec![];

    let mut reading_test = false;

    let mut program: Vec<[u32; 4]> = vec![];

    for line in input.lines() {
        match BEFORE_RE.captures(line) {
            Some(cap) => {
                before = Some(cap["reg"].parse()?);
                reading_test = true;
            },
            None => {}
        }
        match AFTER_RE.captures(line) {
            Some(cap) => {
                let after = Some(cap["reg"].parse()?);

                tests.push(Test {
                    before: before.unwrap(),
                    input: inp.unwrap(),
                    after: after.unwrap(),
                });
                before = None;
                inp = None;
                reading_test = false;
            },
            None => {}
        }
        match INPUT_RE.captures(line) {
            Some(cap) => {
                if reading_test {
                    inp = Some([
                               cap["a"].parse()?,
                               cap["b"].parse()?,
                               cap["c"].parse()?,
                               cap["d"].parse()?,
                    ]);
                } else {
                    program.push([
                               cap["a"].parse()?,
                               cap["b"].parse()?,
                               cap["c"].parse()?,
                               cap["d"].parse()?,
                    ]);
                }
            },
            None => {}
        }
    }
    Ok((tests, program))
}


struct Test {
    before: Register,
    input: [u32; 4],
    after: Register
}

struct System { 
    codes: [Option<OpCode>; 16]
}

impl System {
    fn new() -> System {
        System {
            codes: [None; 16]
        }
    }

    fn execute(&self, input: &Register, i: u32, a: u32, b: u32, c: u32) -> Register {
        self.run(self.codes[i as usize].unwrap(), input, a, b, c)
    }

    fn find_valid(&mut self, test: &Test) -> Vec<OpCode> {
        let mut out = vec![];
        for op_code in OpCode::iter() {
            if !self.codes.contains(&Some(op_code)) {
                if self.run(op_code, &test.before, test.input[1], test.input[2], test.input[3]) == test.after {
                    out.push(op_code);
                }
            }
        }

        if out.len() == 1 {
            self.codes[test.input[0] as usize] = Some(out[0]);
        }
        out
    }

    fn run(&self, opcode: OpCode, register: &Register, a: u32, b: u32, c: u32) -> Register {
        // println!("{:?}", register);
        // println!("{:?} {} {} {}", opcode, a, b, c);
        match opcode {
            OpCode::Addr => register.set(c, register.get(a) + register.get(b)),
            OpCode::Addi => register.set(c, register.get(a) + b),
            OpCode::Muli => register.set(c, register.get(a) * b),
            OpCode::Mulr => register.set(c, register.get(a) * register.get(b)),
            OpCode::Seti => register.set(c, a),
            OpCode::Setr => register.set(c, register.get(a)),
            OpCode::Bani => register.set(c, register.get(a) & b),
            OpCode::Banr => register.set(c, register.get(a) & register.get(b)),
            OpCode::Bori => register.set(c, register.get(a) | b),
            OpCode::Borr => register.set(c, register.get(a) | register.get(b)),
            OpCode::Eqir => {
                if a == register.get(b) {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            },
            OpCode::Eqri => {
                if register.get(a) == b {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            },
            OpCode::Eqrr => {
                if register.get(a) == register.get(b) {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            },
            OpCode::Gtir => {
                if a > register.get(b) {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            },
            OpCode::Gtri => {
                if register.get(a) > b {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            },
            OpCode::Gtrr => {
                if register.get(a) > register.get(b) {
                    register.set(c, 1)
                } else {
                    register.set(c, 0)
                }
            }
        }
    }
}

#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Seti,
    Setr,
    Bani,
    Banr,
    Bori,
    Borr,
    Eqir,
    Eqri,
    Eqrr,
    Gtir,
    Gtri,
    Gtrr,

}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Register {
    inner: [u32; 4]
}

impl Register {
    fn new(a: u32, b: u32, c: u32, d: u32) -> Register {
        Register {
            inner: [a, b, c, d]
        }
    }

    fn get(&self, i: u32) -> u32 {
        self.inner[i as usize]
    }

    fn set(mut self, i: u32, v: u32) -> Self {
        self.inner[i as usize] = v;
        self
    }
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Register, Error> {
        lazy_static! {
            static ref REG_RE: Regex = Regex::new(r"^\[(?P<a>\d), (?P<b>\d), (?P<c>\d), (?P<d>\d)\]").unwrap();
        }

        match REG_RE.captures(s) {
            Some(cap) => {
                Ok(Register {
                    inner: [
                        cap["a"].parse()?,
                        cap["b"].parse()?,
                        cap["c"].parse()?,
                        cap["d"].parse()?,
                    ]
                })
            }
            None => Err(Box::<std::error::Error>::from("Could not parse initial state information"))
        }
    }
}
