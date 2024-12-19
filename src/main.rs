use std::{collections::HashMap, fs};

fn can_be_combined(output: &str, parts: &[&str]) -> bool {
    if output.len() < 1 {
        return true;
    }

    for part in parts {
        if output.starts_with(part) {
            if can_be_combined(output.trim_start_matches(part), parts) {
                return true;
            }
        }
    }

    return false;
}

fn get_possible_combination_count<'w, 'p>(
    cache: &mut HashMap<&'w str, usize>,
    output: &'w str,
    parts: &[&'p str],
) -> usize {
    let entry = cache.get(output);

    if let Some(result) = entry {
        return result.clone();
    }

    let mut result = 0;

    for part in parts {
        if output.starts_with(part) {
            let remaining = &output[part.len()..output.len()];

            if remaining.len() == 0 {
                result += 1;
                continue;
            }

            let childs = get_possible_combination_count(cache, remaining, parts);

            result += childs;
        }
    }

    cache.insert(output, result.clone());

    return result;
}

fn main() {
    let input = fs::read_to_string("./inputs/day19.txt").expect("Failed to read file");

    let input = input.trim();

    let mut lines = input.lines();

    let parts: Vec<&str> = lines.next().unwrap().split(", ").collect();

    lines.next();

    let mut words = vec![];
    while let Some(word) = lines.next() {
        words.push(word);
    }

    let result = words
        .iter()
        .filter(|word| can_be_combined(word, &parts))
        .collect::<Vec<_>>()
        .len();

    println!("Result (Part 1): {result}");

    let mut cache = HashMap::new();

    let result: usize = words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            println!("Iteration {i}");
            get_possible_combination_count(&mut cache, word, &parts)
        })
        .sum();

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_combined() {
        let parts = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert_eq!(can_be_combined("brwrr", &parts), true);
        assert_eq!(can_be_combined("bggr", &parts), true);
        assert_eq!(can_be_combined("gbbr", &parts), true);
        assert_eq!(can_be_combined("rrbgbr", &parts), true);
        assert_eq!(can_be_combined("ubwu", &parts), false);
        assert_eq!(can_be_combined("bwurrg", &parts), true);
        assert_eq!(can_be_combined("brgr", &parts), true);
        assert_eq!(can_be_combined("bbrgwb", &parts), false);
    }

    #[test]
    fn test_get_possible_combinations() {
        let parts = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        let mut cache = HashMap::new();

        assert_eq!(
            get_possible_combination_count(&mut cache, "brwrr", &parts),
            2
        );

        assert_eq!(
            get_possible_combination_count(&mut cache, "bggr", &parts),
            1
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "gbbr", &parts),
            4
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "rrbgbr", &parts),
            6
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "ubwu", &parts),
            0
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "bwurrg", &parts),
            1
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "brgr", &parts),
            2
        );
        assert_eq!(
            get_possible_combination_count(&mut cache, "bbrgwb", &parts),
            0
        );
    }
}
