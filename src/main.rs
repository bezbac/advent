use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day1.txt").expect("Failed to read file");

    let numbers: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let mut split = line.trim().split("   ");

            let a = split.next().unwrap();
            let b = split.next().unwrap();

            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            (a, b)
        })
        .collect();

    let mut a: Vec<usize> = numbers.clone().into_iter().map(|(a, _)| a).collect();
    let mut b: Vec<usize> = numbers.clone().into_iter().map(|(_, b)| b).collect();

    a.sort();
    b.sort();

    let result: isize = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (*a as isize - *b as isize).abs())
        .sum();

    println!("Result: {}", result);
}
