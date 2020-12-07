mod passport;

use std::{error, env};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let required_fields = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        ];

        let mut valid_passports = 0;

        std::fs::read_to_string(path)?.split("\n\n")
            .for_each(|passport| {
                if passport::is_valid(&passport::parse(passport.to_string()), &required_fields) {
                    valid_passports += 1;
                }
            });

        println!("Valid passports: {}", valid_passports);
    } else {
        simple_error::bail!("Usage: day04 INPUT_FILE_PATH");
    }

    Ok(())
}