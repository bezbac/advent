use std::{collections::HashMap, fs};

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

    println!("Result (Part 1): {}", result);

    let mut occurences_of_a_in_b: HashMap<usize, usize> = HashMap::new();

    for x in b {
        let entry = occurences_of_a_in_b.entry(x).or_insert(0);
        *entry += 1;
    }

    let result: isize = a
        .iter()
        .map(|x| {
            // Multiply the number from a with the number of occurences in b
            let factor = *occurences_of_a_in_b.get(x).unwrap_or(&0) as isize;
            *x as isize * factor
        })
        .sum();

    println!("Result (Part 2): {}", result);
}
