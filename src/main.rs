use std::fs;

fn check_if_nums_are_safe(nums: &[isize]) -> bool {
    let mut i = 2;
    while i < nums.len() {
        let current = nums[i];
        let prev = nums[i - 1];
        let prevprev = nums[i - 2];

        let current_diff = current - prev;
        let prev_diff = prev - prevprev;

        if current_diff.abs() > 3 || current_diff == 0 {
            return false;
        }

        if prev_diff.abs() > 3 || prev_diff == 0 {
            return false;
        }

        if current_diff < 0 && prev_diff > 0 || current_diff > 0 && prev_diff < 0 {
            // One is an increase, the other is a decrease
            return false;
        }

        i += 1;
    }

    true
}

fn check_if_nums_are_safe_with_dampener(nums: &[isize]) -> bool {
    if check_if_nums_are_safe(nums) {
        return true;
    }

    let mut safe_permutations: Vec<usize> = Vec::new();

    for i in 0..nums.len() {
        let nums_without_i = nums
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, x)| *x)
            .collect::<Vec<isize>>();

        let is_safe = check_if_nums_are_safe(&nums_without_i);

        if is_safe {
            safe_permutations.push(i);
        }
    }

    if safe_permutations.len() == 0 {
        return false;
    }

    true
}

fn main() {
    let input = fs::read_to_string("./inputs/day2.txt").expect("Failed to read file");

    let parsed: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|x| x.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let mut safe = 0;
    for nums in &parsed {
        let is_safe = check_if_nums_are_safe(&nums);
        if is_safe {
            safe += 1;
        }
    }

    println!("Result (Part 1): {safe}");

    // Part 2

    let mut safe = 0;
    for nums in &parsed {
        let is_safe = check_if_nums_are_safe_with_dampener(&nums);
        if is_safe {
            safe += 1;
        }
    }

    println!("Result (Part 2): {safe}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_nums() {
        assert_eq!(check_if_nums_are_safe(&[0, 0, 0]), false);
        assert_eq!(check_if_nums_are_safe(&[1, 2, 0]), false);
        assert_eq!(check_if_nums_are_safe(&[2, 1, 3]), false);
        assert_eq!(check_if_nums_are_safe(&[-10, -7, 0]), false);
        assert_eq!(check_if_nums_are_safe(&[0, 2, 6]), false);
        assert_eq!(check_if_nums_are_safe(&[10, 8, 4]), false);
        assert_eq!(check_if_nums_are_safe(&[1, 2, 7, 8, 9]), false);
        assert_eq!(check_if_nums_are_safe(&[9, 7, 6, 2, 1]), false);
        assert_eq!(check_if_nums_are_safe(&[1, 3, 2, 4, 5]), false);
        assert_eq!(check_if_nums_are_safe(&[8, 6, 4, 4, 1]), false);
        assert_eq!(check_if_nums_are_safe(&[10, 6, 4, 3, 1]), false);
    }

    #[test]
    fn safe_nums() {
        assert_eq!(check_if_nums_are_safe(&[7, 6, 4, 2, 1]), true);
        assert_eq!(check_if_nums_are_safe(&[94, 96, 97, 98, 99]), true);
        assert_eq!(check_if_nums_are_safe(&[1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn dampener() {
        assert_eq!(check_if_nums_are_safe_with_dampener(&[0, 0, 0, 0]), false);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[1, 2, 3, 4]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[10, 6, 5, 4]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[6, 6, 5, 4]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[10, 9, 12, 8]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[10, 9, -12, 8]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[10, 9, 8, 7]), true);
        assert_eq!(check_if_nums_are_safe_with_dampener(&[10, 9, 8, -7]), true);
    }
}
