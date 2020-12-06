use std::{env, io, error};
use std::fs::File;
use std::io::BufRead;

use ksum;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let mut entries: Vec<i32> = io::BufReader::new(file).lines()
            .filter_map(|line| line.unwrap().parse().ok())
            .collect();

        entries.sort();

        if let Some(pair) = ksum::ksum(&entries, 2, 2020).iter().next() {
            assert_eq!(2, pair.len());
            println!("{} * {} = {}", pair[0], pair[1], pair[0] * pair[1]);
        } else {
            println!("No pair with sum 2020 found in input list");
        }

        if let Some(triple) = ksum::ksum(&entries, 3, 2020).iter().next() {
            assert_eq!(3, triple.len());
            println!("{} * {} * {} = {}", triple[0], triple[1], triple[2],
                     triple[0] * triple[1] * triple[2]);
        } else {
            println!("No triple with sum 2020 found in input list");
        }
    } else {
        simple_error::bail!("Usage: day01 INPUT_FILE_PATH");
    }

    Ok(())
}
