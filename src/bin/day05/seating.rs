use regex::Regex;
use std::error;

#[derive(Debug, PartialEq)]
pub struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    pub fn from_code(code: &str) -> Result<Seat, Box<dyn error::Error>> {
        lazy_static! {
            static ref SEAT_CODE_RE: Regex = Regex::new("^[BF]{7}[LR]{3}$").unwrap();
        }

        if !SEAT_CODE_RE.is_match(code) {
            simple_error::bail!("Invalid seat code: {}", code);
        }

        let mut row: u8 = 0;
        let mut col: u8 = 0;

        let chars: Vec<char> = code.chars().collect();

        for i in 0..7 {
            row <<= 1;

            if chars[i] == 'B' {
                row |= 1;
            }
        }

        for i in 7..10 {
            col <<= 1;

            if chars[i] == 'R' {
                col |= 1;
            }
        }

        Ok(Seat { row, col })
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
        if let Ok(seat) = Seat::from_code("FBFBBFFRLR") {
            assert_eq!(Seat { row: 44, col: 5}, seat);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn get_id() {
        assert_eq!(357, Seat { row: 44, col: 5 }.get_id());
    }
}