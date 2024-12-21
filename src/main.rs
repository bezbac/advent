use std::{collections::HashMap, fs};

use pathfinding::prelude::astar_bag_collect;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
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

        Map { tiles, start, end }
    }

    fn find_shortest_paths(&self) -> Option<(Vec<Vec<(usize, usize)>>, usize)> {
        let start = self.start;
        let end = self.end;
        let result = astar_bag_collect(
            &start,
            |&(x, y)| {
                [
                    (x as isize, y as isize - 1),
                    (x as isize, y as isize + 1),
                    (x as isize - 1, y as isize),
                    (x as isize + 1, y as isize),
                ]
                .into_iter()
                .filter_map(|(x, y)| -> Option<(usize, usize)> {
                    let is_within_bounds = x >= 0
                        && y >= 0
                        && (x as usize) < self.width()
                        && (y as usize) < self.height();

                    if !is_within_bounds {
                        return None;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    if let Tile::Wall = self.tiles[y][x] {
                        return None;
                    }

                    Some((x, y))
                })
                .map(|(x, y)| ((x, y), 1))
                .collect::<Vec<_>>()
            },
            |&(x, y)| {
                let (ex, ey) = end;

                (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                    .sqrt() as usize
            },
            |position| position == &end,
        );

        result
    }
}

fn find_cheats(map: Map) -> (usize, HashMap<usize, usize>) {
    let baseline = map.find_shortest_paths();

    let baseline = baseline.unwrap().1;

    let results: Vec<(usize, usize)> = (0..map.height())
        .flat_map(|y| (0..map.width()).map(move |x| (x, y)))
        .filter(|&(x, y)| map.tiles[y][x] == Tile::Wall)
        .into_iter()
        .par_bridge()
        .filter_map(|(x, y)| {
            let mut derived = map.clone();
            derived.tiles[y][x] = Tile::Empty;

            let paths = derived.find_shortest_paths();

            let Some((paths, cost)) = paths else {
                return None;
            };

            let saved_cost = baseline.abs_diff(cost);

            if saved_cost < 1 {
                return None;
            }

            Some((saved_cost, paths.len()))
        })
        .collect();

    let mut result = HashMap::new();

    for (saved_cost, count) in results {
        let entry = result.entry(saved_cost).or_default();
        *entry += count;
    }

    (baseline, result)
}

fn main() {
    let input = fs::read_to_string("./inputs/day20.txt").expect("Failed to read file");

    let map = Map::parse(&input);

    let (_, cheats) = find_cheats(map);

    let result: usize = cheats
        .iter()
        .filter_map(|(saved, count)| {
            if saved < &100 {
                return None;
            }

            return Some(count);
        })
        .sum();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
        "#;

        let map = Map::parse(input);

        let (baseline, cheats) = find_cheats(map);

        assert_eq!(baseline, 84);

        assert_eq!(
            cheats,
            [
                (64, 1),
                (40, 1),
                (38, 1),
                (36, 1),
                (20, 1),
                (12, 3),
                (10, 2),
                (8, 4),
                (6, 2),
                (4, 14),
                (2, 14)
            ]
            .into_iter()
            .collect::<HashMap<usize, usize>>()
        )
    }
}
