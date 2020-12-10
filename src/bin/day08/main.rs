use std::{error, env, io};
use std::fs::File;
use std::io::BufRead;
use crate::console::{Exit, Instruction};

mod console;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let instructions:Vec<Instruction> = io::BufReader::new(file).lines()
            .map(|line| console::Instruction::from_str(line.unwrap().as_str()))
            .collect();

        if let Exit::Loop(acc) = console::eval(&instructions) {
            println!("ACC value at start of loop: {}", acc);
        }

        println!("ACC value after repair: {}", console::get_acc_after_repair(&instructions));
    } else {
        simple_error::bail!("Usage: day08 INPUT_FILE_PATH");
    }

    Ok(())
}