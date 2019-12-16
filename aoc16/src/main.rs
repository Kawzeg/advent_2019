use std::io;

fn sum_pattern(signal: &Vec<i8>, pattern: &Vec<i8>) -> i8 {
    let mut sum: i32 = 0;
    for i in 0..signal.len() {
        let p = pattern[i];
        let x = signal[i];
        sum += p as i32 * x as i32;
    }
    sum = sum.abs() % 10;
    sum as i8
}

fn make_pattern(i: usize, len: usize) -> Vec<i8> {
    let mut r = vec![];
    let base = [0, 1, 0, -1];
    let mut j = 0;
    let mut k = 0;
    while j < len + 1 {
        let x = base[k % base.len()];
        k += 1;
        for _ in 0..i {
            r.push(x);
            j += 1;
        }
    }
    r.remove(0);
    r
}

fn phase(signal: &Vec<i8>) -> Vec<i8> {
    let mut r = vec![];
    for i in 0..signal.len() {
        let pattern = make_pattern(i + 1, signal.len());
        r.push(sum_pattern(signal, &pattern));
    }
    r
}

fn fft(signal: Vec<i8>, num_phases: u32) -> Vec<i8> {
    let mut r = signal;
    for i in 0..num_phases {
        r = phase(&r);
        println!("Phase {}: {:?}", i, r);
    }
    r
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    //let input = "12345678";
    let signal: Vec<i8> = input
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    println!("{:?}", signal);
    println!("Pattern 1: {:?}", make_pattern(1, 10));
    println!("Pattern 2: {:?}", make_pattern(2, 10));
    println!("Phase 1: {:?}", phase(&signal));
    println!("Phase 100: {:?}", fft(signal, 100));
    Ok(())
}
