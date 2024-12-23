use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::BitXor,
};

use itertools::Itertools;

fn mix(sn: isize, modifier: isize) -> isize {
    modifier.bitxor(sn)
}

fn prune(sn: isize) -> isize {
    return sn % 16777216;
}

fn get_next_secret_number(sn: isize) -> isize {
    let mut result = prune(mix(sn, sn * 64));
    result = prune(mix(result, result / 32));
    result = prune(mix(result, result * 2048));
    result
}

fn generate_nth_secret_number(input: isize, n: usize) -> isize {
    let mut current = input;
    for _ in 0..n {
        current = get_next_secret_number(current);
    }
    return current;
}

fn generate_n_secret_numbers(input: isize, n: usize) -> Vec<isize> {
    let mut result = vec![input];
    let mut current = input;
    for _ in 0..n {
        current = get_next_secret_number(current);
        result.push(current);
    }
    result
}

fn get_price_from_secret_number(input: isize) -> isize {
    return input % 10;
}

fn get_changes(input: &[isize]) -> Vec<isize> {
    let mut result = vec![];

    for (i, v) in input[1..].iter().enumerate() {
        result.push(v - input[i])
    }

    result
}

fn generate_n_prices(input: isize, n: usize) -> Vec<isize> {
    let secret_numbers = generate_n_secret_numbers(input, n);
    secret_numbers
        .into_iter()
        .map(|sn| get_price_from_secret_number(sn))
        .collect()
}

fn get_sequences_with_prices(prices: &[isize], changes: &[isize]) -> Vec<(Vec<isize>, isize)> {
    let mut result = vec![];

    for (i, sequence) in changes.windows(4).enumerate() {
        let price = prices[i + 4];

        result.push((sequence.to_vec(), price))
    }

    result
}

// If a sequence occurs multiple times with different prices, this function will filter out
// all occurences that are not the best.
// This also means, that the sequences outputted by this function will be unique
fn filter_first_sequences(sequences: &[(Vec<isize>, isize)]) -> HashSet<(Vec<isize>, isize)> {
    let mut dedup = HashSet::new();
    sequences
        .into_iter()
        .filter(|(seq, _)| {
            if dedup.contains(seq) {
                return false;
            }

            dedup.insert(seq);

            return true;
        })
        .cloned()
        .collect()
}

fn pick_best_sequence(input: &[Vec<(Vec<isize>, isize)>]) -> (Vec<isize>, isize) {
    let mut grouped: HashMap<&Vec<isize>, isize> = HashMap::new();

    for sequences in input {
        for (sequence, price) in sequences {
            *grouped.entry(sequence).or_default() += price
        }
    }

    let sorted: Vec<_> = grouped
        .into_iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .collect();

    let first = sorted.first().unwrap();

    (first.0.clone(), first.1)
}

fn find_best_sequence_from_sn(secret_numbers: &[isize]) -> (Vec<isize>, isize) {
    let prices: Vec<Vec<isize>> = secret_numbers
        .into_iter()
        .map(|x| generate_n_prices(*x, 2000))
        .collect();

    let changes: Vec<_> = prices.iter().map(|prices| get_changes(prices)).collect();

    let sequences_for_prices: Vec<_> = prices
        .iter()
        .zip(changes)
        .map(|(prices, changes)| {
            filter_first_sequences(&get_sequences_with_prices(&prices, &changes))
                .into_iter()
                .collect::<Vec<_>>()
        })
        .collect();

    return pick_best_sequence(&sequences_for_prices);
}

