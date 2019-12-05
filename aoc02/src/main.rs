#[derive(Debug)]
enum Op {
    Stop,
    Add,
    Mul,
}

#[derive(Debug)]
struct Intcode {
    pos: usize,
    data: Vec<usize>,
}
impl Intcode {
    fn deref(&self, at: usize) -> usize {
        self.data[at]
    }
    fn arg(&self, offset: usize) -> usize {
        self.data[self.pos + offset]
    }
    fn arg1(&self) -> usize {
        self.arg(1)
    }
    fn arg2(&self) -> usize {
        self.arg(2)
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

fn add(val1: usize, val2: usize) -> usize {
    val1 + val2
}

fn mul(val1: usize, val2: usize) -> usize {
    val1 * val2
}

fn do_op(input: Intcode, op: fn(usize, usize) -> usize) -> Intcode {
    let val1 = input.deref(input.arg(1));
    let val2 = input.deref(input.arg(2));
    let r = op(val1, val2);
    let r_loc = input.arg(3);
    let mut new_data = input.data;
    new_data[r_loc] = r;
    let new_pos = input.pos + 4;
    Intcode {
        data: new_data,
        pos: new_pos,
    }
}

fn move_forward(input: Intcode) -> Intcode {
    let new_pos = input.pos + 4;
    Intcode {
        data: input.data,
        pos: new_pos,
    }
}

fn run_once(input: Intcode) -> Intcode {
    match input.op() {
        Op::Stop => input,
        Op::Add => do_op(input, add),
        Op::Mul => do_op(input, mul),
    }
}

pub fn run(input: Intcode) -> Intcode {
    let mut output = input;
    loop {
        let r_loc = output.arg(3);
        println!("[{:?}: {}({}) {}({}) @{}]", output.op(),
            output.arg(1), output.deref(output.arg(1)),
            output.arg(2), output.deref(output.arg(2)),
            output.arg(3));
        match output.op() {
            Op::Stop => {
                break;
            }
            Op::Add => output = do_op(output, add),
            Op::Mul => output = do_op(output, mul),
        }
        println!("-> {}", output.deref(r_loc));
        //println!("Intermediate {:?}", output);
    }
    output
}

use std::io;
use std::path::Path;
fn intcode_from_file<P: AsRef<Path>>(file: P) -> io::Result<Intcode> {
    let data = std::fs::read_to_string(file)?;
    let ints: Vec<usize> = data
        .trim_end()
        .split(",")
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    Ok(Intcode { data: ints, pos: 0 })
}

pub fn run_with_param(code: &Intcode, noun: usize, verb: usize) -> usize {
    let mut new_data = code.data.clone();
    new_data[1] = noun;
    new_data[2] = verb;
    let input = Intcode {
        data: new_data,
        pos: 0
    };
    let output = run(input);
    output.data[0]
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let target = 19690720;
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let r = run_with_param(&input, noun, verb);
            if r == target {
                println!("FOUND IT: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
    Ok(())
}
