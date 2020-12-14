use std::{error, env};
use itertools::Itertools;
use crate::bus::BusSchedule;

mod bus;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let bus_notes = std::fs::read_to_string(path).unwrap();
            let (timestamp, routes) = bus_notes.split('\n').collect_tuple().unwrap();
            let timestamp: u32 = timestamp.parse().unwrap();

            let schedule = BusSchedule::from(routes);
            let earliest_arrival = schedule.get_first_route_arriving_after_time(timestamp);

            println!("Earliest route arriving after {}: {} @ {}", timestamp, earliest_arrival.route, earliest_arrival.arrival);
            println!("({} - {}) * {} = {}", earliest_arrival.arrival, timestamp, earliest_arrival.route,
                     (earliest_arrival.arrival - timestamp) * earliest_arrival.route);
        }
    } else {
        simple_error::bail!("Usage: day13 INPUT_FILE_PATH");
    }

    Ok(())
}
