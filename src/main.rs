use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

use itertools::Itertools;
use memoize::memoize;
use pathfinding::prelude::astar_bag_collect;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DirectionalCommand {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl From<DirectionalCommand> for char {
    fn from(value: DirectionalCommand) -> Self {
        match value {
            DirectionalCommand::Right => '>',
            DirectionalCommand::Left => '<',
            DirectionalCommand::Up => '^',
            DirectionalCommand::Down => 'v',
            DirectionalCommand::Activate => 'A',
        }
    }
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

impl From<Key> for char {
    fn from(value: Key) -> char {
        match value {
            Key::Zero => '0',
            Key::One => '1',
            Key::Two => '2',
            Key::Three => '3',
            Key::Four => '4',
            Key::Five => '5',
            Key::Six => '6',
            Key::Seven => '7',
            Key::Eight => '8',
            Key::Nine => '9',
            Key::Activate => 'A',
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn key_to_pos(key: Key) -> (usize, usize) {
    match key {
        Key::Seven => (0, 0),
        Key::Four => (0, 1),
        Key::One => (0, 2),
        Key::Eight => (1, 0),
        Key::Five => (1, 1),
        Key::Two => (1, 2),
        Key::Zero => (1, 3),
        Key::Nine => (2, 0),
        Key::Six => (2, 1),
        Key::Three => (2, 2),
        Key::Activate => (2, 3),
    }
}

fn get_directional_command_from_delta(d: (isize, isize)) -> Option<DirectionalCommand> {
    match d {
        (0, 0) => None,
        (-1, 0) => Some(DirectionalCommand::Left),
        (1, 0) => Some(DirectionalCommand::Right),
        (0, -1) => Some(DirectionalCommand::Up),
        (0, 1) => Some(DirectionalCommand::Down),
        _ => panic!("Cannot get directional command for {:?}", d),
    }
}

#[memoize]
fn compute_shortest_paths_between_keys() -> HashMap<(Key, Key), HashSet<Vec<DirectionalCommand>>> {
    let all_keys = [
        Key::Zero,
        Key::One,
        Key::Two,
        Key::Three,
        Key::Four,
        Key::Five,
        Key::Six,
        Key::Seven,
        Key::Eight,
        Key::Nine,
        Key::Activate,
    ];

    let mut result = HashMap::new();

    for a in all_keys {
        for b in all_keys {
            if a == b {
                result.insert((a, b), HashSet::new());
                continue;
            }

            let start_pos = key_to_pos(a);
            let end_pos = key_to_pos(b);

            let Some((shortest_paths, _)) = astar_bag_collect(
                &start_pos,
                |&(x, y)| {
                    [
                        (x as isize, y as isize - 1),
                        (x as isize, y as isize + 1),
                        (x as isize - 1, y as isize),
                        (x as isize + 1, y as isize),
                    ]
                    .into_iter()
                    .filter_map(|(x, y)| -> Option<(usize, usize)> {
                        let is_within_bounds =
                            x >= 0 && y >= 0 && (x as usize) < 3 && (y as usize) < 4;

                        if !is_within_bounds {
                            return None;
                        }

                        let x = x as usize;
                        let y = y as usize;

                        if (x, y) == (0, 3) {
                            // This position is forbidden
                            return None;
                        }

                        Some((x, y))
                    })
                    .map(|(x, y)| ((x, y), 1))
                    .collect::<Vec<_>>()
                },
                |&(x, y)| {
                    let (ex, ey) = end_pos;
                    (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                        .sqrt() as usize
                },
                |position| position == &end_pos,
            ) else {
                panic!("Could not compute shortest paths");
            };

            let shortest_paths = shortest_paths
                .into_iter()
                .map(|positions| {
                    positions
                        .iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(i, pos)| {
                            let last = positions[i];
                            let delta = (
                                pos.0 as isize - last.0 as isize,
                                pos.1 as isize - last.1 as isize,
                            );
                            get_directional_command_from_delta(delta)
                        })
                        .collect()
                })
                .filter(|path: &Vec<_>| {
                    // Filter out zigzag moves
                    if path.len() < 2 {
                        return true;
                    }
                    let windows: Vec<_> = path.chunks(2).collect();
                    if windows.len() != windows.into_iter().dedup().count() {
                        return false;
                    }
                    let first = path.first().unwrap();
                    let last = path.last().unwrap();
                    if first != last {
                        return true;
                    }
                    return path.into_iter().all(|cmd| cmd == first);
                })
                .collect();

            result.insert((a, b), shortest_paths);
        }
    }

    result
}

fn directional_command_to_position(cmd: DirectionalCommand) -> (usize, usize) {
    match cmd {
        DirectionalCommand::Up => (1, 0),
        DirectionalCommand::Down => (1, 1),
        DirectionalCommand::Left => (0, 1),
        DirectionalCommand::Right => (2, 1),
        DirectionalCommand::Activate => (2, 0),
    }
}

fn parse_keys(input: &str) -> Vec<Key> {
    let mut result = vec![];

    for c in input.trim().chars() {
        result.push(Key::try_from(c).unwrap())
    }

    result
}

#[memoize]
fn compute_shortest_paths_between_commands(
) -> HashMap<(DirectionalCommand, DirectionalCommand), HashSet<Vec<DirectionalCommand>>> {
    let all_commands = [
        DirectionalCommand::Up,
        DirectionalCommand::Down,
        DirectionalCommand::Left,
        DirectionalCommand::Right,
        DirectionalCommand::Activate,
    ];

    let mut result = HashMap::new();

    for a in all_commands {
        for b in all_commands {
            if a == b {
                result.insert((a, b), HashSet::new());
                continue;
            }

            let start_pos = directional_command_to_position(a);
            let end_pos = directional_command_to_position(b);

            let Some((shortest_paths, _)) = astar_bag_collect(
                &start_pos,
                |&(x, y)| {
                    [
                        (x as isize, y as isize - 1),
                        (x as isize, y as isize + 1),
                        (x as isize - 1, y as isize),
                        (x as isize + 1, y as isize),
                    ]
                    .into_iter()
                    .filter_map(|(x, y)| -> Option<(usize, usize)> {
                        let is_within_bounds =
                            x >= 0 && y >= 0 && (x as usize) < 3 && (y as usize) < 4;

                        if !is_within_bounds {
                            return None;
                        }

                        let x = x as usize;
                        let y = y as usize;

                        if (x, y) == (0, 0) {
                            // This position is forbidden
                            return None;
                        }

                        Some((x, y))
                    })
                    .map(|(x, y)| ((x, y), 1))
                    .collect::<Vec<_>>()
                },
                |&(x, y)| {
                    let (ex, ey) = end_pos;
                    (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                        .sqrt() as usize
                },
                |position| position == &end_pos,
            ) else {
                panic!("Could not compute shortest paths");
            };

            let shortest_paths = shortest_paths
                .into_iter()
                .map(|positions| {
                    positions
                        .iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(i, pos)| {
                            let last = positions[i];
                            let delta = (
                                pos.0 as isize - last.0 as isize,
                                pos.1 as isize - last.1 as isize,
                            );
                            get_directional_command_from_delta(delta)
                        })
                        .collect()
                })
                .collect();

            result.insert((a, b), shortest_paths);
        }
    }

    result
}

fn encode(code: &[Key]) -> HashSet<Vec<DirectionalCommand>> {
    let shortest_paths_between_keys = compute_shortest_paths_between_keys();

    let mut encoded = HashSet::new();

    for (i, x) in code.iter().enumerate() {
        if i == 0 {
            encoded = shortest_paths_between_keys
                .get(&(Key::Activate, *x))
                .unwrap()
                .clone();
            continue;
        }

        let previous = code[i - 1];

        encoded = encoded
            .iter()
            .cartesian_product(
                &shortest_paths_between_keys
                    .get(&(previous, *x))
                    .unwrap()
                    .clone(),
            )
            .map(|(a, b)| {
                let mut a = a.clone();
                let mut b = b.clone();
                let mut res = vec![];
                res.append(&mut a);
                res.push(DirectionalCommand::Activate);
                res.append(&mut b);
                res
            })
            .collect();
    }

    encoded = encoded
        .into_iter()
        .map(|mut moves| {
            moves.push(DirectionalCommand::Activate);
            moves
        })
        .collect();

    encoded
}

fn reencode(commands: &[DirectionalCommand]) -> HashSet<Vec<DirectionalCommand>> {
    let shortest_paths_between_commands = compute_shortest_paths_between_commands();

    let mut encoded = HashSet::new();

    for (i, x) in commands.iter().enumerate() {
        if i == 0 {
            encoded = shortest_paths_between_commands
                .get(&(DirectionalCommand::Activate, *x))
                .unwrap()
                .clone();

            continue;
        }

        let previous = commands[i - 1];

        let paths = shortest_paths_between_commands
            .get(&(previous, *x))
            .unwrap()
            .clone();

        if paths.len() == 0 {
            encoded = encoded
                .into_iter()
                .map(|mut encoded| {
                    encoded.push(DirectionalCommand::Activate);
                    encoded
                })
                .collect();
            continue;
        }

        encoded = encoded
            .iter()
            .cartesian_product(&paths)
            .map(|(a, b)| {
                let mut a = a.clone();
                let mut b = b.clone();
                let mut res = vec![];
                res.append(&mut a);
                res.push(DirectionalCommand::Activate);
                res.append(&mut b);
                res
            })
            .collect();
    }

    encoded = encoded
        .into_iter()
        .map(|mut moves| {
            moves.push(DirectionalCommand::Activate);
            moves
        })
        .collect();

    encoded
}

#[derive(Clone, Copy, PartialEq)]
enum Level {
    Zero,
    One,
    Two,
}

fn find_shortest_encoding(code: &[Key], level: Level) -> Vec<DirectionalCommand> {
    println!(
        "Finding shortest encoding for {}",
        code.iter().map(|k| char::from(*k)).join(",")
    );

    let mut encoded = encode(code);

    println!("Found {} possible 0 level encodings", encoded.len());

    if level == Level::One || level == Level::Two {
        encoded = encoded
            .into_par_iter()
            .flat_map(|commands| reencode(&commands))
            .collect();

        println!("Found {} possible 1 level encodings", encoded.len());
    }

    if level == Level::Two {
        encoded = encoded
            .into_par_iter()
            .flat_map(|commands| reencode(&commands))
            .collect();

        println!("Found {} possible 2 level encodings", encoded.len());
    }

    encoded
        .into_par_iter()
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn get_numeric_part(code: &[Key]) -> usize {
    code.iter()
        .filter_map(|key| {
            if let Ok(value) = usize::try_from(*key) {
                return Some(value);
            } else {
                return None;
            }
        })
        .map(|v| v.to_string())
        .join("")
        .parse()
        .unwrap()
}

fn calculate_checksum(encoded: &[DirectionalCommand], code: &[Key]) -> usize {
    let a = encoded.len();
    let b = get_numeric_part(code);
    a * b
}

fn calculate_checksums(codes: &[Vec<Key>]) -> usize {
    codes
        .par_iter()
        .map(|code| {
            let encoded = find_shortest_encoding(&code, Level::Two);
            calculate_checksum(&encoded, &code)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./inputs/day21.txt").expect("Failed to read file");

    let codes: Vec<_> = input
        .trim()
        .lines()
        .map(|line| parse_keys(line.trim()))
        .collect();

    let result = calculate_checksums(&codes);

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_shortest_paths_between_keys() {
        let shortest_paths_between_keys = compute_shortest_paths_between_keys();

        assert_eq!(
            shortest_paths_between_keys
                .get(&(Key::Activate, Key::Four))
                .unwrap(),
            &[vec![
                DirectionalCommand::Up,
                DirectionalCommand::Up,
                DirectionalCommand::Left,
                DirectionalCommand::Left,
            ]]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        assert_eq!(
            shortest_paths_between_keys
                .get(&(Key::Seven, Key::Zero))
                .unwrap(),
            &[vec![
                DirectionalCommand::Right,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
            ]]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        assert_eq!(
            shortest_paths_between_keys
                .get(&(Key::Nine, Key::One))
                .unwrap(),
            &[
                vec![
                    DirectionalCommand::Left,
                    DirectionalCommand::Left,
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                ],
                vec![
                    DirectionalCommand::Down,
                    DirectionalCommand::Down,
                    DirectionalCommand::Left,
                    DirectionalCommand::Left,
                ]
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        assert_eq!(
            shortest_paths_between_keys
                .get(&(Key::Seven, Key::Six))
                .unwrap(),
            &[
                vec![
                    DirectionalCommand::Right,
                    DirectionalCommand::Right,
                    DirectionalCommand::Down
                ],
                vec![
                    DirectionalCommand::Down,
                    DirectionalCommand::Right,
                    DirectionalCommand::Right
                ]
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_compute_shortest_paths_between_commands() {
        let shortest_paths_between_commands = compute_shortest_paths_between_commands();

        assert_eq!(
            shortest_paths_between_commands
                .get(&(DirectionalCommand::Activate, DirectionalCommand::Activate))
                .unwrap(),
            &[].into_iter().collect::<HashSet<_>>()
        );

        assert_eq!(
            shortest_paths_between_commands
                .get(&(DirectionalCommand::Right, DirectionalCommand::Up))
                .unwrap(),
            &[
                vec![DirectionalCommand::Up, DirectionalCommand::Left,],
                vec![DirectionalCommand::Left, DirectionalCommand::Up,],
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        assert_eq!(
            shortest_paths_between_commands
                .get(&(DirectionalCommand::Left, DirectionalCommand::Activate))
                .unwrap(),
            &[
                vec![
                    DirectionalCommand::Right,
                    DirectionalCommand::Right,
                    DirectionalCommand::Up
                ],
                vec![
                    DirectionalCommand::Right,
                    DirectionalCommand::Up,
                    DirectionalCommand::Right,
                ]
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_encode() {
        assert!(encode(&parse_keys("029A")).contains(&vec![
            DirectionalCommand::Left,
            DirectionalCommand::Activate,
            DirectionalCommand::Up,
            DirectionalCommand::Activate,
            DirectionalCommand::Right,
            DirectionalCommand::Up,
            DirectionalCommand::Up,
            DirectionalCommand::Activate,
            DirectionalCommand::Down,
            DirectionalCommand::Down,
            DirectionalCommand::Down,
            DirectionalCommand::Activate,
        ]));
    }

    #[test]
    fn test_reencode() {
        assert_eq!(
            reencode(&[
                DirectionalCommand::Up,
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
            ]),
            [
                vec![
                    DirectionalCommand::Left,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Right,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Down,
                    DirectionalCommand::Left,
                    DirectionalCommand::Activate
                ],
                vec![
                    DirectionalCommand::Left,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Right,
                    DirectionalCommand::Activate,
                    DirectionalCommand::Left,
                    DirectionalCommand::Down,
                    DirectionalCommand::Activate
                ],
            ]
            .into_iter()
            .collect()
        );

        assert!(
            reencode(&[
                DirectionalCommand::Left,
                DirectionalCommand::Activate,
                DirectionalCommand::Up,
                DirectionalCommand::Activate,
                DirectionalCommand::Up,
                DirectionalCommand::Right,
                DirectionalCommand::Up,
                DirectionalCommand::Activate,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Down,
                DirectionalCommand::Activate,
            ])
            .len()
                > 0,
        );
    }

    #[test]
    fn test_find_shortest_encoding() {
        assert_eq!(
            find_shortest_encoding(&parse_keys("029A"), Level::Zero).len(),
            12
        );
        assert_eq!(
            find_shortest_encoding(&parse_keys("029A"), Level::Two).len(),
            68
        );
        assert_eq!(
            find_shortest_encoding(&parse_keys("980A"), Level::Two).len(),
            60
        );
        assert_eq!(
            find_shortest_encoding(&parse_keys("179A"), Level::Two).len(),
            68
        );
        assert_eq!(
            find_shortest_encoding(&parse_keys("456A"), Level::Two).len(),
            64
        );
        assert_eq!(
            find_shortest_encoding(&parse_keys("379A"), Level::Two).len(),
            64
        );
    }

    #[test]
    fn test_calculate_checksums() {
        assert_eq!(
            calculate_checksums(
                &["029A", "980A", "179A", "456A", "379A"]
                    .into_iter()
                    .map(|input| parse_keys(input))
                    .collect::<Vec<_>>()
            ),
            126384
        );
    }

    #[test]
    fn test_get_numeric_part() {
        assert_eq!(get_numeric_part(&parse_keys("029A")), 29);
        assert_eq!(get_numeric_part(&parse_keys("980A")), 980);
        assert_eq!(get_numeric_part(&parse_keys("179A")), 179);
        assert_eq!(get_numeric_part(&parse_keys("456A")), 456);
        assert_eq!(get_numeric_part(&parse_keys("379A")), 379);
    }
}
