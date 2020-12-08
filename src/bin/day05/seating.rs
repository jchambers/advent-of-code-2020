#[derive(Debug, PartialEq)]
pub struct Seat {
    pub row: u8,
    pub col: u8,
}

impl Seat {
    pub fn from_code(boarding_pass: &str) -> Seat {
        let mut code:u32 = 0;

        for c in boarding_pass.chars() {
            code <<= 1;

            if c == 'B' || c == 'R' {
                code |= 1;
            }
        }

        Seat {
            row: (code >> 3) as u8,
            col: (code & 0b111) as u8
        }
    }

    pub fn get_id(&self) -> u32 {
        (self.row as u32 * 8) + (self.col as u32)
    }
}

#[cfg(test)]
mod test {
    use crate::seating::Seat;

    #[test]
    fn from_code() {
        assert_eq!(Seat { row: 44, col: 5}, Seat::from_code("FBFBBFFRLR"));
    }

    #[test]
    fn get_id() {
        assert_eq!(357, Seat { row: 44, col: 5 }.get_id());
    }
}