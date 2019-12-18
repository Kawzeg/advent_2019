use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

use std::cmp::Ordering;

const KEYS: &str = "abcdefghijklmnopqrstuvwxyz";
const DOORS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Eq, Clone)]
struct State {
    keys: Vec<char>, // Collected keys
    is: [usize; 4],  // Position in the map vector
}
impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        let mut self_sorted = self.keys.clone();
        let mut other_sorted = other.keys.clone();
        self_sorted.sort();
        other_sorted.sort();
        self_sorted == other_sorted && self.is == other.is
    }
}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted = self.keys.clone();
        sorted.sort();
        sorted.hash(state);
        self.is.hash(state);
    }
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

fn is_wall(tile: char, keys: &Vec<char>) -> bool {
    if tile == '#' {
        true
    } else if keys.contains(&tile.to_ascii_lowercase()) {
        false
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(tile)
    }
}

fn neighbour_list(map: &Map<char>, i: usize, keys: &Vec<char>) -> Vec<usize> {
    let mut r = vec![];
    if i > map.width && !is_wall(map.tiles[i - map.width], keys) {
        r.push(i - map.width);
    }
    if i < map.tiles.len() - map.width && !is_wall(map.tiles[i + map.width], keys) {
        r.push(i + map.width);
    }
    if i % map.width > 0 && !is_wall(map.tiles[i - 1], keys) {
        r.push(i - 1);
    }
    if i % map.width < map.width - 1 && !is_wall(map.tiles[i + 1], keys) {
        r.push(i + 1);
    }
    r
}

#[derive(Debug, PartialEq, Eq)]
struct Vertex<T>(i64, T);
impl<T> Ord for Vertex<T>
where
    T: PartialEq,
    T: Eq,
{
    fn cmp(&self, other: &Vertex<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl<T> PartialOrd for Vertex<T>
where
    T: PartialEq,
    T: Eq,
{
    fn partial_cmp(&self, other: &Vertex<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_dijkstra_map(map: &Map<char>, key: char) -> Map<(i64, Vec<char>)> {
    let mut to_visit: BinaryHeap<Vertex<usize>> = BinaryHeap::new();
    let mut visited: Vec<(i64, Vec<char>)> = vec![(std::i64::MAX, vec![]); map.tiles.len()];
    let mut distances = HashMap::new();
    let i = get_index(map, key).unwrap();
    to_visit.push(Vertex(0, i));
    distances.insert(i, 0);
    while let Some(Vertex(cost, i)) = to_visit.pop() {
        let neighbours = neighbour_list(map, i, &KEYS.chars().collect());
        for n in neighbours {
            let new_distance = cost + 1;
            let is_shorter = distances
                .get(&n)
                .map_or(true, |&current| new_distance < current);
            if is_shorter {
                distances.insert(n, new_distance);
                let tile = map.tiles[n];
                let mut new_doors = visited[i].1.clone();
                if DOORS.contains(tile) {
                    new_doors.push(tile);
                }
                visited[n] = (new_distance, new_doors);
                to_visit.push(Vertex(new_distance, n));
            }
        }
    }
    Map {
        width: map.width,
        height: map.height,
        tiles: visited,
    }
}

fn build_dijkstra_maps(map: &Map<char>, keys: Vec<char>) -> HashMap<char, Map<(i64, Vec<char>)>> {
    let mut r = HashMap::new();
    for key in keys {
        if let Some(_) = get_index(&map, key) {
            r.insert(key, build_dijkstra_map(map, key));
        }
    }
    r
}

fn display(map: &Map<char>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let mut c = map.tiles[y * map.width + x];
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
        if c == '\n' {
            height += 1;
            continue;
        }
        tiles.push(c);
    }
    Map {
        width: width,
        height: height,
        tiles: tiles,
    }
}

fn get_index(map: &Map<char>, target: char) -> Option<usize> {
    map.tiles.iter().position(|x| *x == target)
}

fn get_robots(map: &Map<char>) -> [usize; 4] {
    let mut r = [0; 4];
    let mut r_i = 0;
    for (i, &tile) in map.tiles.iter().enumerate() {
        if tile == '@' {
            r[r_i] = i;
            r_i += 1;
        }
    }
    r
}

fn is_reachable(
    map: &Map<char>,
    d_map: &Map<(i64, Vec<char>)>,
    start: usize,
    keys: &Vec<char>,
) -> bool {
    if d_map.tiles[start].0 == std::i64::MAX {
        return false
    }
    let needed_keys: Vec<char> = d_map.tiles[start]
        .1
        .iter()
        .map(|&x| x.to_ascii_lowercase())
        .collect();
    needed_keys.iter().all(|x| keys.contains(x))
}

fn fast_neighbours(
    map: &Map<char>,
    state: &State,
    d_maps: &HashMap<char, Map<(i64, Vec<char>)>>,
) -> Vec<(State, i64)> {
    let mut r = vec![];
    let mut keys = vec![];
    for key in KEYS.chars() {
        if !state.keys.contains(&key) {
            keys.push(key);
        }
    }

    for key in &keys {
        let distances = d_maps.get(&key).unwrap();
        for (i, robot) in state.is.iter().enumerate() {
            if is_reachable(&map, &distances, *robot, &state.keys) {
                let new_index = get_index(&map, *key).unwrap();
                let mut new_keys = state.keys.clone();
                new_keys.push(*key);
                let mut new_is = state.is;
                new_is[i] = new_index;
                let new_state = State {
                    keys: new_keys,
                    is: new_is,
                };
                r.push((new_state, distances.tiles[*robot].0));
                break;
            }
        }
    }
    r
}

fn main() -> io::Result<()> {
    let file = "./resources/input_2";
    let num_keys = 26;
    let input = std::fs::read_to_string(file)?;

    let map = parse_map(input);
    display(&map);
    let initial_state = State {
        keys: vec![],
        is: get_robots(&map),
    };
    let mut distances: HashMap<State, i64> = HashMap::new();
    let mut to_visit: BinaryHeap<Vertex<State>> = BinaryHeap::new();
    to_visit.push(Vertex(0, initial_state));
    let mut lowest_path = vec![];
    let mut lowest_cost = std::i64::MAX;
    let distance_maps = build_dijkstra_maps(&map, KEYS.chars().collect());
    while let Some(Vertex(cost, state)) = to_visit.pop() {
        println!("Cost: {:?}, Keys: {:?}", cost, state.keys);
        if cost > lowest_cost {
            break;
        }
        let neighbours = fast_neighbours(&map, &state, &distance_maps);
        for (n, new_cost) in neighbours {
            //println!("New cost: {} to {:?}", new_cost, n);
            let new_distance = cost + new_cost;
            //println!("State: {:?}", n.keys);
            //println!("Current: {:?}", distances.get(&n));
            //println!("New: {:?}", new_distance);
            let is_shorter = distances
                .get(&n)
                .map_or(true, |&current| new_distance < current);
            //pause();
            if is_shorter {
                distances.insert(n.clone(), new_distance);
                //println!("Pushing {} steps to {:?}", new_distance, n);
                if n.keys.len() == num_keys {
                    if new_distance < lowest_cost {
                        lowest_cost = new_distance;
                        lowest_path = n.keys.clone();
                    }
                }
                to_visit.push(Vertex(new_distance, n));
            }
        }
    }
    println!("DONE");
    println!("{} steps to {:?}", lowest_cost, lowest_path);
    println!("Hello, world!");
    Ok(())
}
