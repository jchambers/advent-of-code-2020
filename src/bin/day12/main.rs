use std::{env, io, error};
use std::io::BufRead;
use crate::navigation::Instruction;
use std::fs::File;

mod navigation;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        {
            let instructions: Vec<navigation::Instruction> = io::BufReader::new(file).lines()
                .map(|line| Instruction::from(line.unwrap().as_str()))
                .collect();

            let mut position = navigation::FerryPosition::default();
            position.apply(instructions.into_iter());

            println!("Final position: x = {}, y = {}, distance = {}", position.x, position.y, position.x.abs() + position.y.abs());
        }
    } else {
        simple_error::bail!("Usage: day12 INPUT_FILE_PATH");
    }

    Ok(())
}