use std::collections::VecDeque;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Op {
    Stop,
    Add,
    Mul,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, Clone, Copy)]
struct Arg {
    value: i32,
    mode: ParameterMode,
}

#[derive(Debug)]
struct Operation {
    opcode: Op,
    args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Intcode {
    pos: usize,
    data: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
}
impl Intcode {
    fn from_data(data: Vec<i32>) -> Intcode {
        Intcode {
            pos: 0,
            data: data,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
    fn new(other: Intcode) -> Intcode {
        Intcode {
            pos: other.pos,
            data: other.data,
            input: other.input,
            output: other.output,
        }
    }
    fn deref(&self, at: usize) -> i32 {
        self.data[at]
    }
    fn arg(&self, offset: usize) -> i32 {
        self.data[self.pos + offset]
    }
    fn param_mode(op: i32, shift: i32) -> ParameterMode {
        match (op / shift) % 10 {
            1 => ParameterMode::Immediate,
            0 => ParameterMode::Position,
            _ => panic!("UNKNOWN PARAMETER MODE {}", op / shift % 10),
        }
    }
    fn param_modes(&self) -> [ParameterMode; 3] {
        let mode1 = Self::param_mode(self.arg(0), 100);
        let mode2 = Self::param_mode(self.arg(0), 1000);
        let mode3 = Self::param_mode(self.arg(0), 10000);
        [mode1, mode2, mode3]
    }
    fn map_opcode(code: i32) -> Op {
        match code {
            99 => Op::Stop,
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::Read,
            4 => Op::Write,
            5 => Op::JumpIfTrue,
            6 => Op::JumpIfFalse,
            7 => Op::LessThan,
            8 => Op::Equals,
            _ => {
                panic!("UNKNOWN OP CODE {}", code);
            }
        }
    }
    fn op(&self) -> Operation {
        let code = self.arg(0);
        let opcode = code % 100;
        let op = Self::map_opcode(opcode);
        let args: Vec<Arg> = match op {
            Op::Add | Op::Mul | Op::LessThan | Op::Equals => {
                let modes = self.param_modes();
                let values = [self.arg(1), self.arg(2), self.arg(3)];
                modes
                    .iter()
                    .zip(values.iter())
                    .map(|(mode, val)| Arg {
                        value: *val,
                        mode: *mode,
                    })
                    .collect()
            }
            Op::Read | Op::Write => vec![Arg {
                value: self.arg(1),
                mode: Self::param_mode(code, 100),
            }],
            Op::JumpIfFalse | Op::JumpIfTrue => vec![Arg {
                value: self.arg(1),
                mode: Self::param_mode(code, 100)
            }, Arg {
                value: self.arg(2),
                mode: Self::param_mode(code, 1000)
            }],
            Op::Stop => vec![],
        };
        Operation {
            opcode: op,
            args: args,
        }
    }

    fn arg_to_val(&self, arg: Arg) -> i32 {
        match arg.mode {
            ParameterMode::Immediate => arg.value,
            ParameterMode::Position => self.deref(arg.value as usize),
        }
    }
}

fn add(val1: i32, val2: i32) -> i32 {
    val1 + val2
}

fn mul(val1: i32, val2: i32) -> i32 {
    val1 * val2
}

fn less_than(val1: i32, val2: i32) -> i32 {
    if val1 < val2 {
        1
    } else {
        0
    }
}

fn equals(val1: i32, val2: i32) -> i32 {
    if val1 == val2 {
        1
    } else {
        0
    }
}

fn do_op(code: Intcode, op: fn(i32, i32) -> i32) -> Intcode {
    let args = code.op().args;
    let val1 = code.arg_to_val(args[0]);
    let val2 = code.arg_to_val(args[1]);
    let r = op(val1, val2);
    let r_loc = code.arg(3);
    let mut output = Intcode::new(code);
    output.data[r_loc as usize] = r;
    output.pos += 4;
    output
}

fn read(code: Intcode) -> Intcode {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read stdin");
    let r = input.trim().parse::<i32>().unwrap();
    let r_loc = code.arg(1);
    let mut new_code = Intcode::new(code);
    new_code.data[r_loc as usize] = r;
    new_code.pos += 2;
    new_code
}

fn write(code: Intcode) -> Intcode {
    let args = code.op().args;
    let val = code.arg_to_val(args[0]);
    let mut new_code = Intcode::new(code);
    println!("Intcode says: {}", val);
    new_code.pos += 2;
    new_code
}

fn jump_if_con(code: Intcode, con: fn(i32) -> bool) -> Intcode {
    let args = code.op().args;
    let val = code.arg_to_val(args[0]);
    let new_pos: usize = if con(val) {
        code.arg_to_val(args[1]) as usize
    } else {
        code.pos + 3
    };
    let mut new_code = Intcode::new(code);
    new_code.pos = new_pos;
    new_code
}

pub fn run(input: Intcode) -> Intcode {
    let mut code = input;
    loop {
        println!(
            "[{:?}]",
            code.op()
        );

        code = match code.op().opcode {
            Op::Stop => break,
            Op::Add => do_op(code, add),
            Op::Mul => do_op(code, mul),
            Op::LessThan => do_op(code, less_than),
            Op::Equals => do_op(code, equals),
            Op::Read => read(code),
            Op::Write => write(code),
            Op::JumpIfTrue => jump_if_con(code, |x| {x != 0}),
            Op::JumpIfFalse => jump_if_con(code, |x| {x == 0}),
        };
    }
    code
}

pub fn intcode_from_file<P: AsRef<Path>>(file: P) -> io::Result<Intcode> {
    let data = std::fs::read_to_string(file)?;
    let ints: Vec<i32> = data
        .trim_end()
        .split(",")
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    Ok(Intcode::from_data(ints))
}

fn run_with_param(code: &Intcode, noun: i32, verb: i32) -> i32 {
    let mut new_data = code.data.clone();
    new_data[1] = noun;
    new_data[2] = verb;
    let input = Intcode::from_data(new_data);
    let output = run(input);
    output.data[0]
}

fn run_with_io(code: Intcode, input: Vec<i32>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
