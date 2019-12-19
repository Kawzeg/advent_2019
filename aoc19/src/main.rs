use intcode::{intcode_from_file, run_with_io};
use std::io;

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

fn square_fits(map: &Map<i64>, x: usize, y: usize, width: usize, height: usize) -> bool {
    let mut iters = 0;
    let left_top = map.xy_index(x, y);
    let left_bot = map.xy_index(x, y + height-1);
    let right_top = map.xy_index(x+width-1, y);
    let right_bot = map.xy_index(x+width-1, y+height-1);
    return map.tiles[left_top] == 1 && map.tiles[left_bot] == 1 && map.tiles[right_top] == 1 && map.tiles[right_bot] == 1;
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let code = intcode_from_file(file)?;
    let width = 800;
    let height = 1100;
    let tiles = vec![0; width * height];
    let mut y_in_beam = false;
    let mut y;
    let mut last_y = 0;
    let mut map = Map {
        width: width,
        height: height,
        tiles: tiles,
    };
    
    for x in 0..width {
        y = last_y;
        loop {
            let out = run_with_io(&code, vec![x as i64, y as i64]);
            let pulling = out.output[0];
            if pulling == 1 {
                let i = map.xy_index(x, y);
                map.tiles[i] = pulling;
                if !y_in_beam {
                    y_in_beam = true;
                    last_y = y;
                }
            } else {
                if y_in_beam {
                    y_in_beam = false;
                    break;
                }
            }
            y += 1;
            if y == height {
                y_in_beam = false;
                break;
            }
        }
    }
    
    let mut num_pulled = 0;
    for x in 0..50 {
        for y in 0..50 {
            let i = map.xy_index(x, y);
            let pulling = map.tiles[i];
            if pulling == 1 {
                num_pulled += 1;
            }
        }
    }
    
    println!("Part 1 answer: {}", num_pulled);

    for x in 0..width - 100 {
        for y in 0..height - 100 {
            if square_fits(&map, x, y, 100, 100) {
                println!("Square fits at ({}, {})", x, y);
                println!("Part 2 Solution: {}", x*10000+y);
                return Ok(());
            }
        }
    }

    Ok(())
}
