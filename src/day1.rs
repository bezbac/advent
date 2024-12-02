use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day1.txt").expect("Failed to read file");

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let mut split = line.trim().split("   ");

        let a_num: isize = split
            .next()
            .expect("Could not read first number in line")
            .parse()?;

        let b_num: isize = split
            .next()
            .expect("Could not read second number in line")
            .parse()?;

        a.push(a_num);
        b.push(b_num);
    }

    a.sort_unstable();
    b.sort_unstable();

    let result: isize = a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("Result (Part 1): {result}");

    let occurences_of_a_in_b: HashMap<isize, isize> =
        b.iter().fold(HashMap::new(), |mut acc, x| {
            let entry = acc.entry(*x).or_insert(0);
            *entry += 1;
            acc
        });

    let result: isize = a
        .iter()
        .map(|x| {
            // Multiply the number from a with the number of occurences in b
            let factor = occurences_of_a_in_b.get(x).unwrap_or(&0);
            x * factor
        })
        .sum();

    println!("Result (Part 2): {result}");

    Ok(())
}
