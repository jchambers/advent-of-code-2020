#[macro_use]
extern crate lazy_static;

use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;

mod seating;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        if let Some(max_id) = io::BufReader::new(file).lines()
            .filter_map(|line| seating::Seat::from_code(line.unwrap().as_str()).ok())
            .map(|seat| seat.get_id())
            .max() {

            println!("Max seat ID: {}", max_id);
        } else {
            unreachable!()
        }
    } else {
        simple_error::bail!("Usage: day05 INPUT_FILE_PATH");
    }

    Ok(())
}