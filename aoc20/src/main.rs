use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Map<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
    default: T,
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
    fn get_at(&self, x: usize, y: usize) -> &T {
        let i = self.xy_index(x, y);
        if i >= self.tiles.len() {
            &self.default
        } else {
            &self.tiles[i]
        }
    }
}

fn find_floor(map: &Map<char>, x: usize, y: usize, w: usize, h: usize) -> (usize, usize) {
    let mut neighbours = vec![];
    for x1 in x..x + w {
        if y > 0 {
            neighbours.push((x1, y - 1));
        }
        neighbours.push((x1, y + h));
    }
    for y1 in y..y + h {
        if x > 0 {
            neighbours.push((x - 1, y1));
        }
        neighbours.push((x + w, y1));
    }
    for (x, y) in neighbours {
        if *map.get_at(x, y) == '.' {
            return (x, y);
        }
    }
    panic!("Floor not found");
}

fn find_portal(map: &Map<char>, id: &String) -> Vec<(usize, usize)> {
    let first = id.chars().nth(0).unwrap();
    let second = id.chars().nth(1).unwrap();
    let mut r = vec![];
    for x in 0..map.width {
        for y in 0..map.height {
            let &c = map.get_at(x, y);
            if c == first {
                if *map.get_at(x + 1, y) == second {
                    r.push(find_floor(map, x, y, 2, 1));
                } else if *map.get_at(x, y + 1) == second {
                    r.push(find_floor(map, x, y, 1, 2));
                }
            }
        }
    }
    r
}

fn get_neighbours(
    map: &Map<char>,
    x: usize,
    y: usize,
    portals: &HashMap<(usize, usize), (usize, usize)>,
) -> Vec<usize> {
    let mut r = vec![];
    let mut potentials = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        potentials.push((x - 1, y));
    }
    if y > 0 {
        potentials.push((x, y - 1));
    }
    for (x, y) in potentials {
        let &c = map.get_at(x, y);
        if c == '.' {
            r.push(map.xy_index(x, y));
        }
    }
    match portals.get(&(x, y)) {
        Some(&dest) => r.push(map.xy_index(dest.0, dest.1)),
        _ => {}
    }
    r
}

/// Returns a map of the neighbouring indices
fn parse_map(input: String) -> (Map<Vec<usize>>, usize, usize) {
    let width = input.find('\n').unwrap();
    let mut height = 0;
    for c in input.chars() {
        if c == '\n' {
            height += 1;
            continue;
        }
    }
    let char_map = Map {
        default: ' ',
        width: width,
        height: height,
        tiles: input.chars().filter(|&x| x != '\n').collect(),
    };
    let mut portals = HashMap::new();
    for f in ALPHABET.chars() {
        for s in ALPHABET.chars() {
            let mut id = "".to_string();
            id.push(f);
            id.push(s);
            let coords = find_portal(&char_map, &id);
            if coords.len() > 1 {
                portals.insert(coords[0], coords[1]);
                portals.insert(coords[1], coords[0]);
            }
        }
    }
    let mut tiles = vec![];
    for y in 0..char_map.height {
        for x in 0..char_map.width {
            if x == 57 && y == 132 {
                println!("START");
                println!("{:?}", get_neighbours(&char_map, x, y, &portals));
            }
            let &c = char_map.get_at(x, y);
            if c == '.' {
                let neighbours = get_neighbours(&char_map, x, y, &portals);
                tiles.push(neighbours);
            } else {
                tiles.push(vec![]);
            }
        }
    }
    let (x, y) = find_portal(&char_map, &"AA".to_string())[0];
    let start = char_map.xy_index(x, y);
    println!("Entrance at ({}, {})", x, y);
    println!("Charmap there: {}", char_map.get_at(x, y));
    println!("Waymap there: {:?}", tiles[char_map.xy_index(x, y)]);
    let (x, y) = find_portal(&char_map, &"ZZ".to_string())[0];
    let finish = char_map.xy_index(x, y);
    (
        Map {
            default: vec![],
            width: width,
            height: height,
            tiles: tiles,
        },
        start,
        finish,
    )
}

fn is_outer(x: usize, y: usize, width: usize, height: usize) -> bool {
    let r = x == 2 || y == 2 || 
    x == width - 3 ||
    y == height - 3;
    println!(
        "Checking ({},{}) w={}, h={}, for outer = {}",
        x, y, width, height, r
    );
    r
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    n: usize,
    inception: usize,
}

fn solve(map: &Map<Vec<usize>>, start: usize, goal: usize) -> i64 {
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Vertex(
        0,
        State {
            n: start,
            inception: 0,
        },
    ));
    let mut distances = HashMap::new();
    while let Some(Vertex(cost, state)) = to_visit.pop() {
        println!("{}: Visiting {:?}", cost, state);
        let i = state.n;
        let inception = state.inception;
        let neighbours = &map.tiles[i];
        for &n in neighbours {
            if n == goal && inception == 0 {
                return cost + 1;
            }
            let new_cost = cost + 1;
            let new_inception;
            let dist = (n as isize - state.n as isize).abs() as usize;
            if !(dist == 1 || dist == map.width) {
                // went through portal
                let (x, y) = map.index_xy(i);
                if is_outer(x, y, map.width, map.height) {
                    if inception == 0 {
                        continue;
                    }
                    new_inception = inception - 1;
                } else {
                    new_inception = inception + 1;
                }
            } else {
                new_inception = inception;
            }
            let new_state = State {
                n: n,
                inception: new_inception,
            };
            let is_shorter = distances
                .get(&new_state)
                .map_or(true, |&current| new_cost < current);
            if is_shorter {
                distances.insert(new_state, new_cost);
                to_visit.push(Vertex(new_cost, new_state));
            }
        }
    }
    panic!("Found no way");
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    let (map, start, finish) = parse_map(input);

    println!("Map at start: {:?}", map.tiles[start]);

    println!("Steps: {}", solve(&map, start, finish));

    println!("Hello, world!");
    Ok(())
}
