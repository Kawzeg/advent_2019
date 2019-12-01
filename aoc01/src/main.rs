use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn total_fuel(mass: i64) -> i64 {
    let f = fuel(mass);
    if f <= 0 {
        0
    } else {
        f + total_fuel(f)
    }
}

fn fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn main() -> io::Result<()> {
    let file = File::open("./resources/input")?;
    let reader = BufReader::new(file);

    let mut modules: Vec<i64> = vec![];

    for line in reader.lines() {
        modules.push(line?.to_string().parse::<i64>().unwrap());
    }
    let masses: Vec<i64> = modules.into_iter().map(total_fuel).collect();
    println!("fuel {}", fuel(14));
    println!("total_fuel {}", total_fuel(14));
    println!("{}", masses.iter().sum::<i64>());
    Ok(())
}
