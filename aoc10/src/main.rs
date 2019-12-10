#![feature(vec_remove_item)]
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::io;
use std::fmt;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

#[derive(PartialEq, Eq)]
struct Target {
    origin: (i32, i32),
    target: (i32, i32),
}
impl fmt::Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Target ({}, {})", self.target.0, self.target.1)
    }
}
impl Ord for Target {
    fn cmp(&self, other: &Self) -> Ordering {
        let dx1 = self.target.0 - self.origin.0;
        let dx2 = other.target.0 - other.origin.0;
        let dy1 = self.target.1 - self.origin.1;
        let dy2 = other.target.1 - other.origin.1;
        let dist1 = distance(self.target, self.origin);
        let dist2 = distance(other.target, other.origin);
        let mut norm_y1 = dy1 as f64/ dist1 as f64;
        let mut norm_y2 = dy2 as f64 / dist2 as f64;
        let mut norm_x1 = dx1 as f64 / dist1 as f64;
        let mut norm_x2 = dx2 as f64 / dist2 as f64;
        if dx1 < 0 {
            norm_y1 = 2. - norm_y1;
        }
        if dx2 < 0 {
            norm_y2 = 2. - norm_y2;
        }
        if norm_y1 < norm_y2 {
            //println!("{:?} < {:?}", self, other);
            Ordering::Less
        } else if norm_y1 == norm_y2 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
            for i in 1..factor {
                let c = (a.0 + ddx * i, a.1 + ddy * i);
                if asteroids.contains(&c) {
                    return false;
                }
            }
            true
        }
    }
}

fn make_los_vector(coords: &Vec<(i32, i32)>, a: (i32, i32)) -> Vec<(i32, i32)> {
    let mut los_vector = vec![];
    for b in coords {
            if a == *b {
                continue;
            }
            if has_los(coords, a, *b) {
                los_vector.push(*b);
            }
        }
    los_vector
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
    println!("Asteroids: {:?}", &coords[..20]);
    println!("has_los {:?}", has_los(&coords, coords[0], coords[1]));
    println!("has_los {:?}", has_los(&coords, coords[0], coords[2]));
    let mut los_map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for a in &coords {
        los_map.insert(*a, make_los_vector(&coords, *a));
    }
    let mut best_score= 0;
    let mut best_c = (0,0);

    for (c, score) in &los_map {
        if score.len() > best_score {
            best_c = *c;
            best_score = score.len()
        }
    }

    println!("Best asteroid: {:?} at {:?}", best_score, best_c);

    let laser_origin = best_c; // (30, 34);
    let mut targets: Vec<Target> = los_map.get(&laser_origin).unwrap().iter().map(|c|{Target{
        origin: laser_origin,
        target: *c,
    }}).collect();

    let mut shots: Vec<Target> = vec![];
    while coords.len() > 1 {
        let mut targets: Vec<Target> = make_los_vector(&coords, laser_origin).iter().map(|c|{Target{
            origin: laser_origin,
            target: *c,
        }}).collect();
        targets.sort();
        for debris in &targets {
            coords.remove_item(&debris.target);
        }
        println!("Shooting at {} targets: {:?}", targets.len(), targets);
        shots.extend(targets);
    }

    println!("Shots: {:?}", shots);
    println!("1st: {:?}", shots[0]);
    println!("2nd: {:?}", shots[1]);
    println!("3rd: {:?}", shots[2]);
    println!("10th: {:?}", shots[9]);
    println!("20th: {:?}", shots[19]);
    println!("50th: {:?}", shots[49]);
    println!("100th: {:?}", shots[99]);
    println!("199th: {:?}", shots[198]);
    println!("200th: {:?}", shots[199]);


    Ok(())
}
