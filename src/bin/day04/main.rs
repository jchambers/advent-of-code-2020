#[macro_use]
extern crate lazy_static;

mod passport;

use std::{error, env};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let mut passports_with_required_fields = 0;

            std::fs::read_to_string(path)?.split("\n\n")
                .for_each(|passport| {
                    if passport::has_required_fields(&passport::parse(passport.to_string())) {
                        passports_with_required_fields += 1;
                    }
                });

            println!("Passports with all required fields: {}", passports_with_required_fields);
        }

        {
            let mut valid_passports = 0;

            std::fs::read_to_string(path)?.split("\n\n")
                .for_each(|passport| {
                    if passport::is_valid(&passport::parse(passport.to_string())) {
                        valid_passports += 1;
                    }
                });

            println!("Valid passports: {}", valid_passports);
        }
    } else {
        simple_error::bail!("Usage: day04 INPUT_FILE_PATH");
    }

    Ok(())
}