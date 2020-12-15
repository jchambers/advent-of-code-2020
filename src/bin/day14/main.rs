use std::{env, io, error};
use std::io::BufRead;
use std::fs::File;

mod mask;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let file = File::open(path)?;

            let instructions: Vec<mask::Instruction> = io::BufReader::new(file).lines()
                .map(|line| mask::Instruction::from(line.unwrap().as_str()))
                .collect();

            let sum = mask::eval_value_mask(instructions.into_iter());

            println!("Sum of non-zero values using value-masking approach: {}", sum);
        }

        {
            let file = File::open(path)?;

            let instructions: Vec<mask::Instruction> = io::BufReader::new(file).lines()
                .map(|line| mask::Instruction::from(line.unwrap().as_str()))
                .collect();

            let sum = mask::eval_address_mask(instructions.into_iter());

            println!("Sum of non-zero values using address-masking approach: {}", sum);
        }
    } else {
        simple_error::bail!("Usage: day14 INPUT_FILE_PATH");
    }

    Ok(())
}