use std::collections::HashMap;
use crate::mask::Instruction::{SetMask, SetValue};

pub fn eval_value_mask<I>(instructions: I) -> u64
    where I: Iterator<Item = Instruction>
{
    let mut memory = HashMap::new();
    let mut mask = Mask::default();

    for instruction in instructions {
        match instruction {
            SetMask(m) => {
                mask = m
            },

            SetValue(address, value) => {
                memory.insert(address, mask.apply(value));
            },
        };
    }

    memory.values().sum()
}

pub fn eval_address_mask<I>(instructions: I) -> u64
    where I: Iterator<Item = Instruction>
{
    let mut memory = HashMap::new();
    let mut mask = Mask::default();

    for instruction in instructions {
        match instruction {
            SetMask(m) => {
                mask = m
            },

            SetValue(address, value) => {
                for masked_address in get_addresses(address, &mask) {
                    memory.insert(masked_address, value);
                }
            },
        };
    }

    memory.values().sum()
}

fn get_addresses(base_address: u64, mask: &Mask) -> Vec<u64> {
    let masked_address = base_address | mask.set;

    if mask.float.count_ones() == 0 {
        return vec![masked_address];
    }

    // If there are N floating bits, then there are 2^N possible derived addresses; the plan is to
    // count from 0 to 2^N - 1, then "expand" that result out so the bits of the counter are in the
    // positions marked as floating bits by the given mask.
    let mut positions = Vec::new();

    for i in 0..64 {
        if mask.float & (1 << i) != 0 {
            positions.push(i);
        }
    }

    let mut addresses = Vec::with_capacity(1 << mask.float.count_ones());

    for state in 0..(1 << mask.float.count_ones()) {
        let mut expanded_state: u64 = 0;

        for i in 0..positions.len() {
            // Choose the ith bit (state & (1 << i)), then move it to its final position in the
            // expanded state value.
            expanded_state |= (state & (1 << i)) << (positions[i] - i);
        }

        addresses.push(masked_address ^ expanded_state);
    }

    addresses
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    SetMask(Mask),
    SetValue(u64, u64),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Mask {
    set: u64,
    clear: u64,
    float: u64,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if line.starts_with("mask = ") {
            SetMask(Mask::from(&line["mask = ".len()..]))
        } else if line.starts_with("mem[") {
            let open_bracket = line.find('[').unwrap();
            let close_bracket = line.find(']').unwrap();
            let equal_sign = line.find('=').unwrap();

            let address: u64 = line[(open_bracket + 1)..close_bracket].parse().unwrap();
            let value: u64 = line[(equal_sign + 1)..].trim().parse().unwrap();

            SetValue(address, value)
        } else {
            unreachable!()
        }
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value & self.clear) | self.set
    }
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            set: 0,
            clear: 0xffffffffffffffff,
            float: 0,
        }
    }
}

impl From<&str> for Mask {

    fn from(mask: &str) -> Self {
        let mut set = 0;
        let mut clear = 0;
        let mut float = 0;

        for c in mask.chars() {
            set <<= 1;
            clear <<= 1;
            float <<= 1;

            match c {
                '1' => {
                    set |= 1;
                    clear |= 1;
                },

                '0' => {},

                'X' => {
                    clear |= 1;
                    float |= 1;
                },

                _ => {}
            }
        }

        Mask {
            set,
            clear,
            float,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mask::{Mask, Instruction};
    use crate::mask::Instruction::{SetMask, SetValue};
    use crate::mask;
    use std::collections::HashSet;

    #[test]
    fn instruction_from_string() {
        assert_eq!(SetMask(Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
                   Instruction::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));

        assert_eq!(SetValue(8, 11),
                   Instruction::from("mem[8] = 11"));

        assert_eq!(SetValue(7, 101),
                   Instruction::from("mem[7] = 101"));

        assert_eq!(SetValue(1234, 567890),
                   Instruction::from("mem[1234] = 567890"));
    }

    #[test]
    fn mask_from_string() {
        let expected = Mask {
            set: 0b000000000000000000000000000001000000,
            clear: 0b111111111111111111111111111111111101,
            float: 0b111111111111111111111111111110111101,
        };

        assert_eq!(expected, Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    }

    #[test]
    fn mask_apply() {
        let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
        assert_eq!(64, mask.apply(0));
    }

    #[test]
    fn eval() {
        let instructions: Vec<Instruction> =
            vec!["mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                 "mem[8] = 11",
                 "mem[7] = 101",
                 "mem[8] = 0",].iter()
                .map(|line| Instruction::from(*line))
                .collect();

        assert_eq!(165, mask::eval_value_mask(instructions.into_iter()));
    }

    #[test]
    fn get_addresses() {
        {
            let mask = Mask::from("000000000000000000000000000000X1001X");
            let address = 42;

            let expected: HashSet<u64> = vec![26, 27, 58, 59].into_iter().collect();
            assert_eq!(expected, mask::get_addresses(address, &mask).into_iter().collect());
        }

        {
            let mask = Mask::from("00000000000000000000000000000000X0XX");
            let address = 26;

            let expected: HashSet<u64> = vec![16, 17, 18, 19, 24, 25, 26, 27].into_iter().collect();
            assert_eq!(expected, mask::get_addresses(address, &mask).into_iter().collect());
        }
    }

    #[test]
    fn eval_address_mask() {
        let instructions: Vec<Instruction> =
            vec!["mask = 000000000000000000000000000000X1001X",
                 "mem[42] = 100",
                 "mask = 00000000000000000000000000000000X0XX",
                 "mem[26] = 1",].iter()
                .map(|line| Instruction::from(*line))
                .collect();

        assert_eq!(208, mask::eval_address_mask(instructions.into_iter()));
    }
}