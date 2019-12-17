use intcode::{intcode_from_file, run_with_io};
use std::io;

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

fn main() -> io::Result<()> {
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
