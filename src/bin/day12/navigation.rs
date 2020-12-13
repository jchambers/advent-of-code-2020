use crate::navigation::Heading::{East, North, South, West};
use crate::navigation::Instruction::{Translate, Rotate, Forward};

#[derive(Debug, Eq, PartialEq)]
pub struct FerryPosition {
    pub x: i32,
    pub y: i32,
    pub heading: Heading
}

#[derive(Debug, Eq, PartialEq)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Translate(Heading, i32),
    Rotate(i32),
    Forward(i32)
}

impl From<u8> for Heading {

    fn from(i: u8) -> Self {
        match i {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!()
        }
    }
}

impl From<&Heading> for u8 {
    fn from(heading: &Heading) -> Self {
        match heading {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let magnitude:i32 = string[1..].parse().unwrap();

        match &string[0..1] {
            "N" => Translate(North, magnitude),
            "S" => Translate(South, magnitude),
            "E" => Translate(East, magnitude),
            "W" => Translate(West, magnitude),
            "L" => Rotate(-magnitude),
            "R" => Rotate(magnitude),
            "F" => Forward(magnitude),
            _ => unreachable!()
        }
    }
}

impl FerryPosition {

    pub fn apply<I>(&mut self, instructions: I)
        where I: Iterator<Item = Instruction>
    {
        for instruction in instructions {
            match instruction {
                Translate(heading, magnitude) => {
                    match heading {
                        North => self.y += magnitude,
                        South => self.y -= magnitude,
                        East => self.x += magnitude,
                        West => self.x -= magnitude,
                    }
                },

                Rotate(angle) => {
                    let delta_heading_ordinal:u8 = ((((angle % 360) + 360) % 360) / 90) as u8;
                    let current_heading_ordinal:u8 = u8::from(&self.heading);

                    self.heading = Heading::from((current_heading_ordinal + delta_heading_ordinal) % 4);
                },

                Forward(magnitude) => {
                    match self.heading {
                        North => self.y += magnitude,
                        South => self.y -= magnitude,
                        East => self.x += magnitude,
                        West => self.x -= magnitude,
                    }
                },
            };
        }
    }
}

impl Default for FerryPosition {

    fn default() -> Self {
        FerryPosition {
            x: 0,
            y: 0,
            heading: East
        }
    }
}

#[cfg(test)]
mod test {
    use crate::navigation::{Instruction, FerryPosition};
    use crate::navigation::Instruction::*;
    use crate::navigation::Heading::*;

    #[test]
    fn instruction_from() {
        assert_eq!(Translate(North, 4), Instruction::from("N4"));
        assert_eq!(Translate(South, 17), Instruction::from("S17"));
        assert_eq!(Translate(East, 99), Instruction::from("E99"));
        assert_eq!(Translate(West, 3), Instruction::from("W3"));
        assert_eq!(Rotate(-270), Instruction::from("L270"));
        assert_eq!(Rotate(90), Instruction::from("R90"));
        assert_eq!(Forward(10), Instruction::from("F10"));
    }

    #[test]
    fn ferry_position_apply() {
        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate(North, 2)].into_iter());

            let expected = FerryPosition {
                x: 0,
                y: 2,
                heading: East
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate(South, 2)].into_iter());

            let expected = FerryPosition {
                x: 0,
                y: -2,
                heading: East
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate(East, 2)].into_iter());

            let expected = FerryPosition {
                x: 2,
                y: 0,
                heading: East
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate(West, 2)].into_iter());

            let expected = FerryPosition {
                x: -2,
                y: 0,
                heading: East
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Forward(7)].into_iter());

            let expected = FerryPosition {
                x: 7,
                y: 0,
                heading: East
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            assert_eq!(East, position.heading);

            position.apply(vec![Rotate(90)].into_iter());
            assert_eq!(South, position.heading);

            position.apply(vec![Rotate(90)].into_iter());
            assert_eq!(West, position.heading);

            position.apply(vec![Rotate(90)].into_iter());
            assert_eq!(North, position.heading);

            position.apply(vec![Rotate(-270)].into_iter());
            assert_eq!(East, position.heading);
        }

        {
            let instructions :Vec<Instruction> = vec!["F10", "N3", "F7", "R90", "F11"].iter()
                .map(|string| Instruction::from(*string))
                .collect();

            let mut position = FerryPosition::default();
            position.apply(instructions.into_iter());

            let expected = FerryPosition {
                x: 17,
                y: -8,
                heading: South
            };

            assert_eq!(expected, position);
        }
    }
}