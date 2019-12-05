use intcode::{Intcode, intcode_from_file, run};
use std::io;

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let output = run(input);
    println!("Output is {:?}", output);
    Ok(())
}
