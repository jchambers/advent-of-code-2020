use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    name: String,
    ranges: [(u32, u32); 2],
}

#[derive(Debug, Eq, PartialEq)]
pub struct Ticket {
    values: Vec<u32>,
}

pub fn get_error_rate(tickets: &Vec<Ticket>, fields: &Vec<Field>) -> u32 {
    tickets.iter()
        .map(|ticket| ticket.get_invalid_values(fields).iter().sum::<u32>())
        .sum()
}

impl From<&str> for Field {

    fn from(line: &str) -> Self {
        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new("^([a-z ]+):\\s*([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
        }

        let captures = FIELD_RE.captures(line).unwrap();

        Field {
            name: String::from(&captures[1]),
            ranges: [(captures[2].parse().unwrap(), captures[3].parse().unwrap()),
                     (captures[4].parse().unwrap(), captures[5].parse().unwrap())],
        }
    }
}

impl From<&str> for Ticket {
    fn from(line: &str) -> Self {
        Ticket {
            values: line.split(',').map(|number| number.parse().unwrap()).collect(),
        }
    }
}

impl Field {
    pub fn is_valid(&self, value: u32) -> bool {
        (value >= self.ranges[0].0 && value <= self.ranges[0].1) ||
            (value >= self.ranges[1].0 && value <= self.ranges[1].1)
    }
}

impl Ticket {
    pub fn get_invalid_values(&self, fields: &Vec<Field>) -> Vec<u32> {
        self.values.iter()
            .filter(|value| fields.iter().all(|field| !field.is_valid(**value)))
            .map(|value| *value)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::ticket::{Field, Ticket};
    use crate::ticket;

    #[test]
    fn field_from_str() {
        let expected = Field {
            name: String::from("row"),
            ranges: [(6, 11), (33, 44)],
        };

        assert_eq!(expected, Field::from("row: 6-11 or 33-44"));
    }

    #[test]
    fn ticket_from_str() {
        let expected = Ticket {
            values: vec![7, 1, 14],
        };

        assert_eq!(expected, Ticket::from("7,1,14"));
    }

    #[test]
    fn field_is_valid() {
        let field = Field::from("class: 1-3 or 5-7");

        assert!(field.is_valid(3));
        assert!(field.is_valid(5));
        assert!(!field.is_valid(4));
    }

    #[test]
    fn ticket_get_invalid_values() {
        let fields = vec![Field::from("class: 1-3 or 5-7"),
                          Field::from("row: 6-11 or 33-44"),
                          Field::from("seat: 13-40 or 45-50"),];

        let empty: Vec<u32> = vec![];

        assert_eq!(empty, Ticket::from("7,3,47").get_invalid_values(&fields));
        assert_eq!(vec![4], Ticket::from("40,4,50").get_invalid_values(&fields));
        assert_eq!(vec![55], Ticket::from("55,2,20").get_invalid_values(&fields));
        assert_eq!(vec![12], Ticket::from("38,6,12").get_invalid_values(&fields));
    }

    #[test]
    fn get_error_rate() {
        let fields = vec![Field::from("class: 1-3 or 5-7"),
                          Field::from("row: 6-11 or 33-44"),
                          Field::from("seat: 13-40 or 45-50"),];

        let tickets = vec![Ticket::from("7,3,47"),
                           Ticket::from("40,4,50"),
                           Ticket::from("55,2,20"),
                           Ticket::from("38,6,12"),];

        assert_eq!(71, ticket::get_error_rate(&tickets, &fields));


    }
}