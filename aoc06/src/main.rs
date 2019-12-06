use std::io;
use std::collections::HashMap;

fn count_orbits(orbits: &HashMap<&str, &str>, start: &str) -> i32 {
    if start == "COM" {
        0
    } else {
        1 + count_orbits(orbits, orbits.get(start).unwrap())
    }
}

fn all_orbits(orbits: &HashMap<&str, &str>, start: &str) -> Vec<String> {
    if start == "COM" {
        vec!["COM".to_string()]
    } else {
        let mut partial = all_orbits(orbits, orbits.get(start).unwrap());
        partial.push(start.to_string());
        partial
    }
}

fn find_root(start_orbits: &Vec<String>, end_orbits: &Vec<String>) -> String {
    let mut last_common = &start_orbits[0];
    for (a, b) in start_orbits.iter().zip(end_orbits.iter()) {
        if a == b {
            last_common = a;
        } else {
            break;
        }
    }
    last_common.to_string()
}

fn orbits_to_root(orbits: &HashMap<&str, &str>, start: &str, root: &str) -> i32{
    if start == root {
        0
    } else {
        1 + orbits_to_root(orbits, orbits.get(start).unwrap(), root)
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
    println!("Found {} orbits", orbits.len());

    let mut total_orbits = 0;
    for orbit in orbits.keys() {
        total_orbits += count_orbits(&orbits, orbit);
    }
    println!("Total orbits: {}", total_orbits);
    let YOU = all_orbits(&orbits, "YOU");
    let SAN = all_orbits(&orbits, "SAN");
    println!("YOUr orbits: {:?}", all_orbits(&orbits, "YOU"));
    let root = find_root(&YOU, &SAN);
    println!("Root: {}", root);
    let you_root = orbits_to_root(&orbits, "YOU", &root) - 1;
    let san_root = orbits_to_root(&orbits, "SAN", &root) - 1;
    println!("Jumps: {}", you_root + san_root);
    Ok(())
}
