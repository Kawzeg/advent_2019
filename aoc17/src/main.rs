use intcode::{intcode_from_file, run_with_io};
use std::io;

use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    Space,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Scaffold,
    Space,
    Robot(Direction),
}

#[derive(Clone)]
struct Map<T: Copy> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Copy> Map<T> {
    fn index_xy(&self, i: usize) -> (usize, usize) {
        let x = i % self.width;
        let y = i / self.width;
        (x, y)
    }
    fn xy_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn get_at(&self, x: usize, y: usize) -> T {
        self.tiles[self.xy_index(x, y)]
    }
}

fn parse_map(output: Vec<i64>) -> Map<TileType> {
    let mut width = 0;
    for x in &output {
        if *x == 10 {
            break;
        }
        width += 1;
    }
    let height = output.len() / (width + 1);
    let mut tiles = vec![];
    for tile in output {
        let t = match tile {
            35 => TileType::Scaffold,
            46 => TileType::Space,
            60 => TileType::Robot(Direction::West),
            62 => TileType::Robot(Direction::East),
            88 => TileType::Robot(Direction::Space),
            118 => TileType::Robot(Direction::South),
            94 => TileType::Robot(Direction::North),
            _ => continue,
        };
        tiles.push(t);
    }
    Map {
        tiles: tiles,
        width: width,
        height: height,
    }
}

fn get_neighbours(map: &Map<TileType>, x: usize, y: usize) -> Vec<TileType> {
    let mut r = vec![];
    r.push(map.get_at(x-1, y));
    r.push(map.get_at(x+1, y));
    r.push(map.get_at(x, y-1));
    r.push(map.get_at(x, y+1));
    r
}

fn is_intersection(map: &Map<TileType>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x >= map.width-1 || y >= map.height-1 || map.get_at(x,y) != TileType::Scaffold {
        false
    } else {
        let neighbours = get_neighbours(map, x, y);
        neighbours.iter().all(|x| *x == TileType::Scaffold)
    }
}

fn get_intersections(map: &Map<TileType>) -> Vec<(usize, usize)> {
    let mut r = vec![];
    for x in 0..map.width {
        for y in 0..map.width {
            if is_intersection(map, x, y) {
                r.push((x, y));
            }
        }
    }
    r
}

fn part1() -> io::Result<()> {
    let code = intcode_from_file("./resources/input")?;
    let out = run_with_io(&code, vec![]);
    for x in &out.output {
        print!("{}", std::char::from_u32(*x as u32).unwrap().to_string());
    }
    let map = parse_map(out.output);
    for y in 0..map.height {
        for x in 0..map.width {
            let mut c = match map.tiles[map.xy_index(x, y)] {
                TileType::Scaffold => "#",
                TileType::Space => ".",
                TileType::Robot(Direction::North) => "^",
                TileType::Robot(Direction::East) => ">",
                TileType::Robot(Direction::South) => "v",
                TileType::Robot(Direction::West) => "<",
                TileType::Robot(Direction::Space) => "X",
            };
            print!("{}", c);
        }
        println!();
    }
    let mut sum = 0;
    let intersections = get_intersections(&map);
    println!("Found {} intersections", intersections.len());
    for intersection in intersections {
        println!("Intersetion at: {:?}", intersection);
        sum += intersection.0 * intersection.1;
    }
    println!("Alignment Parameters: {}", sum);
    println!("Hello, world!");
    Ok(())
}

fn add_sequence(input: &mut Vec<i64>, seq: &str) {
    for c in seq.as_bytes() {
        input.push(*c as i64);
    }
    input.push('\n' as i64); // Newline
}

fn main() -> io::Result<()> {
    let mut code = intcode_from_file("./resources/input")?;
    code.data[0] = 2;
    let out = run_with_io(&code, vec![]);
    for x in &out.output {
        print!("{}", std::char::from_u32(*x as u32).unwrap().to_string());
    }
    let path = "L,12,L,8,R,10,R,10,L,6,L,4,L,12,L,12,L,8,R,10,R,10,L,6,L,4,L,12,R,10,L,8,L,4,R,10,L,6,L,4,L,12,L,12,L,8,R,10,R,10,R,10,L,8,L,4,R,10,L,6,L,4,L,12,R,10,L,8,L,4,R,10";
    println!("Path should be: {}", path);
    let mainseq = "A,B,A,B,C,B,A,C,B,C";
    let a = "L,12,L,8,R,10,R,10";
    let b = "L,6,L,4,L,12";
    let c = "R,10,L,8,L,4,R,10";

    let mut input: Vec<i64> = vec![];
    add_sequence(&mut input, mainseq);
    add_sequence(&mut input, a);
    add_sequence(&mut input, b);
    add_sequence(&mut input, c);
    let video_feed = 'n'; // 'y' or 'n' to select interactive video feed or not
    input.push(video_feed as i64);
    input.push('\n' as i64);
    let out = run_with_io(&code, input);
    let mut y = 0;
    let mut reached_video = false;
    for c in out.output {
        if c == 10 {
            y += 1;
            if y > 55 && !reached_video {
                reached_video = true;
                y = 0;
            }
            if video_feed == 'y' && reached_video && y % 50 == 0 {
                pause();
            }
        }
        if c > 256 {
            println!("Space dust collected: {}", c);
            break;
        }
        print!("{}", std::char::from_u32(c as u32).unwrap().to_string());
    }
    Ok(())
}