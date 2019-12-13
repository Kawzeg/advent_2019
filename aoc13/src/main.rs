use intcode::{intcode_from_file, run_with_io};
use std::io;
use std::io::prelude::*;

const width: i64 = 37;
const height: i64 = 25;

/// Normalizes to 1, 0 or -1
fn norm(x: i64) -> i64 {
    if x == 0 {
        0
    } else if x > 0 {
        1
    } else {
        -1
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
    fn new(x: i64, y: i64, t: i64) -> Tile {
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
    tiles: Vec<Tile>,
    score: i64,
}

fn display(world: &World) {
    let blocks = world
        .tiles
        .iter()
        .filter(|t| t.tipe == TileType::Block)
        .count();
    for y in 0..height {
        for x in 0..width {
            match world.tiles[(y * width + x) as usize].tipe {
                TileType::Block => {
                    print!("#");
                }
                TileType::Ball => {
                    print!("■");
                }
                TileType::Empty => {
                    print!(" ");
                }
                TileType::Wall => {
                    print!("O");
                }
                TileType::Paddle => {
                    print!("_");
                }
            }
        }
        print!("\n");
    }
    println!("Score: {}", world.score);
}

fn xy_index(x: i64, y: i64) -> usize {
    (y * width + x) as usize
}


// Returns the ball x position
fn update(world: &mut World, output: Vec<i64>) -> (i64, i64) {
    let mut ball_x = 0;
    let mut paddle_x = -1;
    for j in 0..output.len() / 3 {
        let i = j * 3;
        let x = output[i];
        let y = output[i + 1];
        let tipe = output[i + 2];
        if x == -1 && y == 0 {
            world.score = tipe;
        } else {
            println!("New tile: {} {} {}", x, y, tipe);
            let tile = Tile::new(x, y, tipe);
            if tile.tipe == TileType::Ball {
                ball_x = tile.x;
            } else if tile.tipe == TileType::Paddle {
                paddle_x = tile.x;
            }
            let index = xy_index(x, y);
            if world.tiles.len() <= index {
                world.tiles.push(tile);
            } else {
                world.tiles[index] = tile;
            }
        }
    }
    (ball_x, paddle_x)
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let mut code = intcode_from_file(file)?;
    let mut world = World {
        tiles: vec![],
        score: 0,
    };
    code.data[0] = 2;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut joy_in = 0;
    while !code.halted {
        code = run_with_io(&code, vec![joy_in]);
        let (new_ball_x, new_paddle_x) = update(&mut world, code.output);
        display(&world);
        code.output = vec![];
        if new_paddle_x != -1 {
            paddle_x = new_paddle_x;
        }
        if new_ball_x != -1 {
            ball_x = new_ball_x;
        }
        joy_in = norm(ball_x - paddle_x);
        println!("Input: {}", joy_in);
        pause();
    }
    Ok(())
}
