mod jolt;

use std::fs::File;
use std::{env, error, io};
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let mut jolts: Vec<u32> = io::BufReader::new(file).lines()
            .filter_map(|line| line.unwrap().parse::<u32>().ok())
            .collect();

        let deltas = jolt::get_deltas(&mut jolts);

        println!("{} * {} = {}", deltas[0], deltas[2], deltas[0] * deltas[2]);
    } else {
        simple_error::bail!("Usage: day10 INPUT_FILE_PATH");
    }

    Ok(())
}
