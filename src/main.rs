use std::fs;

#[derive(PartialEq, Debug)]
struct MultiplicationInstruction {
    a: i64,
    b: i64,
}

fn parse(input: &str) -> Vec<MultiplicationInstruction> {
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

            result.push(MultiplicationInstruction {
                a: a.iter().collect::<String>().parse().unwrap(),
                b: b.iter().collect::<String>().parse().unwrap(),
            });
        }
    }

    result
}

fn execute(instructions: &[MultiplicationInstruction]) -> i64 {
    instructions
        .iter()
        .fold(0, |acc, instruction| acc + instruction.a * instruction.b)
}

fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").expect("Failed to read file");

    let result = execute(&parse(&input));

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec![
                MultiplicationInstruction { a: 2, b: 4 },
                MultiplicationInstruction { a: 5, b: 5 },
                MultiplicationInstruction { a: 11, b: 8 },
                MultiplicationInstruction { a: 8, b: 5 },
            ]
        );
    }

    #[test]
    fn test_parse_and_run() {
        assert_eq!(
            execute(&[
                MultiplicationInstruction { a: 2, b: 4 },
                MultiplicationInstruction { a: 5, b: 5 },
                MultiplicationInstruction { a: 11, b: 8 },
                MultiplicationInstruction { a: 8, b: 5 },
            ]),
            161
        );
    }
}
