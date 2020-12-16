use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Field {
    name: String,
    ranges: [(u32, u32); 2],
}

#[derive(Debug, Eq, PartialEq)]
pub struct Ticket {
    pub values: Vec<u32>,
}

pub fn get_error_rate(tickets: &Vec<Ticket>, fields: &Vec<Field>) -> u32 {
    tickets.iter()
        .map(|ticket| ticket.get_invalid_values(fields).iter().sum::<u32>())
        .sum()
}

fn get_candidate_fields<'a>(tickets: &Vec<Ticket>, fields: &'a Vec<Field>) -> Vec<HashSet<&'a Field>> {
    let valid_tickets: Vec<&Ticket> = tickets.iter()
        .filter(|ticket| ticket.is_valid(fields))
        .collect();

    let mut candidates_by_position = Vec::new();

    for i in 0..valid_tickets[0].values.len() {
        let values_for_position: Vec<u32> = valid_tickets.iter()
            .map(|ticket| ticket.values[i])
            .collect();

        candidates_by_position.push(fields.iter()
            .filter(|field| values_for_position.iter().all(|value| field.is_valid(*value)))
            .collect());
    }

    candidates_by_position
}

fn get_field_order<'a>(tickets: &Vec<Ticket>, fields: &'a Vec<Field>) -> Vec<&'a Field> {
    let mut candidates_by_position = get_candidate_fields(tickets, fields);

    while candidates_by_position.iter().any(|field_set| field_set.len() > 1) {
        let mut confirmed_fields = Vec::new();

        candidates_by_position.iter()
            .filter(|field_set| field_set.len() == 1)
            .for_each(|single_field_set| {
                confirmed_fields.push(*single_field_set.iter().next().unwrap());
            });

        candidates_by_position.iter_mut()
            .filter(|field_set| field_set.len() > 1)
            .for_each(|field_set| {
                confirmed_fields.iter().for_each(|field| {
                    field_set.remove(field);
                });
            })
    }

    candidates_by_position.into_iter()
        .map(|field_set| *field_set.iter().next().unwrap())
        .collect()
}

pub fn get_departure_field_indices(tickets: &Vec<Ticket>, fields: &Vec<Field>) -> Vec<usize> {
    let mut indices = Vec::new();
    let ordered_fields = get_field_order(tickets, fields);

    for i in 0..ordered_fields.len() {
        if ordered_fields[i].is_departure_field() {
            indices.push(i);
        }
    }

    indices
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

    pub fn is_departure_field(&self) -> bool {
        self.name.starts_with("departure")
    }
}

impl Ticket {
    pub fn get_invalid_values(&self, fields: &Vec<Field>) -> Vec<u32> {
        self.values.iter()
            .filter(|value| fields.iter().all(|field| !field.is_valid(**value)))
            .map(|value| *value)
            .collect()
    }

    pub fn is_valid(&self, fields: &Vec<Field>) -> bool {
        self.get_invalid_values(fields).is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::ticket::{Field, Ticket};
    use crate::ticket;
    use std::collections::HashSet;

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

    #[test]
    fn ticket_is_valid() {
        let fields = vec![Field::from("class: 1-3 or 5-7"),
                          Field::from("row: 6-11 or 33-44"),
                          Field::from("seat: 13-40 or 45-50"),];

        assert!(Ticket::from("7,3,47").is_valid(&fields));
        assert!(!Ticket::from("40,4,50").is_valid(&fields));
        assert!(!Ticket::from("55,2,20").is_valid(&fields));
        assert!(!Ticket::from("38,6,12").is_valid(&fields));
    }

    #[test]
    fn get_candidate_fields() {
        let fields = vec![Field::from("class: 0-1 or 4-19"),
                          Field::from("row: 0-5 or 8-19"),
                          Field::from("seat: 0-13 or 16-19"),];

        let tickets = vec![Ticket::from("3,9,18"),
                           Ticket::from("15,1,5"),
                           Ticket::from("5,14,9"),];

        let mut expected = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        expected[0].insert(&fields[1]);
        expected[1].insert(&fields[0]);
        expected[1].insert(&fields[1]);
        expected[2].insert(&fields[0]);
        expected[2].insert(&fields[1]);
        expected[2].insert(&fields[2]);

        assert_eq!(expected, ticket::get_candidate_fields(&tickets, &fields));
    }

    #[test]
    fn get_field_order() {
        let fields = vec![Field::from("class: 0-1 or 4-19"),
                          Field::from("row: 0-5 or 8-19"),
                          Field::from("seat: 0-13 or 16-19"),];

        let tickets = vec![Ticket::from("3,9,18"),
                           Ticket::from("15,1,5"),
                           Ticket::from("5,14,9"),];

        assert_eq!(vec![&fields[1], &fields[0], &fields[2]], ticket::get_field_order(&tickets, &fields));
    }

    #[test]
    fn field_is_departure_field() {
        assert!(Field::from("departure time: 31-660 or 678-951").is_departure_field());
        assert!(!Field::from("arrival location: 26-482 or 504-959").is_departure_field());
    }
}