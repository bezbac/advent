use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_adjecency_list(input: &str) -> HashMap<usize, HashSet<usize>> {
    let mut result: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('|');
        let from = parts.next().unwrap().trim().parse().unwrap();
        let to = parts.next().unwrap().trim().parse().unwrap();

        result.entry(from).or_default().insert(to);
    }

    result
}

fn reverse_adjecency_list(
    adjecency_list: &HashMap<usize, HashSet<usize>>,
) -> HashMap<usize, HashSet<usize>> {
    let mut result: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (from, tos) in adjecency_list {
        for to in tos {
            result.entry(*to).or_default().insert(*from);
        }
    }

    result
}

fn is_valid_ordering(adjecency_list: &HashMap<usize, HashSet<usize>>, ordering: &[usize]) -> bool {
    let reverse_adjecency_list = reverse_adjecency_list(adjecency_list);

    for i in 1..ordering.len() {
        let num = ordering[i];

        {
            // Check forward
            let remaining = &ordering[i + 1..];
            let limiters = reverse_adjecency_list.get(&num);

            if let Some(limiters) = limiters {
                for limiter in limiters {
                    if remaining.contains(limiter) {
                        return false;
                    }
                }
            }
        }

        {
            // Check backward
            let checked = &ordering[0..i];
            let limiters = adjecency_list.get(&num);

            if let Some(limiters) = limiters {
                for limiter in limiters {
                    if checked.contains(limiter) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn calculate_valid_ordering_checksum(
    adjecency_list: &HashMap<usize, HashSet<usize>>,
    orderings: &[Vec<usize>],
) -> usize {
    let mut result = 0;

    for ordering in orderings {
        if is_valid_ordering(adjecency_list, ordering) {
            assert!(ordering.len() % 2 == 1, "Odering must have odd length");
            let middle_num = ordering[ordering.len() / 2];
            result += middle_num;
        }
    }

    result
}

fn correct_ordering(
    adjecency_list: &HashMap<usize, HashSet<usize>>,
    ordering: &[usize],
) -> Vec<usize> {
    let mut remaining = ordering.to_vec();

    let mut result = vec![];

    result.push(remaining.pop().unwrap());

    while let Some(num) = remaining.pop() {
        let mut after_idx = 0;
        for (idx, n) in result.iter().enumerate() {
            if let Some(l) = adjecency_list.get(n) {
                if l.contains(&num) {
                    // Number must come after result[i]
                    after_idx = idx + 1;
                }
            }
        }

        result.insert(after_idx, num);
    }

    result
}

fn calculate_invalid_ordering_checksum(
    adjecency_list: &HashMap<usize, HashSet<usize>>,
    orderings: &[Vec<usize>],
) -> usize {
    let mut result = 0;

    for ordering in orderings {
        if !is_valid_ordering(adjecency_list, ordering) {
            let sorted = correct_ordering(&adjecency_list, &ordering);
            assert!(
                is_valid_ordering(adjecency_list, &sorted),
                "Encountered invalid ordering {:?}",
                sorted
            );
            assert!(sorted.len() % 2 == 1, "Odering must have odd length");
            let middle_num = sorted[sorted.len() / 2];
            result += middle_num;
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day5.txt").expect("Failed to read file");

    let mut split: std::str::Split<'_, &str> = input.split("\n\n");

    let adjecency_list = parse_adjecency_list(split.next().unwrap().trim());

    let mut orderings: Vec<Vec<usize>> = vec![];
    for line in split.next().unwrap().lines() {
        let mut ordering = vec![];

        for x in line.split(',') {
            ordering.push(x.parse().unwrap());
        }

        orderings.push(ordering);
    }

    let result = calculate_valid_ordering_checksum(&adjecency_list, &orderings);

    println!("Result (Part 1): {result}");

    let result = calculate_invalid_ordering_checksum(&adjecency_list, &orderings);

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_adjecency_list() {
        let input = r#"
47|53
97|13
97|61
        "#
        .trim();

        let adjecency_list = parse_adjecency_list(input);
        let expected: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (47, vec![53].into_iter().collect::<HashSet<_>>()),
            (97, vec![13, 61].into_iter().collect::<HashSet<_>>()),
        ]);

        assert_eq!(adjecency_list, expected);
    }

    #[test]
    fn test_reverse_adjecency_list() {
        let adjecency_list: HashMap<usize, HashSet<usize>> =
            HashMap::from_iter(vec![(97, vec![75, 61].into_iter().collect::<HashSet<_>>())]);

        let reversed = reverse_adjecency_list(&adjecency_list);

        let expected: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (75, vec![97].into_iter().collect::<HashSet<_>>()),
            (61, vec![97].into_iter().collect::<HashSet<_>>()),
        ]);

        assert_eq!(reversed, expected);
    }

    #[test]
    fn test_is_valid_ordering() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (75, vec![47, 61, 53, 29].into_iter().collect::<HashSet<_>>()),
            (47, vec![61, 53, 29].into_iter().collect::<HashSet<_>>()),
            (61, vec![53, 29].into_iter().collect::<HashSet<_>>()),
            (53, vec![29].into_iter().collect::<HashSet<_>>()),
            (97, vec![75].into_iter().collect::<HashSet<_>>()),
            (29, vec![13].into_iter().collect::<HashSet<_>>()),
        ]);

        assert_eq!(is_valid_ordering(&adjecency_list, &[75, 47]), true);
        assert_eq!(is_valid_ordering(&adjecency_list, &[47, 61]), true);
        assert_eq!(is_valid_ordering(&adjecency_list, &[61, 53]), true);

        assert_eq!(is_valid_ordering(&adjecency_list, &[47, 75]), false);
        assert_eq!(is_valid_ordering(&adjecency_list, &[61, 47]), false);
        assert_eq!(is_valid_ordering(&adjecency_list, &[53, 61]), false);

        assert_eq!(
            is_valid_ordering(&adjecency_list, &[75, 47, 61, 53, 29]),
            true
        );
        assert_eq!(
            is_valid_ordering(&adjecency_list, &[75, 97, 47, 61, 53]),
            false
        );
        assert_eq!(is_valid_ordering(&adjecency_list, &[61, 13, 29]), false);
    }

    #[test]
    fn test_correct_ordering() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (47, vec![53, 13, 61, 29].into_iter().collect::<HashSet<_>>()),
            (
                97,
                vec![13, 61, 47, 29, 53, 75]
                    .into_iter()
                    .collect::<HashSet<_>>(),
            ),
            (
                75,
                vec![29, 53, 47, 61, 13].into_iter().collect::<HashSet<_>>(),
            ),
            (61, vec![13].into_iter().collect::<HashSet<_>>()),
            (29, vec![13].into_iter().collect::<HashSet<_>>()),
            (53, vec![29, 13].into_iter().collect::<HashSet<_>>()),
            (61, vec![53, 29].into_iter().collect::<HashSet<_>>()),
        ]);

        assert_eq!(
            correct_ordering(&adjecency_list, &[75, 47, 61, 53, 29]),
            vec![75, 47, 61, 53, 29]
        );
        assert_eq!(
            correct_ordering(&adjecency_list, &[97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
        assert_eq!(
            correct_ordering(&adjecency_list, &[75, 29, 13]),
            vec![75, 29, 13]
        );
        assert_eq!(
            correct_ordering(&adjecency_list, &[75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            correct_ordering(&adjecency_list, &[61, 13, 29]),
            vec![61, 29, 13]
        );
        assert_eq!(
            correct_ordering(&adjecency_list, &[97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_correct_ordering_2() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (73, vec![68].into_iter().collect::<HashSet<_>>()),
            (88, vec![73].into_iter().collect::<HashSet<_>>()),
        ]);

        let corrected = correct_ordering(&adjecency_list, &[68, 73, 88]);
        assert!(
            is_valid_ordering(&adjecency_list, &corrected),
            "{:?}",
            corrected
        );

        let corrected = correct_ordering(&adjecency_list, &[88, 68, 73]);
        assert!(
            is_valid_ordering(&adjecency_list, &corrected),
            "{:?}",
            corrected
        );
    }

    #[test]
    fn test_correct_ordering_simple() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (1, vec![2].into_iter().collect::<HashSet<_>>()),
            (2, vec![3].into_iter().collect::<HashSet<_>>()),
            (3, vec![4].into_iter().collect::<HashSet<_>>()),
            (4, vec![5].into_iter().collect::<HashSet<_>>()),
            (5, vec![6].into_iter().collect::<HashSet<_>>()),
            (6, vec![7].into_iter().collect::<HashSet<_>>()),
            (7, vec![8].into_iter().collect::<HashSet<_>>()),
            (8, vec![9].into_iter().collect::<HashSet<_>>()),
        ]);

        assert!(is_valid_ordering(
            &adjecency_list,
            &correct_ordering(&adjecency_list, &[1, 2, 3, 4, 5, 6, 7, 8, 9])
        ));
    }

    #[test]
    fn test_correct_ordering_3() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (1, vec![2, 3, 9].into_iter().collect::<HashSet<_>>()),
            (2, vec![4, 5, 7, 8, 9].into_iter().collect::<HashSet<_>>()),
            (3, vec![6, 7].into_iter().collect::<HashSet<_>>()),
            (4, vec![7].into_iter().collect::<HashSet<_>>()),
            (5, vec![6].into_iter().collect::<HashSet<_>>()),
            (6, vec![7].into_iter().collect::<HashSet<_>>()),
            (7, vec![9].into_iter().collect::<HashSet<_>>()),
            (8, vec![9].into_iter().collect::<HashSet<_>>()),
        ]);

        assert!(is_valid_ordering(
            &adjecency_list,
            &correct_ordering(&adjecency_list, &[1, 2, 3, 4, 5, 6, 7, 8, 9])
        ));
    }

    #[test]
    fn test_calculate_valid_ordering_checksum() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (75, vec![47, 61, 53, 29].into_iter().collect::<HashSet<_>>()),
            (47, vec![61, 53, 29].into_iter().collect::<HashSet<_>>()),
            (61, vec![53, 29].into_iter().collect::<HashSet<_>>()),
            (53, vec![29].into_iter().collect::<HashSet<_>>()),
            (97, vec![75].into_iter().collect::<HashSet<_>>()),
            (29, vec![13].into_iter().collect::<HashSet<_>>()),
        ]);

        let ordering_checksum = calculate_valid_ordering_checksum(
            &adjecency_list,
            &[
                vec![75, 47, 61, 53, 29],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![75, 47, 5, 53, 29],
                vec![75, 47, 12, 53, 29],
                vec![47, 75, 13],
            ],
        );

        assert_eq!(ordering_checksum, 61 + 5 + 12);
    }

    #[test]
    fn test_calculate_invalid_ordering_checksum() {
        let adjecency_list: HashMap<usize, HashSet<usize>> = HashMap::from_iter(vec![
            (47, vec![53, 13, 61, 29].into_iter().collect::<HashSet<_>>()),
            (
                97,
                vec![13, 61, 47, 29, 53, 75]
                    .into_iter()
                    .collect::<HashSet<_>>(),
            ),
            (
                75,
                vec![29, 53, 47, 61, 13].into_iter().collect::<HashSet<_>>(),
            ),
            (61, vec![13].into_iter().collect::<HashSet<_>>()),
            (29, vec![13].into_iter().collect::<HashSet<_>>()),
            (53, vec![29, 13].into_iter().collect::<HashSet<_>>()),
            (61, vec![53, 29].into_iter().collect::<HashSet<_>>()),
        ]);

        let ordering_checksum = calculate_invalid_ordering_checksum(
            &adjecency_list,
            &[
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        );

        assert_eq!(ordering_checksum, 47 + 29 + 47);
    }
}
