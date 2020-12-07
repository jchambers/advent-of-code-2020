mod password;

use std::{error, env, io};
use std::io::BufRead;
use std::fs::File;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let file = File::open(path)?;

            let valid_passwords = io::BufReader::new(file).lines()
                .filter_map(|line| { line.ok() })
                .filter_map(|line| {
                    let (policy, password) = password::parse(line.as_str()).unwrap();

                    if password::is_valid_part1(policy, password) {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            println!("Valid passwords by part 1 policy: {}", valid_passwords);
        }

        {
            let file = File::open(path)?;

            let valid_passwords = io::BufReader::new(file).lines()
                .filter_map(|line| { line.ok() })
                .filter_map(|line| {
                    let (policy, password) = password::parse(line.as_str()).unwrap();

                    if password::is_valid_part2(policy, password) {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            println!("Valid passwords by part 2 policy: {}", valid_passwords);
        }
    } else {
        simple_error::bail!("Usage: day02 INPUT_FILE_PATH");
    }

    Ok(())
}