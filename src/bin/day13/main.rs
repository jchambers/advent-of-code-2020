use std::{error, env};
use itertools::Itertools;
use crate::bus::BusSchedule;

mod bus;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let bus_notes = std::fs::read_to_string(path).unwrap();
        let (timestamp, routes) = bus_notes.split('\n').collect_tuple().unwrap();
        let timestamp: u64 = timestamp.parse().unwrap();

        let schedule = BusSchedule::from(routes);
        let earliest_arrival = schedule.get_first_route_arriving_after_time(timestamp);

        println!("Earliest route arriving after {}: {} @ {}", timestamp, earliest_arrival.route, earliest_arrival.arrival);
        println!("Wait {} * route {} = {}", BusSchedule::get_wait_time(timestamp, earliest_arrival.route), earliest_arrival.route,
                 BusSchedule::get_wait_time(timestamp, earliest_arrival.route) * earliest_arrival.route);

        println!("Alignment timestamp: {}", schedule.get_alignment_timestamp());
    } else {
        simple_error::bail!("Usage: day13 INPUT_FILE_PATH");
    }

    Ok(())
}
