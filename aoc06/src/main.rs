use std::io;
use std::collections::HashMap;

fn count_orbits(orbits: &HashMap<&str, &str>, start: &str) -> i32{
    if start == "COM" {
        0
    } else {
        1 + count_orbits(orbits, orbits.get(start).unwrap())
    }
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut orbits : HashMap<&str, &str> = HashMap::new();
    for orbit in lines {
        println!("Orbit is {}", orbit);
        let split : Vec<&str> = orbit.split(")").collect();
        let orbitee = split[0];
        let orbiter = split[1];
        orbits.insert(orbiter, orbitee);
    }
    println!("TFG has {} orbits", count_orbits(&orbits, "TFG"));
    println!("Found {} orbits", orbits.len());

    let mut total_orbits = 0;
    for orbit in orbits.keys() {
        total_orbits += count_orbits(&orbits, orbit);
    }
    println!("Total orbits: {}", total_orbits);
    Ok(())
}
