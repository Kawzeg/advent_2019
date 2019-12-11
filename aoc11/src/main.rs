use intcode::{Intcode, intcode_from_file, run_with_io};
use std::io;
use std::collections::HashMap;

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


#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Rotation {
    Left,
    Right,
}

type Field = HashMap<(i32, i32), i64>;

fn get_color(tiles: &Field, p: (i32, i32)) -> i64 {
    return **tiles.get(&p).get_or_insert(&0)
}

fn paint(tiles: &mut Field, pos: (i32, i32), color: i64) {
    *tiles.entry(pos).or_insert(color) = color;
}

fn rotate(dir: Direction, rot: Rotation) -> Direction {
    match dir {
        Direction::Up => {
            match rot {
                Rotation::Left => Direction::Left,
                Rotation::Right => Direction::Right,
            }
        }
        Direction::Right => {
            match rot {
                Rotation::Left => Direction::Up,
                Rotation::Right => Direction::Down,
            }
        }
        Direction::Down => {
            match rot {
                Rotation::Left => Direction::Right,
                Rotation::Right => Direction::Left,
            }
        }
        Direction::Left => {
            match rot {
                Rotation::Left => Direction::Down,
                Rotation::Right => Direction::Up,
            }
        }
    }
}

#[derive(Debug)]
struct World {
    dir: Direction,
    pos: (i32, i32),
    field: Field,
    brain: Intcode,
}

fn run_once(world: &World) -> World{
    let colour = get_color(&world.field, world.pos);
    println!("Colour is {}", colour);
    let out = run_with_io(&world.brain, vec![colour as i64]);
    let control = &out.output;
    println!("Painter said {:?}", control);
    let new_color = control[0];
    let new_dir = match control[1] {
        0 => rotate(world.dir, Rotation::Left),
        1 => rotate(world.dir, Rotation::Right),
        _ => panic!("Unknown direction {}", control[1])
    };
    println!("Rotated from {:?} to {:?}", world.dir, new_dir);
    let pos = world.pos;
    let new_pos = match new_dir {
        Direction::Up => (pos.0, pos.1+1),
        Direction::Right => (pos.0+1, pos.1),
        Direction::Down => (pos.0, pos.1-1),
        Direction::Left => (pos.0-1, pos.1),
    };
    let mut new_brain = out.clone();
    let mut new_field = world.field.clone();
    paint(&mut new_field, pos, new_color);
    new_brain.output = vec![];
    World {
        dir: new_dir,
        pos: new_pos,
        field: new_field,
        brain: new_brain,
    }
}

fn run(world: World) {
    let mut new_world = world;
    loop {
        new_world = run_once(&new_world);
        println!("Dir: {:?}, pos: {:?}, field: {:?}", new_world.dir, new_world.pos, new_world.field);
        //pause();
        if new_world.brain.halted {
            println!("Done: {:?}", new_world);
            println!("Painted {} fields", new_world.field.len());
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let brain = intcode_from_file(file)?;

    let start_coords: (i32, i32) = (0, 0);
    let painted_tiles: Field = HashMap::new();
    let start_direction = Direction::Up;
    let world = World {
        dir: start_direction,
        pos: start_coords,
        field: painted_tiles,
        brain: brain,
    };

    run(world);
    Ok(())
}
