#[macro_use]
extern crate lazy_static;

use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;

mod seating;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let file = File::open(path)?;

            if let Some(max_id) = io::BufReader::new(file).lines()
                .filter_map(|line| seating::Seat::from_code(line.unwrap().as_str()).ok())
                .map(|seat| seat.get_id())
                .max() {
                println!("Max seat ID: {}", max_id);
            } else {
                unreachable!()
            }
        }

        {
            let file = File::open(path)?;

            let mut candidate_seats: Vec<u32> = io::BufReader::new(file).lines()
                .filter_map(|line| seating::Seat::from_code(line.unwrap().as_str()).ok())
                .filter(|seat| seat.row != 0 && seat.row != 127)
                .map(|seat| seat.get_id())
                .collect();

            candidate_seats.sort();

            for i in 1..candidate_seats.len() {
                if candidate_seats[i] - candidate_seats[i - 1] != 1 {
                    println!("My seat ID: {}", candidate_seats[i] - 1);
                }
            }
        }
    } else {
        simple_error::bail!("Usage: day05 INPUT_FILE_PATH");
    }

    Ok(())
}