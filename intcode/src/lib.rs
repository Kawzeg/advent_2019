use std::io;
use std::path::Path;
use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    Stop,
    Add,
    Mul,
    Read,
    Put,
}

#[derive(Debug)]
pub struct Intcode {
    pos: usize,
    data: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>
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
    fn op(&self) -> Op {
        let code = self.arg(0);
        match code {
            99 => Op::Stop,
            1 => Op::Add,
            2 => Op::Mul,
            _ => {
                panic!("UNKNOWN OP CODE {}", code);
            }
        }
    }
}

fn add(val1: i32, val2: i32) -> i32 {
    val1 + val2
}

fn mul(val1: i32, val2: i32) -> i32 {
    val1 * val2
}

fn do_op(input: Intcode, op: fn(i32, i32) -> i32) -> Intcode {
    let val1 = input.deref(input.arg(1) as usize);
    let val2 = input.deref(input.arg(2) as usize);
    let r = op(val1, val2);
    let r_loc = input.arg(3);
    let mut new_data = input.data;
    new_data[r_loc as usize] = r;
    let new_pos = input.pos + 4;
    let mut output = Intcode::new(input);
    output.data = new_data;
    output.pos = new_pos;
    output
}

fn move_forward(input: Intcode, offset: usize) -> Intcode {
    let new_pos = input.pos + offset;
    let mut output = Intcode::new(input);
    output.pos = new_pos;
    output
}

fn read(code: Intcode) -> Intcode {
    code
}

fn put(code: Intcode) -> Intcode {
    code
}

fn run_once(code: Intcode) -> Intcode {
    match code.op() {
        Op::Stop => code,
        Op::Add => do_op(code, add),
        Op::Mul => do_op(code, mul),
        Op::Read => read(code),
        Op::Put => put(code),
    }
}

pub fn run(input: Intcode) -> Intcode {
    let mut output = input;
    loop {
        let r_loc = output.arg(3);
        println!("[{:?}: {}({}) {}({}) @{}]", output.op(),
            output.arg(1), output.deref(output.arg(1) as usize),
            output.arg(2), output.deref(output.arg(2) as usize),
            output.arg(3));
        match output.op() {
            Op::Stop => {
                break;
            }
            Op::Add => output = do_op(output, add),
            Op::Mul => output = do_op(output, mul),
        }
        println!("-> {}", output.deref(r_loc as usize));
        //println!("Intermediate {:?}", output);
    }
    output
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
