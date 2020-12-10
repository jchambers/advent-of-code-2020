use crate::console::Opcode::{NOP, JMP};
use crate::console::Exit::Terminate;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
    value: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Exit {
    Loop(i32),
    Terminate(i32),
}

pub fn eval(instructions: &Vec<Instruction>) -> Exit {
    let mut visited: Vec<bool> = vec![false; instructions.len()];
    let mut i = 0;
    let mut acc = 0;

    while i < instructions.len() {
        if visited[i] {
            return Exit::Loop(acc);
        }

        visited[i] = true;

        match &instructions[i].opcode {
            Opcode::NOP => i += 1,
            Opcode::JMP => i = ((i as isize) + instructions[i].value as isize) as usize,
            Opcode::ACC => {
                acc += instructions[i].value;
                i += 1;
            }
        }
    }

    Terminate(acc)
}

pub fn get_acc_after_repair(instructions: &Vec<Instruction>) -> i32 {
    for candidate in 0..instructions.len() {
        if instructions[candidate].opcode == Opcode::ACC {
            continue;
        }

        let mut repaired = instructions.clone();

        if repaired[candidate].opcode == NOP {
            repaired[candidate].opcode = JMP;
        } else {
            repaired[candidate].opcode = NOP;
        }

        if let Terminate(acc) = eval(&repaired) {
            return acc;
        }
    }

    unreachable!()
}

impl Instruction {
    pub fn from_str(line: &str) -> Instruction {
        let opcode = match &line[0..3] {
            "nop" => Opcode::NOP,
            "acc" => Opcode::ACC,
            "jmp" => Opcode::JMP,
            _ => unreachable!()
        };

        let value = line[4..].parse().unwrap();

        Instruction { opcode, value }
    }
}

#[cfg(test)]
mod test {
    use crate::console::{Instruction, Opcode, Exit};
    use crate::console;

    #[test]
    fn instruction_from_str() {
        assert_eq!(Instruction { opcode: Opcode::ACC, value: 14 }, Instruction::from_str("acc +14"));
    }

    #[test]
    fn eval() {
        {
            let instructions: Vec<Instruction> = vec!["nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6"]
                .into_iter()
                .map(Instruction::from_str)
                .collect();

            assert_eq!(Exit::Loop(5), console::eval(&instructions));
        }

        {
            let instructions: Vec<Instruction> = vec!["nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "nop -4", "acc +6"]
                .into_iter()
                .map(Instruction::from_str)
                .collect();

            assert_eq!(Exit::Terminate(8), console::eval(&instructions));
        }
    }

    #[test]
    fn get_acc_after_repair() {
        let instructions: Vec<Instruction> = vec!["nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6"]
            .into_iter()
            .map(Instruction::from_str)
            .collect();

        assert_eq!(8, console::get_acc_after_repair(&instructions));
    }
}