#[derive(Debug, Eq, PartialEq)]
pub struct BusSchedule {
    routes: Vec<Option<u64>>
}

#[derive(Debug, Eq, PartialEq)]
pub struct RouteRecommendation {
    pub route: u64,
    pub arrival: u64
}

impl BusSchedule {
    pub fn get_first_route_arriving_after_time(&self, timestamp: u64) -> RouteRecommendation {
        let mut earliest_arrival = 0xffffffff;
        let mut ealiest_route = 0;

        for route in &self.routes {
            if let Some(route) = route {
                let next_arrival = Self::get_next_arrival(timestamp, *route);

                if next_arrival < earliest_arrival {
                    earliest_arrival = next_arrival;
                    ealiest_route = *route;
                }
            }
        }

        RouteRecommendation {
            route: ealiest_route,
            arrival: earliest_arrival,
        }
    }

    fn get_next_arrival(timestamp: u64, route: u64) -> u64 {
        if timestamp % route == 0 {
            timestamp
        } else {
            timestamp + route - (timestamp % route)
        }
    }

    pub fn get_wait_time(timestamp: u64, route: u64) -> u64 {
        if timestamp % route == 0 {
            0
        } else {
            route - (timestamp % route)
        }
    }

    pub fn get_alignment_timestamp(&self) -> u64 {
        let mut offset = 0;
        let mut step = 1;

        for i in 0..self.routes.len() {
            if let Some(route) = self.routes[i] {
                let mut k = 1;

                while Self::get_wait_time(offset + (step * k), route) != (i as u64 % route) {
                    k += 1;
                }

                offset += step * k;
                step *= route;
            }
        }

        offset
    }
}

impl From<&str> for BusSchedule {
    fn from(string: &str) -> Self {
        BusSchedule {
            routes: string.split(',')
                .map(|route| { route.parse().ok() })
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bus::{BusSchedule, RouteRecommendation};

    #[test]
    fn bus_schedule_from_string() {
        let expected = BusSchedule {
            routes: vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)]
        };

        assert_eq!(expected, BusSchedule::from("7,13,x,x,59,x,31,19"));
    }

    #[test]
    fn get_next_arrival() {
        assert_eq!(945, BusSchedule::get_next_arrival(939, 7));
        assert_eq!(949, BusSchedule::get_next_arrival(939, 13));
        assert_eq!(944, BusSchedule::get_next_arrival(939, 59));
        assert_eq!(961, BusSchedule::get_next_arrival(939, 31));
        assert_eq!(950, BusSchedule::get_next_arrival(939, 19));
        assert_eq!(939, BusSchedule::get_next_arrival(939, 3));
    }

    #[test]
    fn get_first_route_arriving_after_time() {
        let schedule = BusSchedule::from("7,13,x,x,59,x,31,19");
        let expected = RouteRecommendation {
            route: 59,
            arrival: 944,
        };

        assert_eq!(expected, schedule.get_first_route_arriving_after_time(939));
    }

    #[test]
    fn get_alignment_timestamp() {
        assert_eq!(3417, BusSchedule::from("17,x,13,19").get_alignment_timestamp());
        assert_eq!(754018, BusSchedule::from("67,7,59,61").get_alignment_timestamp());
        assert_eq!(779210, BusSchedule::from("67,x,7,59,61").get_alignment_timestamp());
        assert_eq!(1261476, BusSchedule::from("67,7,x,59,61").get_alignment_timestamp());
        assert_eq!(1202161486, BusSchedule::from("1789,37,47,1889").get_alignment_timestamp());
    }
}