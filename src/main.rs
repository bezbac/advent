use std::{collections::HashSet, fs};

use pathfinding::prelude::astar;

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn width(&self) -> usize {
        self.tiles.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn parse(input: &str) -> Map {
        let input = input.trim();

        let mut tiles = Vec::new();
        let mut start = None;
        let mut end = None;

        for (y, row) in input.lines().enumerate() {
            let mut row_tiles = Vec::new();

            for (x, c) in row.chars().enumerate() {
                match c {
                    '#' => {
                        row_tiles.push(Tile::Wall);
                    }
                    '.' => {
                        row_tiles.push(Tile::Empty);
                    }
                    'S' => {
                        row_tiles.push(Tile::Empty);
                        start = Some((x, y));
                    }
                    'E' => {
                        row_tiles.push(Tile::Empty);
                        end = Some((x, y));
                    }
                    _ => panic!("Unexpected character"),
                }
            }

            tiles.push(row_tiles);
        }

        let start = start.unwrap();
        let end = end.unwrap();

        Map { start, end, tiles }
    }

    fn print(&self, visited: Option<HashSet<(usize, usize)>>) {
        let visited = visited.unwrap_or_default();

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if visited.contains(&(x, y)) {
                    print!("x");
                    continue;
                }

                match tile {
                    Tile::Empty => print!("."),
                    Tile::Wall => print!("#"),
                }
            }
            print!("\n")
        }

        println!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        }
    }
}

struct Maze {
    map: Map,
    reindeer: (usize, usize, Direction),
}

impl Maze {
    fn parse(input: &str) -> Self {
        let map = Map::parse(input);

        let reindeer = (map.start.0, map.start.1, Direction::East);

        Self { map, reindeer }
    }

    fn next_positions(
        &self,
        current_position: (usize, usize, Direction),
    ) -> Vec<(usize, usize, Direction, usize)> {
        let forward_pos = match current_position.2 {
            Direction::North => (current_position.0 as isize, current_position.1 as isize - 1),
            Direction::South => (current_position.0 as isize, current_position.1 as isize + 1),
            Direction::West => (current_position.0 as isize - 1, current_position.1 as isize),
            Direction::East => (current_position.0 as isize + 1, current_position.1 as isize),
        };

        let rotated_current_position = vec![
            (
                current_position.0,
                current_position.1,
                current_position.2.rotate_clockwise(),
                1000,
            ),
            (
                current_position.0,
                current_position.1,
                current_position.2.rotate_counterclockwise(),
                1000,
            ),
        ];

        if forward_pos.0 < 0
            || forward_pos.1 < 0
            || forward_pos.0 > self.map.width() as isize
            || forward_pos.1 > self.map.height() as isize
        {
            return rotated_current_position;
        }

        let forward_pos = (forward_pos.0 as usize, forward_pos.1 as usize);

        let forward_tile = &self.map.tiles[forward_pos.1][forward_pos.0];

        match forward_tile {
            Tile::Empty => {
                let mut result = vec![(forward_pos.0, forward_pos.1, current_position.2, 1)];
                result.append(&mut rotated_current_position.clone());
                result
            }
            Tile::Wall => rotated_current_position,
        }
    }

    fn solve(&self) -> Option<usize> {
        let mut visited = HashSet::new();

        let result = astar(
            &self.reindeer,
            |&(x, y, direction)| {
                let successors = self
                    .next_positions((x, y, direction))
                    .iter()
                    .map(|(nx, ny, nd, cost)| ((*nx, *ny, *nd), *cost))
                    .collect::<Vec<_>>();

                visited.insert((x, y));

                successors
            },
            |&(x, y, _)| {
                let (ex, ey) = self.map.end;

                (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                    .sqrt() as usize
            },
            |(x, y, _)| (*x, *y) == self.map.end,
        );

        dbg!(&result);

        let path = result.clone().map(|(path, cost)| {
            path.iter()
                .map(|(x, y, _)| (*x, *y))
                .collect::<HashSet<(usize, usize)>>()
        });

        self.map.print(path);

        result.map(|(_, cost)| cost as usize)
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day16.txt").expect("Failed to read file");

    let maze = Maze::parse(&input);

    let result = maze.solve().unwrap();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "#;

        let maze = Maze::parse(input);

        assert_eq!(maze.solve(), Some(7036));
    }
}
