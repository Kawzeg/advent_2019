use intcode::{intcode_from_file, run_with_io};
use std::io;

#[derive(Debug, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug)]
struct Tile {
    tipe: TileType,
    x: i64,
    y: i64,
}
impl Tile {
    fn new(x: i64, y: i64, t: i64) -> Tile{
        Tile {
            tipe: match t {
                0 => TileType::Empty,
                1 => TileType::Wall,
                2 => TileType::Block,
                3 => TileType::Paddle,
                4 => TileType::Ball,
                _ => panic!("Unknown tile type"),
            },
            x: x,
            y: y,
        }
    }
}

#[derive(Debug)]
struct World {
    tiles: Vec<Tile>
}

fn display(output: Vec<i64>) {
    let mut world = World{
        tiles: vec![]
    };
    for x in 0..output.len()/3 {
        let i = x*3;
        let tile = Tile::new(output[i], output[i+1], output[i+2]);
        world.tiles.push(tile);
    }
    let min_x = world.tiles.iter().map(|t|{t.x}).min().unwrap();
    let min_y = world.tiles.iter().map(|t|{t.y}).min().unwrap();
    let max_x = world.tiles.iter().map(|t|{t.x}).max().unwrap();
    let max_y = world.tiles.iter().map(|t|{t.y}).max().unwrap();
    let blocks = world.tiles.iter().filter(|t|{t.tipe == TileType::Block}).count();
    println!("{:?}", output);
    println!("Blocks: {}", blocks);
    println!("Min_x = {}", min_x);
    println!("Min_y = {}", min_y);
    println!("Max_x = {}", max_x);
    println!("Max_y = {}", max_y);
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let output = run_with_io(&input, vec![]);
    display(output.output);
    Ok(())
}