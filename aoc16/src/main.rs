use std::io;

fn sum_pattern(signal: &Vec<i32>, pattern: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..signal.len() {
        let p = pattern[i];
        let x = signal[i];
        sum += p as i32 * x as i32;
    }
    sum = sum.abs() % 10;
    sum as i32
}

fn make_pattern(i: usize, len: usize) -> Vec<i32> {
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
    r.split_at(len).0.to_vec()
}

fn phase(signal: &Vec<i32>) -> Vec<i32> {
    let mut r = vec![];
    for i in 0..signal.len() {
        if i % 10000 == 0 {
            println!("Status: {}", i);
        }
        let pattern = make_pattern(i + 1, signal.len());
        r.push(sum_pattern(signal, &pattern));
    }
    r
}

fn fft(signal: Vec<i32>, num_phases: u32) -> Vec<i32> {
    let mut r = signal;
    for i in 0..num_phases {
        r = phase(&r);
        println!("Phase {}: {:?}", i, r);
    }
    r
}

fn quick_phase(signal: &mut Vec<i32>) -> Vec<i32> {
    signal.reverse();
    let mut result = vec![];
    let mut sum = 0;
    for x in signal {
        sum += *x;
        result.push(sum % 10);
    }
    result.reverse();
    result
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    //let input = "03036732577212944063491565474664";
    let signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    let mut real_signal = vec![];
    for _ in 0..10000 {
        real_signal.append(&mut signal.clone());
    }
    println!("Length: {:?}", real_signal.len());
    let offset = signal[6] + signal[5] * 10 + signal[4] * 100 + signal[3] * 1000 + signal[2] * 10000 + signal[1] * 100000 + signal[0] * 1000000;
    println!("Offset: {}", offset);

    let mut vec = vec![];
    for i in offset as usize..real_signal.len() {
        vec.push(real_signal[i]);
    }

    println!("Offset vec: {:?}", &vec[0..8]);
    for _ in 0..100 {
        vec = quick_phase(&mut vec);
    }
    println!("Quick phase: {:?}", &vec[0..8]);

    print!("Result is: ");
    for i in 0..8 {
        print!("{}", vec[i]);
    }
    println!();

    Ok(())
}
