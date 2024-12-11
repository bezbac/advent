use std::fs;

use memoize::memoize;

#[memoize]
fn parts_after_steps(stone: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return parts_after_steps(1, steps - 1);
    }

    let string = format!("{stone}");
    if string.len() % 2 == 0 {
        let middle = string.len() / 2;
        let iter = string.chars();
        let a = iter.clone().take(middle).collect::<String>();
        let b = iter.skip(middle).collect::<String>();

        return parts_after_steps(a.parse().unwrap(), steps - 1)
            + parts_after_steps(b.parse().unwrap(), steps - 1);
    }

    parts_after_steps(stone * 2024, steps - 1)
}

fn arr_parts_after_steps(stones: &[usize], steps: usize) -> usize {
    stones
        .iter()
        .map(|stone| parts_after_steps(*stone, steps))
        .sum()
}

fn main() {
    let input = fs::read_to_string("./inputs/day11.txt").expect("Failed to read file");

    let stones = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let result: usize = arr_parts_after_steps(&stones, 25);

    println!("Result (Part 1): {result}");

    let result: usize = arr_parts_after_steps(&stones, 75);

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(
            arr_parts_after_steps(&[0, 1, 10, 99, 999], 1),
            7, // 1, 2024, 1, 0, 9, 9, 2021976
        );

        assert_eq!(
            arr_parts_after_steps(&[125, 17], 1),
            3 // 253000, 1, 7
        );
        assert_eq!(
            arr_parts_after_steps(&[253000, 1, 7], 1),
            4, // 253, 0, 2024, 14168
        );
        assert_eq!(
            arr_parts_after_steps(&[253, 0, 2024, 14168], 1),
            5, // 512072, 1, 20, 24, 28676032
        );
        assert_eq!(
            arr_parts_after_steps(&[512072, 1, 20, 24, 28676032], 1),
            9, // 512, 72, 2024, 2, 0, 2, 4, 2867, 6032
        );
        assert_eq!(
            arr_parts_after_steps(&[512, 72, 2024, 2, 0, 2, 4, 2867, 6032], 1),
            13, // 1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
        );
        assert_eq!(
            arr_parts_after_steps(
                &[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
                1
            ),
            22, // 2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2
        );
    }
}
