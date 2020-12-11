use std::fs::File;
use std::{env, error, io};
use std::io::BufRead;

mod xmas;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let numbers: Vec<i64> = io::BufReader::new(file).lines()
            .filter_map(|line| line.unwrap().parse::<i64>().ok())
            .collect();

        if let Some(impostor) = xmas::find_outlier(numbers.as_slice(), 25) {
            println!("Found impostor: {}", impostor);

            if let Some(sequence) = xmas::find_consecutive_sum(&numbers, impostor) {
                println!("Sequence adding to impostor: {:?}", sequence);

                let min = sequence.iter().min().unwrap();
                let max = sequence.iter().max().unwrap();

                println!("{} + {} = {}", min, max, min + max);
            } else {
                println!("No sequence found")
            }
        } else {
            println!("No impostor found")
        }
    } else {
        simple_error::bail!("Usage: day01 INPUT_FILE_PATH");
    }

    Ok(())
}