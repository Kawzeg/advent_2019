use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BinaryHeap;

const KEYS: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    keys: Vec<char>, // Collected keys
    i: usize, // Position in the map vector
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

fn is_wall(tile: char, keys: &Vec<char>) -> bool {
    if keys.contains(&tile.to_ascii_lowercase()) {
        false
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ#".contains(tile)
    }
}

fn neighbour_list(map: &Map<char>, i: usize, keys: &Vec<char>) -> Vec<usize> {
    let mut r = vec![];
    if i > map.width && !is_wall(map.tiles[i - map.width], keys){
        r.push(i - map.width);
    }
    if i < map.tiles.len() - map.width && !is_wall(map.tiles[i + map.width], keys) {
        r.push(i + map.width);
    }
    if i % map.width > 0 && !is_wall(map.tiles[i - 1], keys){
        r.push(i - 1);
    }
    if i % map.width < map.width - 1 && !is_wall(map.tiles[i + 1], keys){
        r.push(i + 1);
    }
    r
}

fn dijkstra(map: &Map<char>, start: char, target: char, keys: &Vec<char>) -> Option<i64> {
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
        let neighbours = neighbour_list(map, i, keys);
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

fn build_dijks(map: &Map<char>, keys: Vec<char>, collected_keys: &Vec<char>) -> Vec<(char, i64)> {
    let mut r = vec![];
    for key in keys {
        let d = dijkstra(map, '@', key, collected_keys);
        if let Some(distance) = d {
            r.push((key, distance));
        }
    }
    r
}

fn optimize(map: &Map<char>, state: &State, steps: i64) -> Vec<(State, i64)> {
    let mut r = vec![];
    let mut keys = vec![];
    for key in KEYS.chars() {
        if !state.keys.contains(&key) {
            keys.push(key);
        }
    }

    //println!("Building dijkstras");
    let distances = build_dijks(map, keys, &state.keys);
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
    let initial_state = State{keys: vec![], i: get_index(&map, '@').unwrap()};
    let mut states: HashMap<State, i64> = HashMap::new();
    states.insert(initial_state, 0);
    loop {
        let mut changed_states: Vec<(State, i64)> = vec![];
        for (state, steps) in &states {
            //states.remove(state);
            let new_states = optimize(&map, state, *steps);
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
