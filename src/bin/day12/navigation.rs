use crate::navigation::Instruction::{Translate, Rotate, Forward};

#[derive(Debug, Eq, PartialEq)]
pub struct FerryPosition {
    pub x: i32,
    pub y: i32,
    pub heading: (i32, i32)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Translate((i32, i32), i32),
    Rotate(i32),
    Forward(i32)
}

fn air_quotes_sine(angle: i32) -> i32 {
    let angle = ((angle % 360) + 360) % 360;

    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!()
    }
}

fn air_quotes_cosine(angle: i32) -> i32 {
    let angle = ((angle % 360) + 360) % 360;

    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!()
    }
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let magnitude:i32 = string[1..].parse().unwrap();

        match &string[0..1] {
            "N" => Translate((0, 1), magnitude),
            "S" => Translate((0, -1), magnitude),
            "E" => Translate((1, 0), magnitude),
            "W" => Translate((-1, 0), magnitude),
            "L" => Rotate(magnitude),
            "R" => Rotate(-magnitude),
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
                    self.x += heading.0 * magnitude;
                    self.y += heading.1 * magnitude;
                },

                Rotate(angle) => {
                    let x_hat = (self.heading.0 * air_quotes_cosine(angle)) - (self.heading.1 * air_quotes_sine(angle));
                    let y_hat = (self.heading.0 * air_quotes_sine(angle)) + (self.heading.1 * air_quotes_cosine(angle));

                    self.heading = (x_hat, y_hat);
                },

                Forward(magnitude) => {
                    self.x += self.heading.0 * magnitude;
                    self.y += self.heading.1 * magnitude;
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
            heading: (1, 0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::navigation::{Instruction, FerryPosition};
    use crate::navigation::Instruction::*;

    #[test]
    fn instruction_from() {
        assert_eq!(Translate((0, 1), 4), Instruction::from("N4"));
        assert_eq!(Translate((0, -1), 17), Instruction::from("S17"));
        assert_eq!(Translate((1, 0), 99), Instruction::from("E99"));
        assert_eq!(Translate((-1, 0), 3), Instruction::from("W3"));
        assert_eq!(Rotate(270), Instruction::from("L270"));
        assert_eq!(Rotate(-90), Instruction::from("R90"));
        assert_eq!(Forward(10), Instruction::from("F10"));
    }

    #[test]
    fn ferry_position_apply() {
        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate((0, 1), 2)].into_iter());

            let expected = FerryPosition {
                x: 0,
                y: 2,
                heading: (1, 0)
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate((0, -1), 2)].into_iter());

            let expected = FerryPosition {
                x: 0,
                y: -2,
                heading: (1, 0)
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate((1, 0), 2)].into_iter());

            let expected = FerryPosition {
                x: 2,
                y: 0,
                heading: (1, 0)
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Translate((-1, 0), 2)].into_iter());

            let expected = FerryPosition {
                x: -2,
                y: 0,
                heading: (1, 0)
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            position.apply(vec![Forward(7)].into_iter());

            let expected = FerryPosition {
                x: 7,
                y: 0,
                heading: (1, 0)
            };

            assert_eq!(expected, position);
        }

        {
            let mut position = FerryPosition::default();
            assert_eq!((1, 0), position.heading);

            position.apply(vec![Rotate(-90)].into_iter());
            assert_eq!((0, -1), position.heading);

            position.apply(vec![Rotate(-90)].into_iter());
            assert_eq!((-1, 0), position.heading);

            position.apply(vec![Rotate(-90)].into_iter());
            assert_eq!((0, 1), position.heading);

            position.apply(vec![Rotate(270)].into_iter());
            assert_eq!((1, 0), position.heading);
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
                heading: (0, -1)
            };

            assert_eq!(expected, position);
        }
    }
}