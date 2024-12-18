use std::{collections::VecDeque, fs, ops::BitXor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
enum u3 {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
}

impl BitXor for u3 {
    type Output = u3;

    fn bitxor(self, rhs: Self) -> Self::Output {
        u3::try_from(u8::from(self).bitxor(u8::from(rhs))).unwrap()
    }
}

impl TryFrom<u8> for u3 {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::ZERO,
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            4 => Self::FOUR,
            5 => Self::FIVE,
            6 => Self::SIX,
            7 => Self::SEVEN,
            _ => return Err(()),
        })
    }
}

impl TryFrom<usize> for u3 {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(value as u8)
    }
}

impl From<u3> for u8 {
    fn from(value: u3) -> Self {
        match value {
            u3::ZERO => 0,
            u3::ONE => 1,
            u3::TWO => 2,
            u3::THREE => 3,
            u3::FOUR => 4,
            u3::FIVE => 5,
            u3::SIX => 6,
            u3::SEVEN => 7,
        }
    }
}

impl From<u3> for usize {
    fn from(value: u3) -> Self {
        u8::from(value) as usize
    }
}

impl From<u3> for isize {
    fn from(value: u3) -> Self {
        u8::from(value) as isize
    }
}

#[derive(Debug)]
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
        u3::try_from(value).and_then(|x| Self::try_from(x))
    }
}

impl TryFrom<u3> for Instruction {
    type Error = ();

    fn try_from(value: u3) -> Result<Self, Self::Error> {
        Ok(match value {
            u3::ZERO => Instruction::Adv,
            u3::ONE => Instruction::Bxl,
            u3::TWO => Instruction::Bst,
            u3::THREE => Instruction::Jnz,
            u3::FOUR => Instruction::Bxc,
            u3::FIVE => Instruction::Out,
            u3::SIX => Instruction::Bdv,
            u3::SEVEN => Instruction::Cdv,
        })
    }
}

#[derive(Debug)]
struct Computer {
    register_a: isize,
    register_b: isize,
    register_c: isize,
    program: VecDeque<u3>,
    output: Vec<u8>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let mut lines = input.lines();

        let line = lines.next().unwrap().trim();
        let line = line.trim_start_matches("Register A: ");
        let register_a = line.parse().unwrap();

        let line = lines.next().unwrap().trim();
        let line = line.trim_start_matches("Register B: ");
        let register_b = line.parse().unwrap();

        let line = lines.next().unwrap().trim();
        let line = line.trim_start_matches("Register C: ");
        let register_c = line.parse().unwrap();

        lines.next();

        let line = lines.next().unwrap().trim();
        let line = line.trim_start_matches("Program: ");

        let program: VecDeque<_> = line
            .trim()
            .split(',')
            .map(|c| {
                let value: u8 = c.parse().unwrap();
                u3::try_from(value).unwrap()
            })
            .collect();

        Self {
            register_a,
            register_b,
            register_c,
            program,
            output: Vec::new(),
        }
    }

    fn get_combo_operand(&self, literal_operand: u3) -> isize {
        match literal_operand {
            u3::ZERO | u3::ONE | u3::TWO | u3::THREE => literal_operand.into(),
            u3::FOUR => self.register_a,
            u3::FIVE => self.register_b,
            u3::SIX => self.register_c,
            u3::SEVEN => {
                panic!("Literal operand 7 is reserved and has no corresponing combo operand")
            }
        }
    }
}

#[derive(Debug)]
struct Execution {
    computer: Computer,
    instruction_pointer: usize,
}

