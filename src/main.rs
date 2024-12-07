use anyhow::Result;
use std::{collections::HashMap, fs};

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

fn check_equation(result: usize, operands: &[usize]) -> bool {
    if operands.len() == 1 {
        return operands[0] == result;
    }

    let mut permutations: Vec<Vec<Operator>> = vec![vec![Operator::Add], vec![Operator::Multiply]];
    for _ in 1..operands.len() - 1 {
        let mut new_permutations = vec![];

        for permutation in permutations {
            let mut new_permutation = permutation.clone();
            new_permutation.push(Operator::Add);
            new_permutations.push(new_permutation);

            let mut new_permutation = permutation.clone();
            new_permutation.push(Operator::Multiply);
            new_permutations.push(new_permutation);
        }

        permutations = new_permutations;
    }

    return permutations.iter().any(|operators| {
        let mut calc_result = operands[0];

        for i in 1..operands.len() {
            let operand = operands[i];
            let operator = operators[i - 1];

            match operator {
                Operator::Add => calc_result += operand,
                Operator::Multiply => calc_result *= operand,
            }
        }

        calc_result == result
    });
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day7.txt").expect("Failed to read file");

    let mut result = 0;

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

        let is_valid = check_equation(eq_result, &operands);

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
    fn test_check_equation() {
        assert_eq!(check_equation(2, &[1, 1]), true);
        assert_eq!(check_equation(8, &[2, 4]), true);
        assert_eq!(check_equation(8, &[2, 2, 2]), true);
        assert_eq!(check_equation(16, &[2, 2, 2, 2]), true);
        assert_eq!(check_equation(18, &[2, 2, 2, 2, 2]), true);

        // From the example
        assert_eq!(check_equation(190, &[10, 19]), true);
        assert_eq!(check_equation(3267, &[81, 40, 27]), true);
        assert_eq!(check_equation(83, &[17, 5]), false);
        assert_eq!(check_equation(156, &[15, 6]), false);
        assert_eq!(check_equation(7290, &[6, 8, 6, 15]), false);
        assert_eq!(check_equation(161011, &[16, 10, 13]), false);
        assert_eq!(check_equation(192, &[17, 8, 14]), false);
        assert_eq!(check_equation(21037, &[9, 7, 18, 13]), false);
        assert_eq!(check_equation(292, &[11, 6, 16, 20]), true);
    }
}
