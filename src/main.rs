use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Wall,
    Box,
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
    tiles: Vec<Vec<Option<Object>>>,
    robot: (usize, usize),
    instructions: VecDeque<Move>,
}

impl Game {
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

    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Object> {
        let row = self.tiles.get(y)?;
        let tile = row.get(x)?;
        *tile
    }

    fn set_tile(&mut self, x: usize, y: usize, value: Option<Object>) {
        self.tiles[y][x] = value;
    }

    fn swap_tiles(&mut self, a: (usize, usize), b: (usize, usize)) {
        let remember = self.get_tile(a.0, a.1);
        self.set_tile(a.0, a.1, self.get_tile(b.0, b.1));
        self.set_tile(b.0, b.1, remember);
    }

    fn parse(input: &str) -> Game {
        let input = input.trim();

        let mut split = input.split("\n\n").into_iter();
        let tile_input = split.next().unwrap();
        let instructions_input = split.next().unwrap();

        let mut tiles: Vec<Vec<Option<Object>>> = Vec::new();
        let mut robot = (0, 0);
        let mut instructions: VecDeque<Move> = VecDeque::new();

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

        for c in instructions_input.chars() {
            if let Ok(instruction) = Move::try_from(c) {
                instructions.push_back(instruction);
            } else {
                panic!("Unknown instruction: {}", c);
            }
        }

        Game {
            tiles,
            robot,
            instructions,
        }
    }

    fn shift_left(&mut self, pos: (usize, usize), n: usize) {
        for i in 1..=n {
            self.swap_tiles(pos, (pos.0 - i, pos.1));
        }
    }

    fn shift_right(&mut self, pos: (usize, usize), n: usize) {
        for i in 1..=n {
            self.swap_tiles((pos.0 + i, pos.1), (pos.0 + i + 1, pos.1));
        }
    }

    fn shift_up(&mut self, pos: (usize, usize), n: usize) {
        for i in 1..=n {
            self.swap_tiles(pos, (pos.0, pos.1 - i));
        }
    }

    fn shift_down(&mut self, pos: (usize, usize), n: usize) {
        for i in 1..=n {
            self.swap_tiles(pos, (pos.0, pos.1 + i));
        }
    }

