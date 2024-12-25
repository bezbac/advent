use std::{collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq)]
struct Lock(Vec<isize>);

#[derive(Debug, Clone, PartialEq)]
struct Key(Vec<isize>);

#[derive(Debug, Clone, PartialEq)]
enum KeyOrLock {
    Lock(Lock),
    Key(Key),
}

impl KeyOrLock {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let is_key = input.lines().next().unwrap().chars().all(|c| c == '#');

        let mut values = vec![];
        if is_key {
            for line in input.lines().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if values.get(i).is_none() {
                        values.insert(i, 0);
                    }

                    if c == '#' {
                        values[i] += 1;
                    }
                }
            }

            return Self::Lock(Lock(values));
        } else {
            for line in input.lines().rev().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if values.get(i).is_none() {
                        values.insert(i, 0);
                    }

                    if c == '#' {
                        values[i] += 1;
                    }
                }
            }

            return Self::Key(Key(values));
        }
    }
}

fn count_combinations(input: &[KeyOrLock]) -> usize {
    let keys: Vec<Key> = input
        .iter()
        .filter_map(|k| {
            if let KeyOrLock::Key(k) = k {
                return Some(k.clone());
            } else {
                return None;
            }
        })
        .collect();

    let locks: Vec<Lock> = input
        .iter()
        .filter_map(|k| {
            if let KeyOrLock::Lock(l) = k {
                return Some(l.clone());
            } else {
                return None;
            }
        })
        .collect();

    let target = keys.iter().flat_map(|k| k.0.iter().copied()).max().unwrap();

    let mut result = 0;

    for key in keys {
        'lockloop: for lock in &locks {
            for i in 0..key.0.len() {
                let k = key.0[i];
                let l = lock.0[i];

                if k + l > target {
                    continue 'lockloop;
                }
            }

            result += 1;
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day25.txt").expect("Failed to read file");

    let objects: Vec<KeyOrLock> = input
        .trim()
        .split("\n\n")
        .map(|text| KeyOrLock::parse(text))
        .collect();

    let result = count_combinations(&objects);

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key() {
        let input = r#"
#####
.####
.####
.####
.#.#.
.#...
.....
        "#;

        let r = KeyOrLock::parse(&input);

        assert_eq!(r, KeyOrLock::Lock(Lock(vec![0, 5, 3, 4, 3])));
    }

    #[test]
    fn test_parse_lock() {
        let input = r#"
.....
#....
#....
#...#
#.#.#
#.###
#####
        "#;

        let r = KeyOrLock::parse(&input);

        assert_eq!(r, KeyOrLock::Key(Key(vec![5, 0, 2, 1, 3])));
    }

    #[test]
    fn test_count_combinations() {
        let input = vec![
            KeyOrLock::Lock(Lock(vec![0, 5, 3, 4, 3])),
            KeyOrLock::Lock(Lock(vec![1, 2, 0, 5, 3])),
            KeyOrLock::Key(Key(vec![5, 0, 2, 1, 3])),
            KeyOrLock::Key(Key(vec![4, 3, 4, 0, 2])),
            KeyOrLock::Key(Key(vec![3, 0, 2, 0, 1])),
        ];

        assert_eq!(count_combinations(&input), 3);
    }
}
