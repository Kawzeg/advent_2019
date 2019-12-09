use intcode::{intcode_from_file, run_with_io};
use std::io;

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let output = run_with_io(&input, vec![2]);

    println!("Result is {:?}", output.output);
    Ok(())
}
