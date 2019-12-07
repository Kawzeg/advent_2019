use intcode::{intcode_from_file, run_with_io, Intcode};
use std::io;

fn run_amplifier(code: &Intcode, phase_setting: i32, input: i32) -> i32 {
    run_with_io(code, vec![phase_setting, input]).pop().unwrap()
}

fn run_amplifiers(code: &Intcode, phases: Vec<i32>) -> i32 {
    let mut input = 0;
    for i in phases {
        input = run_amplifier(code, i, input);
    }
    input
}

fn permutations(phases: Vec<i32>) -> Vec<Vec<i32>> {
    if phases.len() == 0 {
        vec![]
    } else if phases.len() == 1 {
        vec![phases]
    } else {
        let mut mutations: Vec<Vec<i32>> = vec![];
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
    let phases: Vec<&i32> = vec![&0, &1, &2, &3, &4];
    let mut best_phases: Vec<&i32> = vec![];
    let mut max_output = 0;

    'outer: for a in &phases {
        for b in &phases {
            for c in &phases {
                for d in &phases {
                    for e in &phases {
                        if a == b
                            || a == c
                            || a == d
                            || a == e
                            || b == c
                            || b == d
                            || b == e
                            || c == d
                            || c == e
                            || d == e
                        {
                            continue;
                        }
                        let output = run_amplifiers(&input, vec![**a, **b,* *c, **d, **e]);
                        if output > max_output {
                            max_output = output;
                            best_phases = vec![a, b, c, d, e];
                        }
                    }
                }
            }
        }
    }

    println!("Permutations {:?}", permutations(vec![0,1,2,3,4,5,6,7,8,9]).len());
    println!(
        "Maximum output is: {}, with phases {:?}",
        max_output, best_phases
    );
    Ok(())
}
