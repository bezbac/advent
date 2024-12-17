use std::{collections::VecDeque, fs, ops::BitXor};

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => return Err(()),
        })
    }
}

struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: VecDeque<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let mut lines = input.lines();

        let line = lines.next().unwrap();

        let line = line.trim_start_matches("Register A: ");
        let register_a = line.parse().unwrap();

        let line = lines.next().unwrap();
        let line = line.trim_start_matches("Register B: ");
        let register_b = line.parse().unwrap();

        let line = lines.next().unwrap();
        let line = line.trim_start_matches("Register C: ");
        let register_c = line.parse().unwrap();

        lines.next();

        let line = lines.next().unwrap();
        let line = line.trim_start_matches("Program: ");

        let program: VecDeque<_> = line.trim().split(',').map(|c| c.parse().unwrap()).collect();

        Self {
            register_a,
            register_b,
            register_c,
            program,
            output: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let mut instruction_pointer = 0;

        while instruction_pointer + 1 < self.program.len() {
            let instruction = Instruction::try_from(self.program[instruction_pointer]).unwrap();

            let literal_operand = self.program[instruction_pointer + 1];
            let combo_operand = match literal_operand {
                0..=3 => literal_operand as usize,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                _ => panic!("Unexpected combo operand identifier {}", literal_operand),
            };

            match instruction {
                Instruction::Adv => {
                    self.register_a = self.register_a / (combo_operand.pow(2)) as usize
                }
                Instruction::Bxl => {
                    self.register_b = self.register_b.bitxor(literal_operand as usize)
                }
                Instruction::Bst => self.register_b = combo_operand % 8,
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        instruction_pointer = (literal_operand / 2) as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.register_b = self.register_b.bitxor(self.register_c),
                Instruction::Out => {
                    self.output.push((combo_operand % 8) as u8);
                }
                Instruction::Bdv => {
                    self.register_b = self.register_a / (combo_operand.pow(2)) as usize
                }
                Instruction::Cdv => {
                    self.register_c = self.register_a / (combo_operand.pow(2)) as usize
                }
            }

            instruction_pointer += 2;
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day17.txt").expect("Failed to read file");

    let result = 0;

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "#;

        let mut computer = Computer::parse(input);

        assert_eq!(computer.register_a, 729);
        assert_eq!(computer.register_b, 0);
        assert_eq!(computer.register_c, 0);
        assert_eq!(computer.program, vec![0, 1, 5, 4, 3, 0]);

        computer.execute();

        assert_eq!(computer.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
