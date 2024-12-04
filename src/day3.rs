use std::fs;

#[derive(PartialEq, Debug)]
enum Instruction {
    Multiplication { a: i64, b: i64 },
    Do,
    Dont,
}

macro_rules! expect_char {
    ($iter:ident, $char:literal) => {
        match $iter.next() {
            Some(char) if char == $char => {}
            _ => continue,
        }
    };
}

fn parse_number(iter: &mut std::iter::Peekable<std::str::Chars>) -> i64 {
    let mut num_chars = Vec::new();
    'num: while let Some(char) = iter.peek() {
        if char.is_ascii_digit() {
            num_chars.push(*char);
            iter.next();
        } else {
            break 'num;
        }
    }

    num_chars.iter().collect::<String>().parse().unwrap()
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    let mut iter = input.chars().peekable();

    while let Some(char) = iter.next() {
        if char == 'm' {
            expect_char!(iter, 'u');
            expect_char!(iter, 'l');
            expect_char!(iter, '(');

            let a = parse_number(&mut iter);

            expect_char!(iter, ',');

            let b = parse_number(&mut iter);

            expect_char!(iter, ')');

            result.push(Instruction::Multiplication { a, b });
        }

        if char == 'd' {
            expect_char!(iter, 'o');

            let next = iter.next();

            if next == Some('(') {
                expect_char!(iter, ')');
                result.push(Instruction::Do);
            } else if next == Some('n') {
                expect_char!(iter, '\'');
                expect_char!(iter, 't');
                expect_char!(iter, '(');
                expect_char!(iter, ')');
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
            Instruction::Multiplication { a, b } => {
                if enabled {
                    result += a * b;
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
                Instruction::Multiplication { a: 2, b: 4 },
                Instruction::Multiplication { a: 5, b: 5 },
                Instruction::Multiplication { a: 11, b: 8 },
                Instruction::Multiplication { a: 8, b: 5 },
            ]
        );
    }

    #[test]
    fn test_parse_with_dos() {
        assert_eq!(
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            vec![
                Instruction::Multiplication { a: 2, b: 4 },
                Instruction::Dont,
                Instruction::Multiplication { a: 5, b: 5 },
                Instruction::Multiplication { a: 11, b: 8 },
                Instruction::Do,
                Instruction::Multiplication { a: 8, b: 5 },
            ]
        );
    }

    #[test]
    fn test_parse_and_run() {
        assert_eq!(
            execute(
                &[
                    Instruction::Multiplication { a: 2, b: 4 },
                    Instruction::Dont,
                    Instruction::Multiplication { a: 5, b: 5 },
                    Instruction::Multiplication { a: 11, b: 8 },
                    Instruction::Do,
                    Instruction::Multiplication { a: 8, b: 5 },
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
                    Instruction::Multiplication { a: 2, b: 4 },
                    Instruction::Dont,
                    Instruction::Multiplication { a: 5, b: 5 },
                    Instruction::Multiplication { a: 11, b: 8 },
                    Instruction::Do,
                    Instruction::Multiplication { a: 8, b: 5 },
                ],
                true
            ),
            48
        );
    }
}