impl Execution {
    fn run(&mut self) {
        while self.instruction_pointer + 1 < self.computer.program.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction =
            Instruction::try_from(self.computer.program[self.instruction_pointer]).unwrap();

        let literal_operand = self.computer.program[self.instruction_pointer + 1];

        match instruction {
            Instruction::Adv => {
                self.computer.register_a = self.computer.register_a
                    / (2_isize.pow(self.computer.get_combo_operand(literal_operand) as u32))
                        as isize
            }
            Instruction::Bxl => {
                self.computer.register_b = self.computer.register_b ^ isize::from(literal_operand);
            }
            Instruction::Bst => {
                self.computer.register_b =
                    (self.computer.get_combo_operand(literal_operand) % 8) as isize
            }
            Instruction::Jnz => {
                if self.computer.register_a != 0 {
                    self.instruction_pointer = (u8::from(literal_operand) / 2) as usize;
                    return;
                }
            }
            Instruction::Bxc => {
                self.computer.register_b = self.computer.register_b ^ self.computer.register_c
            }
            Instruction::Out => {
                self.computer
                    .output
                    .push((self.computer.get_combo_operand(literal_operand) % 8) as u8);
            }
            Instruction::Bdv => {
                self.computer.register_b = self.computer.register_a
                    / (2_isize.pow(self.computer.get_combo_operand(literal_operand) as u32))
                        as isize
            }
            Instruction::Cdv => {
                self.computer.register_c = self.computer.register_a
                    / (2_isize.pow(self.computer.get_combo_operand(literal_operand) as u32))
                        as isize
            }
        }

        self.instruction_pointer += 2;
    }
}

impl From<Computer> for Execution {
    fn from(value: Computer) -> Self {
        Self {
            computer: value,
            instruction_pointer: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day17.txt").expect("Failed to read file");

    let computer = Computer::parse(&input);

    let mut execution = Execution::from(computer);

    execution.run();

    let result = execution
        .computer
        .output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u3_from_u8() {
        assert_eq!(u3::try_from(0_u8).unwrap() as u8, 0b00000000);
        assert_eq!(u3::try_from(1_u8).unwrap() as u8, 0b00000001);
        assert_eq!(u3::try_from(2_u8).unwrap() as u8, 0b00000010);
        assert_eq!(u3::try_from(3_u8).unwrap() as u8, 0b00000011);
        assert_eq!(u3::try_from(4_u8).unwrap() as u8, 0b00000100);
        assert_eq!(u3::try_from(5_u8).unwrap() as u8, 0b00000101);
        assert_eq!(u3::try_from(6_u8).unwrap() as u8, 0b00000110);
        assert_eq!(u3::try_from(7_u8).unwrap() as u8, 0b00000111);
        assert_eq!(u3::try_from(8_u8).is_err(), true);
    }

    #[test]
    fn test_instructions_one() {
        let computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            output: vec![],
            program: vec![2, 6]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect(),
        };

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(execution.computer.register_b, 1);
    }

    #[test]
    fn test_instructions_two() {
        let computer = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            output: vec![],
            program: vec![5, 0, 5, 1, 5, 4]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect(),
        };

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(execution.computer.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_instructions_three() {
        let computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            output: vec![],
            program: vec![0, 1, 5, 4, 3, 0]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect(),
        };

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(
            execution.computer.output,
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(execution.computer.register_a, 0);
    }

    #[test]
    fn test_instructions_four() {
        let computer = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            output: vec![],
            program: vec![1, 7]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect(),
        };

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(execution.computer.register_b, 26);
    }

    #[test]
    fn test_instructions_five() {
        let computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            output: vec![],
            program: vec![4, 0]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect(),
        };

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(execution.computer.register_b, 44354);
    }

    #[test]
    fn test_one() {
        let input = r#"
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
            "#;

        let computer = Computer::parse(input);

        assert_eq!(computer.register_a, 729);
        assert_eq!(computer.register_b, 0);
        assert_eq!(computer.register_c, 0);
        assert_eq!(
            computer.program,
            vec![0, 1, 5, 4, 3, 0]
                .into_iter()
                .map(|x| u3::try_from(x as u8).unwrap())
                .collect::<VecDeque<_>>()
        );

        let mut execution: Execution = computer.into();

        execution.run();

        assert_eq!(
            execution.computer.output,
            vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
        );
    }
}
