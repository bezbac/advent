use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Wall,
    Box,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Option<Object>>>,
    robot: (usize, usize),
}

impl Map {
    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn print(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.tiles[y][x] {
                    None => print!("."),
                    Some(Object::Box) => print!("O"),
                    Some(Object::Wall) => print!("#"),
                }
            }
            println!();
        }
    }

    fn parse(input: &str) -> Self {
        let input = input.trim();

        let mut split = input.split("\n\n").into_iter();
        let tile_input = split.next().unwrap();
        let mut robot = (0, 0);

        let mut tiles: Vec<Vec<Option<Object>>> = Vec::new();

        for (y, line) in tile_input.lines().enumerate() {
            let mut row: Vec<Option<Object>> = Vec::new();

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(Some(Object::Wall)),
                    'O' => row.push(Some(Object::Box)),
                    '@' => {
                        row.push(None);
                        robot = (x, y);
                    }
                    '.' => row.push(None),
                    _ => panic!("Unknown character: {}", c),
                }
            }

            tiles.push(row);
        }

        Map { tiles, robot }
    }

    fn try_shift_right(&mut self, start: (usize, usize)) {
        let mut row = self.tiles[start.1].clone();

        let mut current: Option<Object> = None;
        for i in start.0..self.width() {
            dbg!(i, current);

            if current.is_none() {
                current = row[i];
                continue;
            }

            let remember = row[i];
            row[i] = Some(current.unwrap());

            match remember {
                None => {
                    // We reached an empty space
                    current = None;
                    break;
                }
                Some(Object::Wall) => {
                    // We reached a wall, the shift did not work
                    return;
                }
                Some(Object::Box) => {}
            }
        }

        self.tiles[start.1] = row;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct Game {
    map: Map,
    instructions: VecDeque<Move>,
}

impl Game {
    fn parse(input: &str) -> Game {
        let input = input.trim();

        let mut split = input.split("\n\n").into_iter();
        let tile_input = split.next().unwrap();
        let map = Map::parse(tile_input);

        let instructions_input = split.next().unwrap();
        let mut instructions: VecDeque<Move> = VecDeque::new();

        for c in instructions_input.chars() {
            if let Ok(instruction) = Move::try_from(c) {
                instructions.push_back(instruction);
            } else {
                panic!("Unknown instruction: {}", c);
            }
        }

        Game { map, instructions }
    }

    fn step(&mut self) {}

    fn run(&mut self) {
        while self.instructions.len() > 0 {
            self.step();
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day15.txt").expect("Failed to read file");

    let result = 0;

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_right() {
        let input = r#"
..O.O#
        "#;

        let mut map = Map::parse(input);

        assert_eq!(
            map.tiles[0],
            vec![
                None,
                None,
                Some(Object::Box),
                None,
                Some(Object::Box),
                Some(Object::Wall)
            ]
        );

        map.try_shift_right((0, 0));

        assert_eq!(
            map.tiles[0],
            vec![
                None,
                None,
                None,
                Some(Object::Box),
                Some(Object::Box),
                Some(Object::Wall)
            ]
        );
    }
}