fn main() {
    let input = fs::read_to_string("./inputs/day22.txt").expect("Failed to read file");

    let starting_numbers: Vec<isize> = input
        .trim()
        .lines()
        .map(|line| -> isize {
            let line = line.trim();
            line.parse().unwrap()
        })
        .collect();

    let result: isize = starting_numbers
        .iter()
        .map(|sn| generate_nth_secret_number(*sn, 2000))
        .sum();

    println!("Result (Part 1): {result}");

    let result = find_best_sequence_from_sn(&starting_numbers);

    println!("Result (Part 2): {}", result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_get_price_from_secret_number() {
        assert_eq!(get_price_from_secret_number(123), 3);
        assert_eq!(get_price_from_secret_number(15887950), 0);
        assert_eq!(get_price_from_secret_number(16495136), 6);
    }

    #[test]
    fn test_example_123() {
        let mut series = vec![123];
        for _ in 0..9 {
            series.push(get_next_secret_number(*series.last().unwrap()));
        }

        let prices: Vec<_> = series
            .into_iter()
            .map(|n| get_price_from_secret_number(n))
            .collect();

        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);

        let changes = get_changes(&prices);

        assert_eq!(changes, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);

        let sequence = get_sequences_with_prices(&prices, &changes);

        assert!(sequence.contains(&(vec![-1, -1, 0, 2], 6)));

        let best_sequence = filter_first_sequences(&sequence);

        assert!(best_sequence.contains(&(vec![-1, -1, 0, 2], 6)));
    }

    #[test]
    fn test_generate_nth_secret_number() {
        assert_eq!(generate_nth_secret_number(1, 2000), 8685429);
        assert_eq!(generate_nth_secret_number(10, 2000), 4700978);
        assert_eq!(generate_nth_secret_number(100, 2000), 15273692);
        assert_eq!(generate_nth_secret_number(2024, 2000), 8667524);
    }

    #[test]
    fn test_find_best_sequence() {
        assert_eq!(
            find_best_sequence_from_sn(&[1, 2, 3, 2024]),
            (vec![-2, 1, -1, 3], 23)
        );
    }

    #[test]
    fn test_filter_best_sequences() {
        assert_eq!(
            filter_first_sequences(&[
                (vec![-2, 1, -1, 3], 3),
                (vec![-2, 1, -1, 3], 12),
                (vec![-2, 1, -1, 3], 2),
                (vec![0, 0, 1, 2], 2),
            ]),
            [(vec![-2, 1, -1, 3], 3), (vec![0, 0, 1, 2], 2)]
                .into_iter()
                .collect()
        );

        assert_eq!(
            filter_first_sequences(&[
                (vec![4, -2, 2, -1,], 5),
                (vec![-2, 2, -1, -1,], 4),
                (vec![2, -1, -1, -4,], 0),
                (vec![-1, -1, -4, 6,], 6),
                (vec![-2, 2, -1, -1,], 7),
            ]),
            [
                (vec![4, -2, 2, -1], 5),
                (vec![2, -1, -1, -4], 0),
                (vec![-1, -1, -4, 6], 6),
                (vec![-2, 2, -1, -1], 4)
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_pick_best_sequence() {
        assert_eq!(
            pick_best_sequence(&[
                vec![(vec![-2, 1, -1, 3], 7)],
                vec![(vec![-2, 1, -1, 3], 7)],
                vec![(vec![2, 1, 1, 3], 2)],
                vec![(vec![-2, 1, -1, 3], 9)],
            ]),
            (vec![-2, 1, -1, 3], 23)
        );
    }

    #[test]
    fn test_get_sequences_with_prices() {
        let prices = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];
        let changes = get_changes(&prices);

        assert_eq!(
            get_sequences_with_prices(&prices, &changes),
            vec![
                (vec![-3, 6, -1, -1], 4),
                (vec![6, -1, -1, 0], 4),
                (vec![-1, -1, 0, 2], 6),
                (vec![-1, 0, 2, -2], 4),
                (vec![0, 2, -2, 0], 4),
                (vec![2, -2, 0, -2], 2)
            ]
        );
    }

    #[test]
    fn test_calculate_next_number() {
        let mut secret_number = 123;

        let mut next_10 = vec![];
        for _ in 0..10 {
            secret_number = get_next_secret_number(secret_number);
            next_10.push(secret_number);
        }

        assert_eq!(
            next_10,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }
}
