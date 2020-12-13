use std::{error, env};
use crate::seating::SeatingMap;

mod seating;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let stable_map = SeatingMap::from_layout(std::fs::read_to_string(path).unwrap()).into_stable_configuration();
        println!("Occupied seats when settled: {}", stable_map.get_occupied_seats());
    } else {
        simple_error::bail!("Usage: day11 INPUT_FILE_PATH");
    }

    Ok(())
}
