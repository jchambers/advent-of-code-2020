#[macro_use]
extern crate lazy_static;

use std::{error, env};
use crate::ticket::{Field, Ticket};

mod ticket;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let (fields, tickets, _my_ticket) = {
            let input = std::fs::read_to_string(path).unwrap();
            let sections: Vec<&str> = input.split_terminator("\n\n").collect();

            let fields: Vec<Field> = sections[0].split_terminator('\n')
                .map(|line| Field::from(line))
                .collect();

            let my_ticket = sections[1].split_terminator('\n')
                .skip(1)
                .map(|line| Ticket::from(line))
                .next()
                .unwrap();

            let tickets: Vec<Ticket> = sections[2].split_terminator('\n')
                .skip(1)
                .map(|line| Ticket::from(line))
                .collect();

            (fields, tickets, my_ticket)
        };

        println!("Error rate: {}", ticket::get_error_rate(&tickets, &fields));
    } else {
        simple_error::bail!("Usage: day16 INPUT_FILE_PATH");
    }

    Ok(())
}