use std::fs;

fn step(stones: &[usize]) -> Vec<usize> {
    let mut result = vec![];

    for stone in stones {
        if stone == &0 {
            result.push(1);
            continue;
        }

        let string = format!("{}", stone);
        if string.len() % 2 == 0 {
            let middle = string.len() / 2;
            let iter = string.chars();
            let a = iter.clone().take(middle).collect::<String>();
            let b = iter.skip(middle).collect::<String>();
            result.push(a.parse().unwrap());
            result.push(b.parse().unwrap());
            continue;
        }

        result.push(stone * 2024);
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day11.txt").expect("Failed to read file");

    let stones = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut evolved = stones.clone();

    for _ in 0..25 {
        let new_stones = step(&evolved);
        evolved = new_stones;
    }

    let result = evolved.len();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(
            step(&[0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );

        assert_eq!(step(&[125, 17]), vec![253000, 1, 7]);
        assert_eq!(step(&[253000, 1, 7]), vec![253, 0, 2024, 14168]);
        assert_eq!(
            step(&[253, 0, 2024, 14168]),
            vec![512072, 1, 20, 24, 28676032]
        );
        assert_eq!(
            step(&[512072, 1, 20, 24, 28676032]),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            step(&[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            step(&[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }
}
