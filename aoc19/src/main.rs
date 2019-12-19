use std::io;
use intcode::{intcode_from_file, run_with_io};

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let code = intcode_from_file(file)?;
    let mut pulled = 0;
    for x in 0..50 {
        for y in 0..50 {
            let out = run_with_io(&code, vec![x, y]);
            if out.output[0] == 1 {
                println!("({}, {}) is pulled", x, y);
                pulled += 1;
            }
        }
    }
    println!("Total: {}", pulled);

    Ok(())
}
