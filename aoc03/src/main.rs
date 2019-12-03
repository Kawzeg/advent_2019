use std::pin::Pin;

use std::cmp::{max, min};
use std::io;

type Visited = Vec<(i32, i32)>;

fn add_left(visited: &mut Visited, num: i32) {
    let cur = visited.last().unwrap();
    let x = cur.0;
    let y = cur.1;
    for x1 in (x - num..x).rev() {
        visited.push((x1, y));
    }
}

fn add_right(visited: &mut Visited, num: i32) {
    let cur = visited.last().unwrap();
    let x = cur.0;
    let y = cur.1;
    for x1 in x..x + num {
        visited.push((x1 + 1, y));
    }
}

fn add_up(visited: &mut Visited, num: i32) {
    let cur = visited.last().unwrap();
    let x = cur.0;
    let y = cur.1;
    for y1 in y..y + num {
        visited.push((x, y1 + 1));
    }
}

fn add_down(visited: &mut Visited, num: i32) {
    let cur = visited.last().unwrap();
    let x = cur.0;
    let y = cur.1;
    for y1 in (y - num..y).rev() {
        visited.push((x, y1));
    }
}

fn add_to(visited: &mut Visited, op: &str) {
    let dir = op.chars().next().unwrap();
    let slice = &op[1..];
    let num: i32 = slice.parse().unwrap();
    match dir {
        'L' => add_left(visited, num),
        'R' => add_right(visited, num),
        'U' => add_up(visited, num),
        'D' => add_down(visited, num),
        _ => {}
    }
}

fn run_through(wire: &Vec<&str>) -> Visited {
    let mut result = vec![];
    result.push((0, 0));
    for op in wire {
        add_to(&mut result, op);
    }
    result
}

fn bounds(visited: &Visited) -> (i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (x, y) in visited {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn add_wire(field: &mut Vec<Vec<u8>>, v: &Visited, origin: (i32, i32), wire: u8) {
    for p in v {
        let x = (p.0 + origin.0) as usize;
        let y = (p.1 + origin.1) as usize;
        if field[x][y] >> wire % 2 == 0 {
            field[x][y] += 1 << wire;
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Spiral {
    origin: (i32, i32),
    curr: (i32, i32),
    size: i32,
}
impl Iterator for Spiral {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == (self.origin.0, self.origin.1 + self.size) {
            self.size += 1;
            let prev = self.curr;
            self.curr = (self.curr.0 + 1, self.curr.1);
            Some(prev)
        } else {
            let prev = self.curr;
            let xo = self.origin.0;
            let yo = self.origin.1;
            let x = prev.0;
            let y = prev.1;

            if x >= xo && y > yo { // Go lower right
                self.curr = (prev.0+1, prev.1-1);
            } else if x > xo && y <= yo { // Go lower left
                self.curr = (prev.0-1, prev.1-1);
            } else if x <= xo && y < yo { // Go upper left
                self.curr = (prev.0-1, prev.1+1);
            } else if x < xo && y >= yo { // Go upper right
                self.curr = (prev.0+1, prev.1+1);
            }
            Some(prev)
        }
    }
}

fn spiral(origin: (i32, i32)) -> Spiral {
    Spiral {
        origin: origin,
        curr: origin,
        size: 0
    }
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    //let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"; //works
    //let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"; // works
    //let input = "R8,U5,L5,D3\nU7,R6,D4,L4"; // works
    let wires: Vec<&str> = input.split("\n").collect();
    let wire0: Vec<&str> = wires[0].split(",").collect();
    let wire1: Vec<&str> = wires[1].split(",").collect();
    let v1 = run_through(&wire0);
    let v2 = run_through(&wire1);
    let bounds1 = bounds(&v1);
    let bounds2 = bounds(&v2);
    println!("Bounds 1: {:?}", bounds1);
    println!("Bounds 2: {:?}", bounds2);
    let min_x = min(bounds1.0, bounds2.0);
    let min_y = min(bounds1.1, bounds2.1);
    let max_x = max(bounds1.2, bounds2.2);
    let max_y = max(bounds1.3, bounds2.3);
    let width: usize = (max_x - min_x) as usize;
    let height: usize = (max_y - min_y) as usize;
    let mut field: Vec<Vec<u8>> = vec![vec![0; height + 1]; width + 1];

    let origin1 = (bounds1.0.abs(), bounds1.1.abs());
    let origin2 = (bounds2.0.abs(), bounds2.1.abs());
    let origin = (max(origin1.0, origin2.0), max(origin1.1, origin2.1));
    println!("1: {:?} 2: {:?}", origin1, origin2);
    println!("Origin: {:?}", origin);

    println!("At origin: {}", field[origin.0 as usize][origin.1 as usize]);
    add_wire(&mut field, &v1, origin, 0);
    println!("At origin: {}", field[origin.0 as usize][origin.1 as usize]);
    add_wire(&mut field, &v2, origin, 1);
    println!("At origin: {}", field[origin.0 as usize][origin.1 as usize]);

    let mut s = spiral(origin);
    s.next(); // skip origin
    for p in s {
        let x = p.0 as usize;
        let y = p.1 as usize;
        if x >= width || y >= height {continue;}
        //println!("At {:?}: {}", p, field[p.0 as usize][p.1 as usize]);
        if field[x][y] > 2 {
            println!("Found it at {:?}", p);
            println!("Distance is {}", p.0+p.1-origin.0-origin.1);
            break;
        }
    }

    Ok(())
}
