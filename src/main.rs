use std::{collections::VecDeque, fs};

fn shift_right(row: &mut Vec<Option<Object>>, start: usize) -> bool {
    match row[start] {
        Some(Object::Box) => {}
        Some(Object::Wall) => return false,
        None => return true,
    }

    let mut current: Option<Object> = row[start];
    row[start] = None;

    for i in start + 1..row.len() {
        if current.is_none() {
            break;
        }

        if row[i] == Some(Object::Wall) {
            return false;
        }

        if row[i].is_none() {
            row[i] = current;
            current = None;
            break;
        }
    }

    if current.is_some() {
        return false;
    }

    true
}

fn shift_left(row: &mut Vec<Option<Object>>, start: usize) -> bool {
    match row[start] {
        Some(Object::Box) => {}
        Some(Object::Wall) => return false,
        None => return true,
    }

    let mut current: Option<Object> = row[start];
    row[start] = None;

    for i in (0..start).rev() {
        if current.is_none() {
            break;
        }

        if row[i] == Some(Object::Wall) {
            return false;
        }

        if row[i].is_none() {
            row[i] = current;
            current = None;
            break;
        }
    }

    if current.is_some() {
        return false;
    }

    true
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Wall,
    Box,
}

#[derive(Debug, PartialEq)]
struct Map {
    tiles: Vec<Vec<Option<Object>>>,
}

impl Map {
    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn print(&self, robot: Option<(usize, usize)>) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(robot) = robot {
                    if robot.0 == x && robot.1 == y {
                        print!("@");
                        continue;
                    }
                }

                match self.tiles[y][x] {
                    None => print!("."),
                    Some(Object::Box) => print!("O"),
                    Some(Object::Wall) => print!("#"),
                }
            }

            println!();
        }
    }

    fn parse(input: &str) -> (Self, (usize, usize)) {
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

        (Map { tiles }, robot)
    }

    fn get_row(&self, index: usize) -> Vec<Option<Object>> {
        self.tiles[index].clone()
    }

    fn set_row(&mut self, index: usize, row: Vec<Option<Object>>) {
        self.tiles[index] = row;
    }

    fn get_column(&self, index: usize) -> Vec<Option<Object>> {
        let mut result = vec![];

        for i in 0..self.height() {
            result.push(self.tiles[i][index])
        }

        result
    }

    fn set_column(&mut self, index: usize, column: Vec<Option<Object>>) {
        for i in 0..self.height() {
            self.tiles[i][index] = column[i]
        }
    }

    fn try_shift_right(&mut self, start: (usize, usize)) -> bool {
        let mut row = self.get_row(start.1);

        let should_update = shift_right(&mut row, start.0);

        if should_update {
            self.set_row(start.1, row);
            return true;
        }

        false
    }

    fn try_shift_left(&mut self, start: (usize, usize)) -> bool {
        let mut row = self.get_row(start.1);

        let should_update = shift_left(&mut row, start.0);

        if should_update {
            self.set_row(start.1, row);
            return true;
        }

        false
    }

    fn try_shift_down(&mut self, start: (usize, usize)) -> bool {
        let mut column = self.get_column(start.0);

        let should_update = shift_right(&mut column, start.1);

        if should_update {
            self.set_column(start.0, column);
            return true;
        }

        false
    }

    fn try_shift_up(&mut self, start: (usize, usize)) -> bool {
        let mut column = self.get_column(start.0);

        let should_update = shift_left(&mut column, start.1);

        if should_update {
            self.set_column(start.0, column);
            return true;
        }

        false
    }

    fn checksum(&self) -> usize {
        let mut result = 0;

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(Object::Box) = tile {
                    result += 100 * y + x;
                }
            }
        }

        result
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
    robot: (usize, usize),
}

impl Game {
    fn parse(input: &str) -> Game {
        let input = input.trim();

        let mut split = input.split("\n\n").into_iter();
        let tile_input = split.next().unwrap();
        let (map, robot) = Map::parse(tile_input);

        let mut instructions: VecDeque<Move> = VecDeque::new();

        while let Some(instructions_input) = split.next() {
            for c in instructions_input.trim().chars() {
                if c == '\n' {
                    continue;
                }

                if let Ok(instruction) = Move::try_from(c) {
                    instructions.push_back(instruction);
                } else {
                    panic!("Unknown instruction: {}", c);
                }
            }
        }

        Game {
            map,
            instructions,
            robot,
        }
    }

