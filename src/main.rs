use anyhow::Result;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct World {
    map: Map,
    guard_direction: Direction,
    guard_x: usize,
    guard_y: usize,
    guard_in_world: bool,
}

impl World {
    fn step(&mut self) {
        if !self.guard_in_world {
            return;
        }

        let (next_position_x, next_position_y) = match self.guard_direction {
            Direction::Up => (self.guard_x as isize, self.guard_y as isize - 1),
            Direction::Down => (self.guard_x as isize, self.guard_y as isize + 1),
            Direction::Left => (self.guard_x as isize - 1, self.guard_y as isize),
            Direction::Right => (self.guard_x as isize + 1, self.guard_y as isize),
        };

        if next_position_x < 0 {
            self.guard_in_world = false;
            return;
        }

        if next_position_y < 0 {
            self.guard_in_world = false;
            return;
        }

        let next_position_x = next_position_x as usize;
        let next_position_y = next_position_y as usize;

        if next_position_x >= self.map.width() {
            self.guard_in_world = false;
            return;
        }

        if next_position_y >= self.map.height() {
            self.guard_in_world = false;
            return;
        }

        let next_tile = &self.map.get_tile(next_position_x, next_position_y);

        match next_tile {
            Tile::Empty => {
                self.guard_x = next_position_x;
                self.guard_y = next_position_y;
            }
            Tile::Wall => {
                // Rotate 90 degrees to the right
                self.guard_direction = self.guard_direction.rotate_clockwise();
            }
        }
    }

    fn read(input: &str) -> Self {
        let input = input.trim();

        let mut guard_x = None;
        let mut guard_y = None;
        let mut guard_direction = None;

        let mut rows = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '^' => {
                        guard_x = Some(x);
                        guard_y = Some(y);
                        guard_direction = Some(Direction::Up);
                        Tile::Empty
                    }
                    '>' => {
                        guard_x = Some(x);
                        guard_y = Some(y);
                        guard_direction = Some(Direction::Right);
                        Tile::Empty
                    }
                    '<' => {
                        guard_x = Some(x);
                        guard_y = Some(y);
                        guard_direction = Some(Direction::Left);
                        Tile::Empty
                    }
                    'v' => {
                        guard_x = Some(x);
                        guard_y = Some(y);
                        guard_direction = Some(Direction::Down);
                        Tile::Empty
                    }
                    _ => panic!("Unknown tile type '{}'", c),
                };

                row.push(tile);
            }

            rows.push(row);
        }

        let map = Map { tiles: rows };

        let guard_x = guard_x.expect("Guard x position not found");
        let guard_y = guard_y.expect("Guard y position not found");
        let guard_direction = guard_direction.expect("Guard direction not found");

        let world = World {
            map,
            guard_direction,
            guard_x,
            guard_y,
            guard_in_world: true,
        };

        world
    }
}

fn count_distinct_visited_positions(world: &mut World) -> usize {
    let mut visited_tile_positions = vec![];

    while world.guard_in_world {
        if !world.guard_in_world {
            break;
        }

        visited_tile_positions.push((world.guard_x, world.guard_y));
        world.step();
    }

    visited_tile_positions.iter().collect::<HashSet<_>>().len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day6.txt").expect("Failed to read file");

    let mut world = World::read(&input);

    let distinct_positions = count_distinct_visited_positions(&mut world);

    println!("Result (Part 1): {distinct_positions}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_world_empty() {
        let input = r#"
...
.^.
...
        "#;

        let world = World::read(input);

        assert_eq!(world.map.width(), 3);
        assert_eq!(world.map.height(), 3);
        assert!(world
            .map
            .tiles
            .iter()
            .flatten()
            .all(|tile| *tile == Tile::Empty));
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 1);
        assert_eq!(world.guard_direction, Direction::Up);
    }

    #[test]
    fn test_read_world_simple() {
        let input = r#"
.#...
###..
.>...
...##
        "#;

        let world = World::read(input);

        assert_eq!(world.map.width(), 5);
        assert_eq!(world.map.height(), 4);

        assert_eq!(world.map.get_tile(0, 0), Tile::Empty);
        assert_eq!(world.map.get_tile(1, 0), Tile::Wall);
        assert_eq!(world.map.get_tile(2, 0), Tile::Empty);
        assert_eq!(world.map.get_tile(3, 0), Tile::Empty);
        assert_eq!(world.map.get_tile(4, 0), Tile::Empty);
        assert_eq!(world.map.get_tile(0, 1), Tile::Wall);
        assert_eq!(world.map.get_tile(1, 1), Tile::Wall);
        assert_eq!(world.map.get_tile(2, 1), Tile::Wall);
        assert_eq!(world.map.get_tile(3, 1), Tile::Empty);
        assert_eq!(world.map.get_tile(4, 1), Tile::Empty);
        assert_eq!(world.map.get_tile(0, 2), Tile::Empty);
        assert_eq!(world.map.get_tile(1, 2), Tile::Empty);
        assert_eq!(world.map.get_tile(2, 2), Tile::Empty);
        assert_eq!(world.map.get_tile(3, 2), Tile::Empty);
        assert_eq!(world.map.get_tile(4, 2), Tile::Empty);
        assert_eq!(world.map.get_tile(0, 3), Tile::Empty);
        assert_eq!(world.map.get_tile(1, 3), Tile::Empty);
        assert_eq!(world.map.get_tile(2, 3), Tile::Empty);
        assert_eq!(world.map.get_tile(3, 3), Tile::Wall);
        assert_eq!(world.map.get_tile(4, 3), Tile::Wall);

        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 2);
        assert_eq!(world.guard_direction, Direction::Right);
    }

    #[test]
    fn test_simulation_up_unobstructed() {
        let input = r#"
...
.^.
...
        "#;

        let mut world = World::read(input);

        assert_eq!(world.guard_direction, Direction::Up);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Up);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 0);

        world.step();

        assert_eq!(world.guard_direction, Direction::Up);
        assert_eq!(world.guard_in_world, false);
    }

    #[test]
    fn test_simulation_right_unobstructed() {
        let input = r#"
...
.>.
...
        "#;

        let mut world = World::read(input);

        assert_eq!(world.guard_direction, Direction::Right);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Right);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 2);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Right);
        assert_eq!(world.guard_in_world, false);
    }

    #[test]
    fn test_simulation_down_unobstructed() {
        let input = r#"
...
.v.
...
        "#;

        let mut world = World::read(input);

        assert_eq!(world.guard_direction, Direction::Down);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Down);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 2);

        world.step();

        assert_eq!(world.guard_direction, Direction::Down);
        assert_eq!(world.guard_in_world, false);
    }

    #[test]
    fn test_simulation_left_unobstructed() {
        let input = r#"
...
.<.
...
        "#;

        let mut world = World::read(input);

        assert_eq!(world.guard_direction, Direction::Left);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 1);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Left);
        assert_eq!(world.guard_in_world, true);
        assert_eq!(world.guard_x, 0);
        assert_eq!(world.guard_y, 1);

        world.step();

        assert_eq!(world.guard_direction, Direction::Left);
        assert_eq!(world.guard_in_world, false);
    }

    #[test]
    fn test_simulation_snake() {
        let input = r#"
#####
#...#
#^..#
....#
#####
        "#;

        let mut world = World::read(input);

        let visited_positions = count_distinct_visited_positions(&mut world);

        assert_eq!(visited_positions, 9);
    }
}
