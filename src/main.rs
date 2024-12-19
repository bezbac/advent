use std::fs;

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
}
