use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BinaryHeap;

const KEYS: &str = "abcdefghijklmnopqrstuvwxyz";
const DOORS: &str = "ABCDEFGHIJKLMNOPQRSTuVWXYZ";

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    keys: Vec<char>, // Collected keys
    i: usize, // Position in the map vector
    map: Map<char>, // Updated map with opened doors
}

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

fn is_wall(tile: char) -> bool {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ#".contains(tile)
}

fn iterate_dijk_step(map: &Map<char>, dijk_map: &mut Map<i64>, dirties: Vec<usize>) -> (i64, Vec<usize>) {
    let mut r = 0;
    let mut new_dirties = vec![];
    for i in dirties {
        let neighbours = get_neighbours(dijk_map, i);
        let ns = get_neighbour_list(neighbours);
        for (x, n_i) in ns {
            if is_wall(map.tiles[n_i]) {
                continue;
            }
            if x > dijk_map.tiles[i] + 1 {
                dijk_map.tiles[n_i] = dijk_map.tiles[i] + 1;
                new_dirties.push(n_i);
                r += 1;
            }
        }
    }
    (r, new_dirties)
}

fn build_dijk_step(map: &Map<char>, dijk: &Map<i64>) -> (i64, Map<i64>, Vec<usize>) {
    let mut r = 0;
    let mut new_dijk = dijk.clone();
    let mut dirties = vec![];
    for (i, tile) in map.tiles.iter().enumerate() {
        if is_wall(*tile) {
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

fn neighbour_list(map: &Map<char>, i: usize) -> Vec<usize> {
    let mut r = vec![];
    if i > map.width && !is_wall(map.tiles[i - map.width]){
        r.push(i - map.width);
    }
    if i < map.tiles.len() - map.width && !is_wall(map.tiles[i + map.width]) {
        r.push(i + map.width);
    }
    if i % map.width > 0 && !is_wall(map.tiles[i - 1]){
        r.push(i - 1);
    }
    if i % map.width < map.width - 1 && !is_wall(map.tiles[i + 1]){
        r.push(i + 1);
    }
    r
}

fn dijkstra(map: &Map<char>, start: char, target: char) -> Option<i64> {
    if let None = get_index(map, target) {
        return None;
    }
    let target_i = get_index(map, target).unwrap();
    let mut to_visit: BinaryHeap<(usize, i64)> = BinaryHeap::new();
    let mut distances = HashMap::new();
    let i = get_index(map, start).unwrap();
    to_visit.push((i, 0));
    distances.insert(i, 0);
    while let Some((i, cost)) = to_visit.pop() {
        let neighbours = neighbour_list(map, i);
        for n in neighbours {
            let new_distance = cost + 1;
            let is_shorter = distances.get(&n).map_or(true, |&current| new_distance < current);
            if is_shorter {
                distances.insert(n, new_distance);
                to_visit.push((n, new_distance));
            }
        }
        if let Some(d) = distances.get(&target_i) {
            return Some(*d);
        }
    }
    distances.get(&target_i).map(|x| *x)
}

fn build_map(map: &Map<char>, target: char) -> Map<i64> {
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
        let x = iterate_dijk_step(&map, &mut dijk_map, dirties);
        changed = x.0;
        dirties = x.1;
        i += 1;
        //println!("Changed {}", changed);
    }
    //println!("Took {} iterations", i);
    dijk_map
}

fn display(map: &Map<char>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let mut c = map.tiles[y*map.width + x];
            print!("{}", c);
        }
        println!();
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn parse_map(input: String) -> Map<char> {
    let width = input.find('\n').unwrap();
    let mut height = 0;
    let mut tiles = vec![];
    for c in input.chars() {
        if c =='\n' {
            height += 1;
            continue;
        }
        tiles.push(c);
    }
    Map {
        width: width,
        height: height,
        tiles: tiles
    }
}

fn get_position(map: &Map<char>, target: char) -> (usize, usize) {
    map.index_xy(map.tiles.iter().position(|x| *x == target).unwrap())
}

fn get_index(map: &Map<char>, target: char) -> Option<usize> {
    map.tiles.iter().position(|x| *x == target)
}

fn build_dijks(map: &Map<char>, keys: Vec<char>) -> Vec<(char, i64)> {
    let mut r = vec![];
    for key in keys {
        let d = dijkstra(map, '@', key);
        if let Some(distance) = d {
            r.push((key, distance));
        }
    }
    r
}

fn optimize(state: &State, steps: i64) -> Vec<(State, i64)> {
    let mut r = vec![];
    let map = &state.map;
    let mut keys = vec![];
    for key in KEYS.chars() {
        if !state.keys.contains(&key) {
            keys.push(key);
        }
    }

    //println!("Building dijkstras");
    let distances = build_dijks(map, keys);
    //println!("Keys: {:?}", distances);

    for (key, d) in distances {
        let new_index = get_index(map, key).unwrap();
        let mut new_keys = state.keys.clone();
        new_keys.push(key);
        new_keys.sort();
        let mut new_map = map.clone();
        let door = key.to_ascii_uppercase();
        let door_index = get_index(map, door).unwrap();
        new_map.tiles[new_index] = '@';
        new_map.tiles[door_index] = '.';
        new_map.tiles[state.i] = '.';
        let new_state = State {
            keys: new_keys,
            i: new_index,
            map: new_map,
        };
        r.push((new_state, steps+d));
    }
    //pause();

    r
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;

    let map = parse_map(input);
    display(&map);
    let initial_state = State{keys: vec![], i: get_index(&map, '@').unwrap(), map: map};
    let mut states: HashMap<State, i64> = HashMap::new();
    states.insert(initial_state, 0);
    loop {
        let mut changed_states: Vec<(State, i64)> = vec![];
        for (state, steps) in &states {
            //states.remove(state);
            let new_states = optimize(state, *steps);
            for (new_state, new_steps) in new_states {
                changed_states.push((new_state, new_steps));
            }
        }
        if changed_states.is_empty() {
            break;
        }
        states = HashMap::new();
        for (state, steps) in changed_states {
            let old = states.insert(state.clone(), steps);
            match old {
                Some(old_steps) => {
                    if old_steps < steps {
                        //println!("Keys: {:?} Not replacing {:?} with {:?}", state.keys, old_steps, steps);
                        //println!("Not the best way!");
                        states.insert(state, old_steps);
                    }
                }
                _ => {}
            }
        }
        //let display_values: Vec<Vec<char>> = states.keys().map(|x| x.keys.clone()).collect();
        println!("States: {:?}", states.len());
        //pause();
    }
    println!("DONE");
    let display_values: Vec<(Vec<char>, i64)> = states.keys().map(|x| (x.keys.clone(), *states.get(x).unwrap())).collect();
    println!("States: {:?}", display_values);
    println!("Hello, world!");
    Ok(())
}
