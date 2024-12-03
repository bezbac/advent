use std::fs;

#[derive(PartialEq, Debug)]
struct MultiplicationInstruction {
    a: i64,
    b: i64,
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Multiplication(MultiplicationInstruction),
    Do,
    Dont,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    let mut iter = input.chars().peekable();

    while let Some(char) = iter.next() {
        if char == 'm' {
            // Expect next to be 'u'
            let next = iter.next();
            if next != Some('u') {
                continue;
            }

            // Expect next to be 'l'
            let next = iter.next();
            if next != Some('l') {
                continue;
            }

            // Expect next to be '('
            let next = iter.next();
            if next != Some('(') {
                continue;
            }

            let mut a = Vec::new();
            'num: while let Some(char) = iter.peek() {
                if char.is_ascii_digit() {
                    a.push(*char);
                    iter.next();
                } else {
                    break 'num;
                }
            }

            // Expect next to be ','
            let next = iter.next();
            if next != Some(',') {
                continue;
            }

            let mut b = Vec::new();
            'num: while let Some(char) = iter.peek() {
                if char.is_ascii_digit() {
                    b.push(*char);
                    iter.next();
                } else {
                    break 'num;
                }
            }

            // Expect next to be ')'
            let next = iter.next();
            if next != Some(')') {
                continue;
            }

            result.push(Instruction::Multiplication(MultiplicationInstruction {
                a: a.iter().collect::<String>().parse().unwrap(),
                b: b.iter().collect::<String>().parse().unwrap(),
            }));
        }

        if char == 'd' {
            // Expect next to be 'o'
            let next = iter.next();
            if next != Some('o') {
                continue;
            }

            let next = iter.next();

            if next == Some('(') {
                // Expect next to be ')'
                let next = iter.next();
                if next != Some(')') {
                    continue;
                }

                result.push(Instruction::Do);
            } else if next == Some('n') {
                // Expect next to be '''
                let next = iter.next();
                if next != Some('\'') {
                    continue;
                }

                // Expect next to be 't'
                let next = iter.next();
                if next != Some('t') {
                    continue;
                }

                // Expect next to be '('
                let next = iter.next();
                if next != Some('(') {
                    continue;
                }

                // Expect next to be ')'
                let next = iter.next();
                if next != Some(')') {
                    continue;
                }

                result.push(Instruction::Dont);
            } else {
                continue;
            }
        }
    }

    result
}

fn execute(instructions: &[Instruction], respect_dos: bool) -> i64 {
    let mut result = 0;

    let mut enabled = true;

    for instruction in instructions {
        match instruction {
            Instruction::Multiplication(multiplication) => {
                if enabled {
                    result += multiplication.a * multiplication.b;
                }
            }
            Instruction::Do => {
                if respect_dos {
                    enabled = true;
                }
            }
            Instruction::Dont => {
                if respect_dos {
                    enabled = false;
                }
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").expect("Failed to read file");

    let result = execute(&parse(&input), false);
    println!("Result (Part 1): {result}");

    let result = execute(&parse(&input), true);
    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec![
                Instruction::Multiplication(MultiplicationInstruction { a: 2, b: 4 }),
                Instruction::Multiplication(MultiplicationInstruction { a: 5, b: 5 }),
                Instruction::Multiplication(MultiplicationInstruction { a: 11, b: 8 }),
                Instruction::Multiplication(MultiplicationInstruction { a: 8, b: 5 }),
            ]
        );
    }

    #[test]
    fn test_parse_with_dos() {
        assert_eq!(
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            vec![
                Instruction::Multiplication(MultiplicationInstruction { a: 2, b: 4 }),
                Instruction::Dont,
                Instruction::Multiplication(MultiplicationInstruction { a: 5, b: 5 }),
                Instruction::Multiplication(MultiplicationInstruction { a: 11, b: 8 }),
                Instruction::Do,
                Instruction::Multiplication(MultiplicationInstruction { a: 8, b: 5 }),
            ]
        );
    }

    #[test]
    fn test_parse_and_run() {
        assert_eq!(
            execute(
                &[
                    Instruction::Multiplication(MultiplicationInstruction { a: 2, b: 4 }),
                    Instruction::Dont,
                    Instruction::Multiplication(MultiplicationInstruction { a: 5, b: 5 }),
                    Instruction::Multiplication(MultiplicationInstruction { a: 11, b: 8 }),
                    Instruction::Do,
                    Instruction::Multiplication(MultiplicationInstruction { a: 8, b: 5 }),
                ],
                false
            ),
            161
        );
    }

    #[test]
    fn test_parse_and_run_with_dos() {
        assert_eq!(
            execute(
                &[
                    Instruction::Multiplication(MultiplicationInstruction { a: 2, b: 4 }),
                    Instruction::Dont,
                    Instruction::Multiplication(MultiplicationInstruction { a: 5, b: 5 }),
                    Instruction::Multiplication(MultiplicationInstruction { a: 11, b: 8 }),
                    Instruction::Do,
                    Instruction::Multiplication(MultiplicationInstruction { a: 8, b: 5 }),
                ],
                true
            ),
            48
        );
    }
}
