use anyhow::Result;
use std::fs;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn check_permutation(result: usize, operands: &[usize], operators: &[Operator]) -> bool {
    let mut calc_result = operands[0];

    for i in 1..operands.len() {
        let operand = operands[i];
        let operator = operators[i - 1];

        match operator {
            Operator::Add => calc_result += operand,
            Operator::Multiply => calc_result *= operand,
            Operator::Concat => {
                calc_result = format!("{}{}", calc_result, operand).parse().unwrap();
            }
        }
    }

    calc_result == result
}

fn check_equation(result: usize, operands: &[usize], operators: &[Operator]) -> bool {
    if operands.len() == 1 {
        return operands[0] == result;
    }

    let mut operators = operators.to_vec();
    operators.sort();
    operators.dedup();

    let mut permutations: Vec<Vec<Operator>> =
        operators.iter().map(|operator| vec![*operator]).collect();

    for _ in 1..operands.len() - 1 {
        let mut new_permutations = vec![];

        for permutation in permutations {
            for operator in &operators {
                let mut new_permutation = permutation.clone();
                new_permutation.push(*operator);
                new_permutations.push(new_permutation);
            }
        }

        permutations = new_permutations;
    }

    return permutations
        .iter()
        .any(|operators| check_permutation(result, operands, operators));
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day7.txt").expect("Failed to read file");

    let mut parsed = vec![];

    for line in input.lines() {
        let mut split = line.trim().split(": ");

        let eq_result = split
            .next()
            .expect("Could not read result in line")
            .parse::<usize>()
            .expect("Could not parse result in line");

        let mut operands = vec![];

        let mut split = split
            .next()
            .expect("Could not read operands in line")
            .split_whitespace();

        while let Some(operand) = split.next() {
            let operand = operand
                .parse::<usize>()
                .expect("Could not parse operand in line");
            operands.push(operand);
        }

        parsed.push((eq_result, operands));
    }

    let mut result = 0;

    for (eq_result, operands) in &parsed {
        let is_valid = check_equation(*eq_result, &operands, &[Operator::Add, Operator::Multiply]);

        if is_valid {
            result += eq_result;
        }
    }

    println!("Result (Part 1): {result}");

    let mut result = 0;

    for (eq_result, operands) in &parsed {
        let is_valid = check_equation(
            *eq_result,
            &operands,
            &[Operator::Add, Operator::Multiply, Operator::Concat],
        );

        if is_valid {
            result += eq_result;
        }
    }

    println!("Result (Part 1): {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_equation_add_mul() {
        let operators = &[Operator::Add, Operator::Multiply];

        assert_eq!(check_equation(2, &[1, 1], operators), true);
        assert_eq!(check_equation(8, &[2, 4], operators), true);
        assert_eq!(check_equation(8, &[2, 2, 2], operators), true);
        assert_eq!(check_equation(16, &[2, 2, 2, 2], operators), true);
        assert_eq!(check_equation(18, &[2, 2, 2, 2, 2], operators), true);

        // From the example
        assert_eq!(check_equation(190, &[10, 19], operators), true);
        assert_eq!(check_equation(3267, &[81, 40, 27], operators), true);
        assert_eq!(check_equation(83, &[17, 5], operators), false);
        assert_eq!(check_equation(156, &[15, 6], operators), false);
        assert_eq!(check_equation(7290, &[6, 8, 6, 15], operators), false);
        assert_eq!(check_equation(161011, &[16, 10, 13], operators), false);
        assert_eq!(check_equation(192, &[17, 8, 14], operators), false);
        assert_eq!(check_equation(21037, &[9, 7, 18, 13], operators), false);
        assert_eq!(check_equation(292, &[11, 6, 16, 20], operators), true);
    }

    #[test]
    fn test_check_equation_all_ops() {
        let operators = &[Operator::Add, Operator::Multiply, Operator::Concat];

        assert_eq!(check_equation(2, &[1, 1], operators), true);
        assert_eq!(check_equation(11, &[1, 1], operators), true);
        assert_eq!(check_equation(8, &[2, 4], operators), true);
        assert_eq!(check_equation(8, &[2, 2, 2], operators), true);
        assert_eq!(check_equation(222, &[2, 2, 2], operators), true);
        assert_eq!(check_equation(16, &[2, 2, 2, 2], operators), true);
        assert_eq!(check_equation(18, &[2, 2, 2, 2, 2], operators), true);

        // From the example
        assert_eq!(check_equation(190, &[10, 19], operators), true);
        assert_eq!(check_equation(3267, &[81, 40, 27], operators), true);
        assert_eq!(check_equation(83, &[17, 5], operators), false);
        assert_eq!(check_equation(156, &[15, 6], operators), true);
        assert_eq!(check_equation(7290, &[6, 8, 6, 15], operators), true);
        assert_eq!(check_equation(161011, &[16, 10, 13], operators), false);
        assert_eq!(check_equation(192, &[17, 8, 14], operators), true);
        assert_eq!(check_equation(21037, &[9, 7, 18, 13], operators), false);
        assert_eq!(check_equation(292, &[11, 6, 16, 20], operators), true);
    }
}
