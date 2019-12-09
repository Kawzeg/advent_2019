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
    AdjustRelBase,
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Clone, Copy)]
struct Arg {
    value: i64,
    mode: ParameterMode,
}

#[derive(Debug)]
struct Operation {
    opcode: Op,
    args: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub struct Intcode {
    pos: usize,
    rel_base: usize,
    data: Vec<i64>,
    input: Vec<i64>,
    pub output: Vec<i64>,
    pub halted: bool
}
impl Intcode {
    pub fn from_data(data: Vec<i64>) -> Intcode {
        Intcode {
            pos: 0,
            rel_base: 0,
            data: data,
            input: Vec::new(),
            output: Vec::new(),
            halted: false,
        }
    }
    fn new(other: Intcode) -> Intcode {
        Intcode {
            pos: other.pos,
            data: other.data,
            input: other.input,
            output: other.output,
            halted: other.halted,
            rel_base: other.rel_base,
        }
    }
    fn deref(&self, at: usize) -> i64 {
        if at >= self.data.len() {
            println!("Trying to deref outside memory");
            0
        } else {
            println!("deref@{}={}", at, self.data[at]);
            self.data[at]
        }
    }
    fn arg(&self, offset: usize) -> i64 {
        self.data[self.pos + offset]
    }
    fn param_mode(op: i64, shift: i64) -> ParameterMode {
        match (op / shift) % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("UNKNOWN PARAMETER MODE {}", op / shift % 10),
        }
    }
    fn param_modes(&self) -> [ParameterMode; 3] {
        let mode1 = Self::param_mode(self.arg(0), 100);
        let mode2 = Self::param_mode(self.arg(0), 1000);
        let mode3 = Self::param_mode(self.arg(0), 10000);
        [mode1, mode2, mode3]
    }
    fn map_opcode(code: i64) -> Op {
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
            9 => Op::AdjustRelBase,
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
            Op::Read | Op::Write | Op::AdjustRelBase => vec![Arg {
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

    fn write_at(&mut self, arg: Arg, val: i64) {
        let addr = match arg.mode {
            ParameterMode::Immediate => panic!("Cannot write in immediate mode"),
            ParameterMode::Position => arg.value as usize,
            ParameterMode::Relative => (arg.value + self.rel_base as i64) as usize,
        };
        if addr >= self.data.len() {
            self.data.extend(vec![0; addr - self.data.len() + 1]);
        }
        println!("write@{}:={}", addr, val);
        self.data[addr] = val;
    }

    fn arg_to_val(&self, arg: Arg) -> i64 {
        println!("Getting arg {:?} from base {}", arg, self.rel_base);
        match arg.mode {
            ParameterMode::Immediate => arg.value,
            ParameterMode::Position => self.deref(arg.value as usize),
            ParameterMode::Relative => self.deref((arg.value + self.rel_base as i64) as usize),
        }
    }
}

fn add(val1: i64, val2: i64) -> i64 {
    val1 + val2
}

fn mul(val1: i64, val2: i64) -> i64 {
    val1 * val2
}

fn less_than(val1: i64, val2: i64) -> i64 {
    if val1 < val2 {
        1
    } else {
        0
    }
}

fn equals(val1: i64, val2: i64) -> i64 {
    if val1 == val2 {
        1
    } else {
        0
    }
}

fn do_op(code: Intcode, op: fn(i64, i64) -> i64) -> Intcode {
    let args = code.op().args;
    let val1 = code.arg_to_val(args[0]);
    let val2 = code.arg_to_val(args[1]);
    let r = op(val1, val2);
    let mut output = Intcode::new(code);
    output.write_at(args[2], r);
    output.pos += 4;
    output
}

fn read(code: Intcode) -> Intcode {
    let args = code.op().args;
    let r = code.input[0];
    let mut new_code = Intcode::new(code);
    new_code.write_at(args[0], r);
    new_code.pos += 2;
    new_code.input = new_code.input[1..].to_vec();
    new_code
}

fn write(code: Intcode) -> Intcode {
    let args = code.op().args;
    let val = code.arg_to_val(args[0]);
    let mut new_code = Intcode::new(code);
    new_code.output.push(val);
    new_code.pos += 2;
    new_code
}

fn jump_if_con(code: Intcode, con: fn(i64) -> bool) -> Intcode {
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

fn adjust_rel_base(code: Intcode) -> Intcode {
    let args = code.op().args;
    let val = code.arg_to_val(args[0]);
    let new_base = code.rel_base as i64 + val;
    let mut new_code = Intcode::new(code);
    new_code.rel_base = new_base as usize;
    new_code.pos += 2;
    new_code
}

pub fn run(input: &Intcode) -> Intcode {
    let mut code = input.clone();
    loop {
        println!("Running {:?}", code.op());
        code = match code.op().opcode {
            Op::Stop => {
                println!("HALT");
                code.halted = true;
                break;
            },
            Op::Add => do_op(code, add),
            Op::Mul => do_op(code, mul),
            Op::LessThan => do_op(code, less_than),
            Op::Equals => do_op(code, equals),
            Op::Read => {
                // If no input is available, halt
                if code.input.len() > 0 {
                    read(code)
                } else {
                    break;
                }
            },
            Op::Write => write(code),
            Op::JumpIfTrue => jump_if_con(code, |x| {x != 0}),
            Op::JumpIfFalse => jump_if_con(code, |x| {x == 0}),
            Op::AdjustRelBase => adjust_rel_base(code),
        };
    }
    code
}

pub fn intcode_from_file<P: AsRef<Path>>(file: P) -> io::Result<Intcode> {
    let data = std::fs::read_to_string(file)?;
    let ints: Vec<i64> = data
        .trim_end()
        .split(",")
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    Ok(Intcode::from_data(ints))
}

pub fn run_with_io(code: &Intcode, input: Vec<i64>) -> Intcode {
    let mut copy = code.clone();
    copy.input = input;
    run(&copy)
}

#[cfg(test)]
mod tests {
    use super::{Intcode, run};
    fn test_output(code: Intcode, expected_out: Vec<i64>) {
        let out = run(&code);
        assert_eq!(out.output, expected_out);
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let code = Intcode::from_data(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        test_output(code, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        let code = Intcode::from_data(vec![1102,34915192,34915192,7,4,7,99,0]);
        test_output(code, vec![1219070632396864]);
        let code = Intcode::from_data(vec![104,1125899906842624,99]);
        test_output(code, vec![1125899906842624]);
    }
}
