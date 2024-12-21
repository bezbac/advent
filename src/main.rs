use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./inputs/day21.txt").expect("Failed to read file");

    let result: usize = 0;

    println!("Result (Part 1): {result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DirectionalCommand {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl DirectionalCommand {
    fn reverse(self) -> Self {
        match self {
            DirectionalCommand::Up => DirectionalCommand::Down,
            DirectionalCommand::Down => DirectionalCommand::Up,
            DirectionalCommand::Left => DirectionalCommand::Right,
            DirectionalCommand::Right => DirectionalCommand::Left,
            DirectionalCommand::Activate => DirectionalCommand::Activate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

impl TryFrom<char> for Key {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,
            'A' => Key::Activate,
            _ => return Err(()),
        })
    }
}

impl TryFrom<Key> for usize {
    type Error = ();

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        Ok(match value {
            Key::Zero => 0,
            Key::One => 1,
            Key::Two => 2,
            Key::Three => 3,
            Key::Four => 4,
            Key::Five => 5,
            Key::Six => 6,
            Key::Seven => 7,
            Key::Eight => 8,
            Key::Nine => 9,
            Key::Activate => return Err(()),
        })
    }
}

fn get_directional_commands_between_directional_commands(
    a: &DirectionalCommand,
    b: &DirectionalCommand,
) -> Vec<DirectionalCommand> {
    let mut result: Vec<DirectionalCommand> = match a {
        DirectionalCommand::Activate => match b {
            DirectionalCommand::Activate => vec![],
            DirectionalCommand::Right => vec![DirectionalCommand::Down],
            DirectionalCommand::Up => vec![DirectionalCommand::Left],
            DirectionalCommand::Down => {
                vec![DirectionalCommand::Down, DirectionalCommand::Left]
            }
            DirectionalCommand::Left => {
                vec![
                    DirectionalCommand::Down,
                    DirectionalCommand::Left,
                    DirectionalCommand::Left,
                ]
            }
        },
        DirectionalCommand::Right => match b {
            DirectionalCommand::Activate => {
                get_directional_commands_between_directional_commands(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            DirectionalCommand::Right => vec![],
            DirectionalCommand::Down => vec![DirectionalCommand::Left],
            DirectionalCommand::Up => {
                vec![DirectionalCommand::Left, DirectionalCommand::Up]
            }
            DirectionalCommand::Left => {
                vec![DirectionalCommand::Left, DirectionalCommand::Left]
            }
        },
        DirectionalCommand::Down => match b {
            DirectionalCommand::Activate | DirectionalCommand::Right => {
                get_directional_commands_between_directional_commands(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            DirectionalCommand::Down => vec![],
            DirectionalCommand::Up => vec![DirectionalCommand::Up],
            DirectionalCommand::Left => vec![DirectionalCommand::Left],
        },
        DirectionalCommand::Up => match b {
            DirectionalCommand::Activate | DirectionalCommand::Right | DirectionalCommand::Down => {
                get_directional_commands_between_directional_commands(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            DirectionalCommand::Up => vec![],
            DirectionalCommand::Left => {
                vec![DirectionalCommand::Down, DirectionalCommand::Left]
            }
        },
        DirectionalCommand::Left => match b {
            DirectionalCommand::Activate
            | DirectionalCommand::Right
            | DirectionalCommand::Down
            | DirectionalCommand::Up => get_directional_commands_between_directional_commands(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            DirectionalCommand::Left => vec![],
        },
    };

    result.sort();

    return result;
}

fn get_directional_commands_between_keys(a: &Key, b: &Key) -> Vec<DirectionalCommand> {
    let mut result = match a {
        &Key::Zero => match b {
            &Key::Zero => return vec![],
            &Key::One => return vec![DirectionalCommand::Up, DirectionalCommand::Left],
            &Key::Two => return vec![DirectionalCommand::Up],
            &Key::Three => return vec![DirectionalCommand::Up, DirectionalCommand::Right],
            &Key::Activate => return vec![DirectionalCommand::Right],
            _ => {
                // Move to two
                let mut result = vec![DirectionalCommand::Up];
                result.append(&mut get_directional_commands_between_keys(&Key::Two, b));
                result
            }
        },
        &Key::One => match b {
            &Key::Zero => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::One => return vec![],
            &Key::Two => return vec![DirectionalCommand::Right],
            &Key::Three => return vec![DirectionalCommand::Right, DirectionalCommand::Right],
            &Key::Four => return vec![DirectionalCommand::Up],
            &Key::Seven => return vec![DirectionalCommand::Up, DirectionalCommand::Up],
            _ => {
                // Move to two
                let mut result = vec![DirectionalCommand::Right];
                result.append(&mut get_directional_commands_between_keys(&Key::Two, b));
                result
            }
        },
        &Key::Two => match b {
            &Key::Zero | &Key::One => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::Two => return vec![],
            &Key::Three => return vec![DirectionalCommand::Right],
            &Key::Four => return vec![DirectionalCommand::Up, DirectionalCommand::Left],
            &Key::Five => return vec![DirectionalCommand::Up],
            &Key::Six => return vec![DirectionalCommand::Up, DirectionalCommand::Right],
            &Key::Activate => {
                // Move to zero
                let mut result = vec![DirectionalCommand::Down];
                result.append(&mut get_directional_commands_between_keys(&Key::Zero, b));
                result
            }
            _ => {
                // Move to five
                let mut result = vec![DirectionalCommand::Up];
                result.append(&mut get_directional_commands_between_keys(&Key::Five, b));
                result
            }
        },
        &Key::Three => match b {
            &Key::Zero | &Key::One | &Key::Two => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::Six => return vec![DirectionalCommand::Up],
            &Key::Nine => return vec![DirectionalCommand::Up, DirectionalCommand::Up],
            &Key::Activate => return vec![DirectionalCommand::Down],
            _ => {
                // Move to two
                let mut result = vec![DirectionalCommand::Left];
                result.append(&mut get_directional_commands_between_keys(&Key::Two, b));
                result
            }
        },
        &Key::Four => match b {
            &Key::Zero | &Key::One | &Key::Two | &Key::Three => {
                get_directional_commands_between_keys(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            &Key::Four => return vec![],
            &Key::Five => return vec![DirectionalCommand::Right],
            &Key::Seven => return vec![DirectionalCommand::Up],
            _ => {
                // Move to five
                let mut result = vec![DirectionalCommand::Right];
                result.append(&mut get_directional_commands_between_keys(&Key::Five, b));
                result
            }
        },
        &Key::Five => match b {
            &Key::Zero | &Key::One | &Key::Two | &Key::Three | &Key::Four => {
                get_directional_commands_between_keys(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            &Key::Five => return vec![],
            &Key::Six => return vec![DirectionalCommand::Right],
            &Key::Activate => {
                // Move to two
                let mut result = vec![DirectionalCommand::Down];
                result.append(&mut get_directional_commands_between_keys(&Key::Two, b));
                result
            }
            _ => {
                // Move to eight
                let mut result = vec![DirectionalCommand::Up];
                result.append(&mut get_directional_commands_between_keys(&Key::Eight, b));
                result
            }
        },
        &Key::Six => match b {
            &Key::Zero | &Key::One | &Key::Two | &Key::Three | &Key::Four | &Key::Five => {
                get_directional_commands_between_keys(b, a)
                    .into_iter()
                    .map(DirectionalCommand::reverse)
                    .collect()
            }
            &Key::Six => return vec![],
            &Key::Nine => return vec![DirectionalCommand::Up],
            &Key::Activate => return vec![DirectionalCommand::Down, DirectionalCommand::Down],
            _ => {
                // Move to five
                let mut result = vec![DirectionalCommand::Left];
                result.append(&mut get_directional_commands_between_keys(&Key::Five, b));
                result
            }
        },
        &Key::Seven => match b {
            &Key::Zero
            | &Key::One
            | &Key::Two
            | &Key::Three
            | &Key::Four
            | &Key::Five
            | &Key::Six => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::Seven => return vec![],
            &Key::Eight => return vec![DirectionalCommand::Right],
            &Key::Nine => return vec![DirectionalCommand::Right, DirectionalCommand::Right],
            &Key::Activate => {
                // Move to eight
                let mut result = vec![DirectionalCommand::Right];
                result.append(&mut get_directional_commands_between_keys(&Key::Eight, b));
                result
            }
        },
        &Key::Eight => match b {
            &Key::Zero
            | &Key::One
            | &Key::Two
            | &Key::Three
            | &Key::Four
            | &Key::Five
            | &Key::Six
            | &Key::Seven => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::Eight => return vec![],
            &Key::Nine => return vec![DirectionalCommand::Right],
            &Key::Activate => {
                // Move to five
                let mut result = vec![DirectionalCommand::Down];
                result.append(&mut get_directional_commands_between_keys(&Key::Five, b));
                result
            }
        },
        &Key::Nine => match b {
            &Key::Zero
            | &Key::One
            | &Key::Two
            | &Key::Three
            | &Key::Four
            | &Key::Five
            | &Key::Six
            | &Key::Seven
            | &Key::Eight => get_directional_commands_between_keys(b, a)
                .into_iter()
                .map(DirectionalCommand::reverse)
                .collect(),
            &Key::Nine => return vec![],
            &Key::Activate => {
                return vec![
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                ]
            }
        },
        &Key::Activate => get_directional_commands_between_keys(b, a)
            .into_iter()
            .map(DirectionalCommand::reverse)
            .collect(),
    };

    result.sort();

    return result;
}

fn get_directional_commands_for_keycode(code: &[Key], start: &Key) -> Vec<DirectionalCommand> {
    let mut result = vec![];

    result.append(&mut get_directional_commands_between_keys(start, &code[0]));
    result.push(DirectionalCommand::Activate);

    for i in 0..code.len() - 1 {
        let a = &code[i];
        let b = &code[i + 1];

        result.append(&mut get_directional_commands_between_keys(a, b));
        result.push(DirectionalCommand::Activate);
    }

    result
}

fn encode_directional_commands(
    commands: &[DirectionalCommand],
    start: &DirectionalCommand,
) -> Vec<DirectionalCommand> {
    let mut result = vec![];

    result.append(&mut get_directional_commands_between_directional_commands(
        start,
        &commands[0],
    ));
    result.push(DirectionalCommand::Activate);

    for i in 0..commands.len() - 1 {
        let a = &commands[i];
        let b = &commands[i + 1];

        result.append(&mut get_directional_commands_between_directional_commands(
            a, b,
        ));
        result.push(DirectionalCommand::Activate);
    }

    result
}

fn get_permutations<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    // Base case: if array has only one element, return it wrapped in a vec
    if arr.len() <= 1 {
        return vec![arr.to_vec()];
    }

    let mut result = Vec::new();

    // Try each element as the first element
    for i in 0..arr.len() {
        // Create a vector without the current element
        let mut remaining = arr.to_vec();
        let current = remaining.remove(i);

        // Recursively get permutations of the remaining elements
        for mut perm in get_permutations(&remaining) {
            // Add the current element to the front
            perm.insert(0, current.clone());
            result.push(perm);
        }
    }

    result
}

fn get_directional_commands_permutations(
    commands: &[DirectionalCommand],
) -> Vec<Vec<DirectionalCommand>> {
    let mut parts = vec![];
    let mut current_part = vec![];
    for c in commands {
        if c == &DirectionalCommand::Activate {
            parts.push(current_part.clone());
            current_part = vec![];
            continue;
        }

        current_part.push(*c);
    }

    let mut inputs: Vec<Vec<DirectionalCommand>> = get_permutations(&parts[0]);
    for part in &parts[1..] {
        inputs = inputs
            .into_iter()
            .cartesian_product(get_permutations(&part))
            .map(|(a, b)| {
                let mut a: Vec<DirectionalCommand> = a;
                let mut b = b;
                a.push(DirectionalCommand::Activate);
                a.append(&mut b);
                a
            })
            .collect()
    }

    inputs = inputs
        .into_iter()
        .map(|mut input| {
            input.push(DirectionalCommand::Activate);
            input
        })
        .collect();

    inputs = inputs.into_iter().unique().collect();

    inputs
}

fn shortest_encode_directional_commands(
    commands: &[DirectionalCommand],
) -> Vec<DirectionalCommand> {
    let inputs = get_directional_commands_permutations(&commands);

    let mut encoded: Vec<_> = inputs
        .into_iter()
        .unique()
        .map(|input| encode_directional_commands(&input, &DirectionalCommand::Activate))
        .collect();

    encoded.sort_by(|a, b| a.len().cmp(&b.len()));

    encoded[0].clone()
}

fn encode_code(code: &[Key], additional_passes: usize) -> Vec<DirectionalCommand> {
    let mut result = get_directional_commands_for_keycode(code, &Key::Activate);

    let mut i = 0;

    // FIXME: This needs to be done in one go. Choosing the shortest encoding in pass 1 might lead to not being able to find the shortest encoding in pass 2
    while i < additional_passes {
        result = shortest_encode_directional_commands(&result);
        i += 1;
    }

    result
}

fn calculate_checksum(code: &[Key]) -> usize {
    let a = encode_code(code, 2).len();

    let b: usize = code
        .iter()
        .filter_map(|key| {
            if let Ok(value) = usize::try_from(*key) {
                if value == 0 {
                    return None;
                }

                return Some(value);
            } else {
                return None;
            }
        })
        .map(|v| v.to_string())
        .join("")
        .parse()
        .unwrap();

    a * b
}

fn parse_keys(input: &str) -> Vec<Key> {
    let mut result = vec![];

    for c in input.trim().chars() {
        result.push(Key::try_from(c).unwrap())
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_directional_commands_between_keys() {
        fn dir_command_check(a: Key, b: Key, expected: Vec<DirectionalCommand>) {
            let mut res = get_directional_commands_between_keys(&a, &b);
            let mut expected = expected;
            res.sort();
            expected.sort();
            assert_eq!(res, expected);
        }

        dir_command_check(Key::Zero, Key::Two, vec![DirectionalCommand::Up]);
        dir_command_check(
            Key::Two,
            Key::Nine,
            vec![
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Right,
            ],
        );
        dir_command_check(
            Key::Nine,
            Key::Activate,
            vec![
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
            ],
        );
        dir_command_check(
            Key::Activate,
            Key::Six,
            vec![DirectionalCommand::Up, DirectionalCommand::Up],
        );
        dir_command_check(
            Key::Activate,
            Key::Four,
            vec![
                DirectionalCommand::Left,
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Left,
            ],
        );
        dir_command_check(
            Key::Activate,
            Key::Eight,
            vec![
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Left,
            ],
        );
        dir_command_check(
            Key::Seven,
            Key::Nine,
            vec![DirectionalCommand::Right, DirectionalCommand::Right],
        );
        dir_command_check(
            Key::Four,
            Key::Six,
            vec![DirectionalCommand::Right, DirectionalCommand::Right],
        );
        dir_command_check(
            Key::One,
            Key::Three,
            vec![DirectionalCommand::Right, DirectionalCommand::Right],
        );
        dir_command_check(
            Key::One,
            Key::Seven,
            vec![DirectionalCommand::Up, DirectionalCommand::Up],
        );
        dir_command_check(
            Key::Seven,
            Key::Nine,
            vec![DirectionalCommand::Right, DirectionalCommand::Right],
        );
        dir_command_check(
            Key::Nine,
            Key::Activate,
            vec![
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
            ],
        );
    }

    #[test]
    fn test_get_directional_commands_permutations() {
        assert_eq!(
            get_directional_commands_permutations(&[
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Activate
            ]),
            vec![vec![
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Activate
            ]]
        );

        assert_eq!(
            get_directional_commands_permutations(&[
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Right,
                DirectionalCommand::Activate
            ]),
            vec![
                vec![
                    DirectionalCommand::Activate,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Down,
                    DirectionalCommand::Right,
                    DirectionalCommand::Activate
                ],
                vec![
                    DirectionalCommand::Activate,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Right,
                    DirectionalCommand::Down,
                    DirectionalCommand::Activate
                ]
            ]
        );
    }

    #[test]
    fn test_get_directional_commands_for_keycode() {
        assert_eq!(
            get_directional_commands_for_keycode(
                &[Key::Zero, Key::Two, Key::Nine, Key::Activate],
                &Key::Activate
            ),
            vec![
                DirectionalCommand::Left,
                DirectionalCommand::Activate,
                DirectionalCommand::Up,
                DirectionalCommand::Activate,
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Right,
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Activate
            ]
        );
    }

    #[test]
    fn test_get_directional_commands_between_directional_commands() {
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Activate,
                &DirectionalCommand::Up
            ),
            vec![DirectionalCommand::Left]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Activate,
                &DirectionalCommand::Right
            ),
            vec![DirectionalCommand::Down]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Left,
                &DirectionalCommand::Right
            ),
            vec![DirectionalCommand::Right, DirectionalCommand::Right]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Right,
                &DirectionalCommand::Left
            ),
            vec![DirectionalCommand::Left, DirectionalCommand::Left]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Up,
                &DirectionalCommand::Right,
            ),
            vec![DirectionalCommand::Down, DirectionalCommand::Right]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Left,
                &DirectionalCommand::Up
            ),
            vec![DirectionalCommand::Up, DirectionalCommand::Right]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Up,
                &DirectionalCommand::Left
            ),
            vec![DirectionalCommand::Down, DirectionalCommand::Left]
        );
        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Activate,
                &DirectionalCommand::Left
            ),
            vec![
                DirectionalCommand::Down,
                DirectionalCommand::Left,
                DirectionalCommand::Left
            ]
        );

        assert_eq!(
            get_directional_commands_between_directional_commands(
                &DirectionalCommand::Activate,
                &DirectionalCommand::Up
            ),
            vec![DirectionalCommand::Left]
        );
    }

    #[test]
    fn test_encode_code() {
        assert_eq!(encode_code(&parse_keys("029A"), 0).len(), 12);
        assert_eq!(encode_code(&parse_keys("029A"), 1).len(), 28);
        assert_eq!(encode_code(&parse_keys("029A"), 2).len(), 68);

        assert_eq!(encode_code(&parse_keys("980A"), 0).len(), 12);
        assert_eq!(encode_code(&parse_keys("980A"), 1).len(), 26);
        assert_eq!(encode_code(&parse_keys("980A"), 2).len(), 60);

        assert_eq!(encode_code(&parse_keys("179A"), 0).len(), 14);
        assert_eq!(encode_code(&parse_keys("179A"), 2).len(), 68);

        assert_eq!(encode_code(&parse_keys("456A"), 0).len(), 12);
        assert_eq!(encode_code(&parse_keys("456A"), 2).len(), 64);

        assert_eq!(encode_code(&parse_keys("379A"), 0).len(), 14);
        assert_eq!(encode_code(&parse_keys("379A"), 2).len(), 64);
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!(calculate_checksum(&parse_keys("029A")), 68 * 29);
        assert_eq!(calculate_checksum(&parse_keys("980A")), 60 * 980);
        assert_eq!(calculate_checksum(&parse_keys("179A")), 68 * 179);
        assert_eq!(calculate_checksum(&parse_keys("456A")), 64 * 456);
        assert_eq!(calculate_checksum(&parse_keys("379A")), 64 * 379);
    }
}
