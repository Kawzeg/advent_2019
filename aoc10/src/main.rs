use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn has_los(asteroids: &Vec<(i32, i32)>, a: (i32, i32), b: (i32, i32)) -> bool {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    if dx == 0 {
        let y1 = min(a.1, b.1);
        let y2 = max(a.1, b.1);
        for y in y1 + 1..y2 {
            let c = (a.0, y);
            if asteroids.contains(&c) {
                return false;
            }
        }
        true
    } else if dy == 0 {
        let x1 = min(a.0, b.0);
        let x2 = max(a.0, b.0);
        for x in x1 + 1..x2 {
            let c = (x, a.1);
            if asteroids.contains(&c) {
                return false;
            }
        }
        true
    } else {
        let factor = gcd(dx, dy).abs();
        if factor == 1 {
            true
        } else {
            let ddx = dx / factor;
            let ddy = dy / factor;
            println!("Checking {} between {:?} and {:?}", factor, a, b);
            println!("ddx={}, ddy={}", ddx, ddy);
            for i in 1..factor {
                let c = (a.0 + ddx * i, a.1 + ddy * i);
                println!("Checking {:?}", c);
                if asteroids.contains(&c) {
                    return false;
                }
            }
            true
        }
    }
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut asteroids: Vec<bool> = vec![false; width * height];
    let mut indices: Vec<usize> = vec![];
    let mut coords: Vec<(i32, i32)> = vec![];
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    for line in lines {
        for c in line.chars() {
            if c == '#' {
                asteroids[i] = true;
                indices.push(i);
                coords.push((x, y));
            } else {
                asteroids[i] = false;
            }
            i += 1;
            x += 1;
        }
        y += 1;
        x = 0;
    }
    println!("Asteroids: {:?}", &coords[..10]);
    println!("has_los {:?}", has_los(&coords, coords[0], coords[1]));
    println!("has_los {:?}", has_los(&coords, coords[0], coords[2]));
    let mut los_map: HashMap<(i32, i32), i32> = HashMap::new();
    for a in &coords {
        for b in &coords {
            if a == b {
                continue;
            }
            if has_los(&coords, *a, *b) {
                *los_map.entry(*a).or_insert(0) += 1;
            }
        }
    }
    let mut best_score= 0;
    let mut best_c = (0,0);

    for (c, score) in los_map {
        if score > best_score {
            best_c = c;
            best_score = score
        }
    }

    println!("Best asteroid: {:?} at {:?}", best_score, best_c);
    Ok(())
}
