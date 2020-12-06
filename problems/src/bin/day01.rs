use std::{env, io, error};
use std::fs::File;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let mut entries: Vec<i32> = io::BufReader::new(file).lines()
            .filter_map(|line| line.unwrap().parse().ok())
            .collect();

        entries.sort();

        for i in 0..(entries.len() - 2) {
            for j in i..(entries.len() - 1) {
                if entries[i] + entries[j] == 2020 {
                    println!("{} * {} = {}", entries[i], entries[j], entries[i] * entries[j]);
                }
            }
        }

        for i in 0..(entries.len() - 2) {
            for j in i..(entries.len() - 1) {
                for k in j..entries.len() {
                    if entries[i] + entries[j] + entries[k] == 2020 {
                        println!("{} * {} * {} = {}", entries[i], entries[j], entries[k],
                                 entries[i] * entries[j] * entries[k]);
                        return Ok(());
                    }
                }
            }
        }

    } else {
        simple_error::bail!("Usage: day01 INPUT_FILE_PATH");
    }

    Ok(())
}