    fn step(&mut self) {
        let mv = self.instructions.pop_front();

        let Some(mv) = mv else { return };

        match mv {
            Move::Up => {
                let mut empty_y = None;
                let mut y = self.robot.1 - 1;
                while y > 0 {
                    if self.get_tile(self.robot.0, y).is_none() {
                        empty_y = Some(y);
                    }
                    y -= 1;
                }

                let Some(empty_y) = empty_y else {
                    return;
                };

                self.shift_up(self.robot, empty_y.abs_diff(self.robot.0));
                self.robot = (self.robot.0, self.robot.1 - 1);
            }
            Move::Down => todo!(),
            Move::Left => {
                let empty_x = self.tiles[self.robot.1][0..self.robot.0]
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, tile)| tile.is_none())
                    .map(|(x, _)| x);

                let Some(empty_x) = empty_x else {
                    return;
                };

                self.shift_left(self.robot, empty_x.abs_diff(self.robot.0));
                self.robot = (self.robot.0 - 1, self.robot.1);
            }
            Move::Right => {
                let empty_x = self.tiles[self.robot.1][self.robot.0 + 1..self.width()]
                    .iter()
                    .enumerate()
                    .find(|(_, tile)| tile.is_none())
                    .map(|(x, _)| x);

                let Some(empty_x) = empty_x else {
                    return;
                };

                let empty_x = empty_x + self.robot.0;

                dbg!(empty_x);
                dbg!(self.robot.1);

                self.shift_right(self.robot, empty_x.abs_diff(self.robot.0));
                self.robot = (self.robot.0 + 1, self.robot.1);
            }
        }
    }

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
    fn test_shift_left() {
        let input = r#"
..O.O
        "#;

        let mut game = Game::parse(input);

        assert_eq!(
            game.tiles[0],
            vec![None, None, Some(Object::Box), None, Some(Object::Box)]
        );

        game.shift_left((4, 0), 1);

        assert_eq!(
            game.tiles[0],
            vec![None, None, Some(Object::Box), Some(Object::Box), None]
        );

        game.shift_left((3, 0), 2);

        assert_eq!(
            game.tiles[0],
            vec![None, Some(Object::Box), Some(Object::Box), None, None]
        );
    }

    #[test]
    fn test_mv_left_unobstructed() {
        let input = r#"
..@

<<
        "#;

        let mut game = Game::parse(input);

        assert_eq!(game.robot, (2, 0));

        game.run();

        assert_eq!(game.robot, (0, 0));
    }

    #[test]
    fn test_mv_right_unobstructed() {
        let input = r#"
@..

>>
        "#;

        let mut game = Game::parse(input);

        assert_eq!(game.robot, (0, 0));

        game.run();

        assert_eq!(game.robot, (2, 0));
    }

    #[test]
    fn test_mv_left_obstructed() {
        let input = r#"
.O.O@

<<<
        "#;

        let mut game = Game::parse(input);

        assert_eq!(game.robot, (4, 0));

        game.run();

        assert_eq!(game.robot, (2, 0));

        assert_eq!(game.tiles[0][0], Some(Object::Box));
        assert_eq!(game.tiles[0][1], Some(Object::Box));
        assert_eq!(game.tiles[0][2], None);
        assert_eq!(game.tiles[0][3], None);
        assert_eq!(game.tiles[0][4], None);
    }

    #[test]
    fn test_mv_right_obstructed() {
        let input = r#"
@O.O.

>>>
        "#;

        let mut game = Game::parse(input);

        game.print();

        assert_eq!(game.robot, (0, 0));

        game.step();

        game.print();

        game.step();

        game.print();

        assert_eq!(game.robot, (2, 0));

        assert_eq!(game.tiles[0][0], None);
        assert_eq!(game.tiles[0][1], None);
        assert_eq!(game.tiles[0][2], None);
        assert_eq!(game.tiles[0][3], Some(Object::Box));
        assert_eq!(game.tiles[0][4], Some(Object::Box));
    }

    #[test]
    fn test_parse() {
        let input = r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<        
        "#;

        let mut game = Game::parse(input);

        assert_eq!(game.tiles.len(), 8);
        assert_eq!(game.tiles.first().unwrap().len(), 8);

        assert_eq!(game.robot, (2, 2));

        assert_eq!(game.instructions.len(), 15);

        assert_eq!(game.instructions[0], Move::Left);
        assert_eq!(game.instructions[1], Move::Up);
        assert_eq!(game.instructions[2], Move::Up);
        assert_eq!(game.instructions[3], Move::Right);
        assert_eq!(game.instructions[4], Move::Right);
        assert_eq!(game.instructions[5], Move::Right);
        assert_eq!(game.instructions[6], Move::Down);
        assert_eq!(game.instructions[7], Move::Down);
        assert_eq!(game.instructions[8], Move::Left);
        assert_eq!(game.instructions[9], Move::Down);
        assert_eq!(game.instructions[10], Move::Right);

        game.step();
        assert_eq!(game.instructions.len(), 14);
        assert_eq!(game.robot, (2, 2));

        game.step();
        assert_eq!(game.instructions.len(), 13);
        assert_eq!(game.robot, (2, 1));

        game.step();
        assert_eq!(game.instructions.len(), 12);
        assert_eq!(game.robot, (2, 1));

        game.step();
        assert_eq!(game.instructions.len(), 11);
        assert_eq!(game.robot, (3, 1));

        game.step();
        assert_eq!(game.instructions.len(), 10);
        assert_eq!(game.robot, (4, 1));

        game.step();
        assert_eq!(game.instructions.len(), 9);
        assert_eq!(game.robot, (4, 1));

        game.step();
        assert_eq!(game.instructions.len(), 8);
        assert_eq!(game.robot, (4, 2));

        game.step();
        assert_eq!(game.instructions.len(), 7);
        assert_eq!(game.robot, (4, 3));
    }
}
