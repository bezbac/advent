use std::{fs, ops::BitXor};

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

fn main() {
    let input = fs::read_to_string("./inputs/day22.txt").expect("Failed to read file");

    let result: isize = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let sn = line.parse().unwrap();
            generate_nth_secret_number(sn, 2000)
        })
        .sum();

    println!("Result (Part 1): {result}");
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
    fn test_generate_nth_secret_number() {
        assert_eq!(generate_nth_secret_number(1, 2000), 8685429);
        assert_eq!(generate_nth_secret_number(10, 2000), 4700978);
        assert_eq!(generate_nth_secret_number(100, 2000), 15273692);
        assert_eq!(generate_nth_secret_number(2024, 2000), 8667524);
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
