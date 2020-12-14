use std::{error, env, io};
use std::fs::File;
use std::io::BufRead;
use crate::bags::BagRules;

mod bags;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());
        let rules: BagRules = lines.collect();

        println!("Potential containers for shiny gold bags: {}", rules.get_top_level_containers(&String::from("shiny gold")));
    } else {
        simple_error::bail!("Usage: day07 INPUT_FILE_PATH");
    }

    Ok(())
}