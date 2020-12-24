mod tile;

use std::fs::File;
use std::{env, error, io};
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let lines: Vec<String> = io::BufReader::new(file).lines()
            .map(|line| line.unwrap())
            .collect();

        println!("Flipped tiles: {}", tile::count_flipped_tiles(lines));
    } else {
        simple_error::bail!("Usage: day24 INPUT_FILE_PATH");
    }

    Ok(())
}