    fn step(&mut self) {
        let Some(instruction) = self.instructions.pop_front() else {
            return;
        };

        match instruction {
            Move::Right => {
                let next_pos = (self.robot.0 + 1, self.robot.1);
                if self.map.try_shift_right(next_pos) {
                    self.robot = next_pos
                }
            }
            Move::Left => {
                let next_pos = (self.robot.0 - 1, self.robot.1);
                if self.map.try_shift_left(next_pos) {
                    self.robot = next_pos
                }
            }
            Move::Down => {
                let next_pos = (self.robot.0, self.robot.1 + 1);
                if self.map.try_shift_down(next_pos) {
                    self.robot = next_pos
                }
            }
            Move::Up => {
                let next_pos = (self.robot.0, self.robot.1 - 1);
                if self.map.try_shift_up(next_pos) {
                    self.robot = next_pos
                }
            }
        }
    }

    fn run(&mut self) {
        while self.instructions.len() > 0 {
            self.step();
        }
    }

    fn checksum(&self) -> usize {
        self.map.checksum()
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day15.txt").expect("Failed to read file");

    let mut game = Game::parse(&input);

    game.run();

    let result = game.checksum();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column() {
        let input = r#"
#
O
.
O
.
.
        "#;

        let (map, _) = Map::parse(input);

        let column = map.get_column(0);

        assert_eq!(
            column,
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                None,
                Some(Object::Box),
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_shift_right() {
        let input = r#"
..O.O#
        "#;

        let (mut map, _) = Map::parse(input);

        map.try_shift_right((0, 0));

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

        map.try_shift_right((2, 0));

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

        map.try_shift_right((3, 0));

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

    #[test]
    fn test_shift_left() {
        let input = r#"
#O.O..
        "#;

        let (mut map, _) = Map::parse(input);

        map.try_shift_left((5, 0));

        assert_eq!(
            map.tiles[0],
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                None,
                Some(Object::Box),
                None,
                None,
            ]
        );

        map.try_shift_left((3, 0));

        assert_eq!(
            map.tiles[0],
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                Some(Object::Box),
                None,
                None,
                None,
            ]
        );

        map.try_shift_left((2, 0));

        assert_eq!(
            map.tiles[0],
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                Some(Object::Box),
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_shift_up() {
        let input = r#"
#
O
.
O
.
.
        "#;

        let (mut map, _) = Map::parse(input);

        map.try_shift_up((0, 5));

        assert_eq!(
            map.get_column(0),
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                None,
                Some(Object::Box),
                None,
                None,
            ]
        );

        map.try_shift_up((0, 3));

        assert_eq!(
            map.get_column(0),
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                Some(Object::Box),
                None,
                None,
                None,
            ]
        );

        map.try_shift_up((0, 2));

        assert_eq!(
            map.get_column(0),
            vec![
                Some(Object::Wall),
                Some(Object::Box),
                Some(Object::Box),
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_shift_down() {
        let input = r#"
.
.
O
.
O
#
        "#;

        let (mut map, _) = Map::parse(input);

        map.try_shift_down((0, 0));

        assert_eq!(
            map.get_column(0),
            vec![
                None,
                None,
                Some(Object::Box),
                None,
                Some(Object::Box),
                Some(Object::Wall),
            ]
        );

        map.try_shift_down((0, 2));

        assert_eq!(
            map.get_column(0),
            vec![
                None,
                None,
                None,
                Some(Object::Box),
                Some(Object::Box),
                Some(Object::Wall),
            ]
        );

        map.try_shift_down((0, 3));

        assert_eq!(
            map.get_column(0),
            vec![
                None,
                None,
                None,
                Some(Object::Box),
                Some(Object::Box),
                Some(Object::Wall),
            ]
        );
    }

    #[test]
    fn test_example_one() {
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

        game.run();

        let expected = r#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########        
        "#;

        let (expected_map, expected_robot_position) = Map::parse(&expected);

        assert_eq!(game.robot, expected_robot_position);
        assert_eq!(game.map, expected_map);

        assert_eq!(game.checksum(), 2028);
    }

    #[test]
    fn test_example_two() {
        let input = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "#;

        let mut game = Game::parse(input);

        game.run();

        let expected = r#"
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########      
        "#;

        let (expected_map, expected_robot_position) = Map::parse(&expected);

        assert_eq!(game.robot, expected_robot_position);
        assert_eq!(game.map, expected_map);

        assert_eq!(game.checksum(), 10092);
    }
}
