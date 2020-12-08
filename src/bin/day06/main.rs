use std::{error, env};

mod customs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let sum: usize = std::fs::read_to_string(path)?.split("\n\n")
            .map(|answers| customs::FormGroup::from_answers(answers))
            .map(|form_group| form_group.get_distinct_answers().len())
            .sum();

        println!("Cardinalit sum: {}", sum);
    } else {
        simple_error::bail!("Usage: day04 INPUT_FILE_PATH");
    }

    Ok(())
}