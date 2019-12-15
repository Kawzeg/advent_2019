use intcode::{intcode_from_file, run_with_io, Intcode};
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


const WIDTH: usize = 45;
const HEIGHT: usize = 45;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn dir_to_int(d: Direction) -> i64 {
    match d {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

#[derive(Debug)]
enum Response {
    Wall,
    Moved,
    FoundSystem,
}

fn int_to_res(r: i64) -> Response {
    match r {
        0 => Response::Wall,
        1 => Response::Moved,
        2 => Response::FoundSystem,
        _ => panic!("Unknown response {}", r),
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TileType {
    Floor,
    Wall,
    OxygenSystem,
    Unknown,
}

#[derive(Clone)]
struct Map<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}
impl<T> Map<T> {
    fn index_xy(&self, i: usize) -> (usize, usize) {
        let x = i % self.width;
        let y = i / self.width;
        (x, y)
    }
    fn xy_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn step(code: &Intcode, dir: Direction) -> (Response, Intcode) {
    let input = dir_to_int(dir);
    let mut outcode = run_with_io(code, vec![input]);
    let response = int_to_res(outcode.output.pop().unwrap());
    (response, outcode)
}

#[derive(Debug)]
struct Neighbours {
    north: Option<(i64, usize)>,
    south: Option<(i64, usize)>,
    east: Option<(i64, usize)>,
    west: Option<(i64, usize)>,
}

fn get_neighbours(map: &Map<i64>, i: usize) -> Neighbours {
    let (x, y) = map.index_xy(i);
    let mut r = Neighbours {
        south: None,
        north: None,
        east: None,
        west: None,
    };
    if x > 0 {
        let wi = map.xy_index(x - 1, y);
        let w = map.tiles[wi];
        r.west = Some((w, wi));
    }
    if x < map.width - 1 {
        let ei = map.xy_index(x + 1, y);
        let e = map.tiles[ei];
        r.east = Some((e, ei));
    }
    if y > 0 {
        let ni = map.xy_index(x, y - 1);
        let n = map.tiles[ni];
        r.north = Some((n, ni));
    }
    if y < map.height - 1 {
        let si = map.xy_index(x, y + 1);
        let s = map.tiles[si];
        r.south = Some((s, si));
    }
    r
}

fn get_neighbour_list(n: Neighbours) -> Vec<(i64, usize)> {
    let n_vec = vec![n.north, n.south, n.east, n.west];
    n_vec
        .iter()
        .filter_map(|x| match x {
            Some((v, i)) => Some((*v, *i)),
            None => None,
        }).collect()
}

fn get_min_neighour(map: &Map<i64>, i: usize) -> (i64, usize) {
    let n = get_neighbours(map, i);
    let n_vec = vec![n.north, n.south, n.east, n.west];
    n_vec
        .iter()
        .filter_map(|x| match x {
            Some((v, i)) => Some((*v, *i)),
            None => None,
        })
        .min()
        .unwrap()
}

fn iterate_dijk_step(map: &Map<TileType>, dijk_map: &Map<i64>, dirties: Vec<usize>) -> (i64, Map<i64>, Vec<usize>) {
    let mut r = 0;
    let mut new_dijk = dijk_map.clone();
    let mut new_dirties = vec![];
    for i in dirties {
        let neighbours = get_neighbours(dijk_map, i);
        let ns = get_neighbour_list(neighbours);
        for (_, n_i) in ns {
            if map.tiles[n_i] == TileType::Wall {
                continue;
            }
            if dijk_map.tiles[n_i] > dijk_map.tiles[i] + 1 {
                new_dijk.tiles[n_i] = dijk_map.tiles[i] + 1;
                new_dirties.push(n_i);
                r += 1;
            }
        }
    }
    (r, new_dijk, new_dirties)
}

fn build_dijk_step(map: &Map<TileType>, dijk: &Map<i64>) -> (i64, Map<i64>, Vec<usize>) {
    let mut r = 0;
    let mut new_dijk = dijk.clone();
    let mut dirties = vec![];
    for (i, tile) in map.tiles.iter().enumerate() {
        if *tile == TileType::Wall {
            continue;
        }
        let min = get_min_neighour(dijk, i).0;
        if min < dijk.tiles[i] - 1 {
            dirties.push(i);
            new_dijk.tiles[i] = min + 1;
            r += 1;
        }
    }
    //println!("Changed {} tiles", r);
    (r, new_dijk, dirties)
}

fn build_map(map: &Map<TileType>, target: TileType) -> Map<i64> {
    let dijk_tiles: Vec<i64> = vec![std::i64::MAX; map.tiles.len()];
    let mut dijk_map = Map {
        width: map.width,
        height: map.height,
        tiles: dijk_tiles,
    };
    for (i, tile) in map.tiles.iter().enumerate() {
        if *tile == target {
            dijk_map.tiles[i] = 0;
        }
    }
    let mut i = 1;
    let x = build_dijk_step(map, &dijk_map);
    let mut changed = x.0;
    let mut dijk_map = x.1;
    let mut dirties = x.2;
    while changed != 0 {
        let x = iterate_dijk_step(&map, &dijk_map, dirties);
        changed = x.0;
        dijk_map = x.1;
        dirties = x.2;
        i += 1;
        //println!("Changed {}", changed);
    }
    //println!("Took {} iterations", i);
    dijk_map
}

fn display(map: &Map<TileType>, r_x: usize, r_y: usize) {
    for y in 0..map.height {
        for x in 0..map.width {
            let mut c = match map.tiles[y*map.width + x] {
                TileType::Floor => ".",
                TileType::Unknown => " ",
                TileType::Wall => "#",
                TileType::OxygenSystem => "O",
            };
            if x == r_x && y == r_y {
                c = "■";
            }
            print!("{}", c);
        }
        println!();
    }
}

fn explore(map: &mut Map<TileType>, code: &Intcode, x: usize, y: usize) {
    let mut code = code.clone();
    let (mut x, mut y) = (x, y);
    let mut dirty = false;
    let mut dijk_map = build_map(&map, TileType::Unknown);
    loop {
        if dirty {
            dijk_map = build_map(&map, TileType::Unknown);
        }
        if dijk_map.tiles[dijk_map.xy_index(x, y)] == std::i64::MAX {
            return;
        }
        let min_neighbour = get_min_neighour(&dijk_map, map.xy_index(x, y));
        let new_i = min_neighbour.1;
        let dir = match new_i as isize - map.xy_index(x, y) as isize {
            -1 => Direction::West,
            1 => Direction::East,
            i if i > 1 => Direction::South,
            i if i < -1 => Direction::North,
            _ => panic!("Shouldn't be possible"),
        };
        if map.tiles[new_i] == TileType::Unknown {
            dirty = true;
        } else {
            dirty = false;
        }
        //println!("Stepping towards {:?}", dir);
        let r = step(&code, dir);
        //println!("Robot says {:?}", r.0);
        match r.0 {
            Response::Wall => {
                map.tiles[new_i] = TileType::Wall;
            }
            Response::Moved => {
                map.tiles[new_i] = TileType::Floor;
                let c = map.index_xy(new_i);
                x = c.0;
                y = c.1;
            }
            Response::FoundSystem => {
                map.tiles[new_i] = TileType::OxygenSystem;
                let c = map.index_xy(new_i);
                x = c.0;
                y = c.1;
            }
        };
        code = r.1;
        //pause();
        //println!("Neighbours are: {:?}", get_neighbours(&dijk_map, dijk_map.xy_index(x, y)));
        display(&map, x, y);
    }
}

fn main() -> io::Result<()> {
    let code = intcode_from_file("./resources/input")?;
    let mut map = Map {
        width: WIDTH,
        height: HEIGHT,
        tiles: vec![TileType::Unknown; WIDTH * HEIGHT],
    };
    let x = map.width / 2;
    let y = map.height / 2;
    let middle = map.xy_index(x, y);
    map.tiles[middle] = TileType::Floor;
    display(&map, x, y);
    explore(&mut map, &code, x, y);
    let goal_map = build_map(&map, TileType::OxygenSystem);
    let distance = goal_map.tiles[middle];
    display(&map, x, y);
    println!("Distance is: {}", distance);
    Ok(())
}
