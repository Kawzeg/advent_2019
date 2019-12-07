use intcode::{intcode_from_file, run_with_io, Intcode};
use std::io;

fn run_amplifier(code: &Intcode, phase_setting: i64, input: i64) -> i64 {
    run_with_io(code, vec![phase_setting, input])
        .output
        .pop()
        .unwrap()
}

fn run_amplifiers(code: &Intcode, phases: Vec<i64>) -> i64 {
    let mut input = 0;
    for i in phases {
        input = run_amplifier(code, i, input);
    }
    input
}

fn run_amplifier_cont(code: &Intcode, input: i64) -> Intcode {
    run_with_io(code, vec![input])
}

fn run_feedback(code: &Intcode, phases: &Vec<i64>) -> i64 {
    let mut input = 0;
    let mut amps: Vec<Intcode> = vec![code.clone(), code.clone(), code.clone(), code.clone(), code.clone()];
    let mut output = 0;
    for i in 0..5 {
        amps[i] = run_amplifier_cont(&amps[i], phases[i]);
    }
    'outer: loop {
        for i in 0..5 {
            let mut amp = run_amplifier_cont(&amps[i], input);
            let out = amp.output.pop();
            if code.halted {
                break 'outer;
            }
            match out {
                None => break 'outer,
                Some(x) => input = x,
            }
            if i == 4 {
                output = out.unwrap();
            }
            amps[i] = amp;
        }
        println!("After loop, it's {}", output);
    }
    return output;
}

fn permutations(phases: Vec<i64>) -> Vec<Vec<i64>> {
    if phases.len() == 0 {
        vec![]
    } else if phases.len() == 1 {
        vec![phases]
    } else {
        let mut mutations: Vec<Vec<i64>> = vec![];
        for (i, pin) in phases.iter().enumerate() {
            let mut tail = phases.clone();
            tail.remove(i);
            for m in permutations(tail) {
                let mut new_mut = vec![*pin];
                new_mut.extend(m);
                mutations.push(new_mut);
            }
        }
        mutations
    }
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let output = run_amplifier(&input, 0, 0);
    println!("Output is {:?}", output);
    let mut max_output = 0;

    let perms = permutations(vec![5, 6, 7, 8, 9]);
    let mut best_phases: Vec<i64> = vec![];

    for p in &perms {
        let output = run_feedback(&input, p);
        if output > max_output {
            max_output = output;
            best_phases = p.clone();
        }
    }

    println!(
        "Maximum output is: {}, with phases {:?}",
        max_output, best_phases
    );
    Ok(())
}
