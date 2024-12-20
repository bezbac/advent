use std::{
    collections::{HashMap, HashSet},
    fs,
};

use pathfinding::prelude::astar_bag_collect;

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
}

struct Run {
    map: Map,
    remaining_cheats: usize,
    forbidden_cheat_positions: HashMap<usize, HashSet<(usize, usize)>>,
}

impl Run {
    fn next_positions(&self, &(x, y): &(usize, usize)) -> Vec<(usize, usize)> {
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
                && (x as usize) < self.map.width()
                && (y as usize) < self.map.height();

            if !is_within_bounds {
                return None;
            }

            let x = x as usize;
            let y = y as usize;

            if let Tile::Wall = self.map.tiles[y][x] {
                if self.remaining_cheats == 2
                    && self
                        .forbidden_cheat_positions
                        .get(&2)
                        .unwrap_or(&HashSet::new())
                        .contains(&(x, y))
                {
                    return None;
                }

                // FIXME: A cheat is only allowed *exactly once*
                if self.remaining_cheats == 1
                    && self
                        .forbidden_cheat_positions
                        .get(&1)
                        .unwrap_or(&HashSet::new())
                        .contains(&(x, y))
                {
                    return None;
                }

                if self.remaining_cheats == 0 {
                    return None;
                }
            }

            Some((x, y))
        })
        .collect()
    }

    fn find_shortest_paths(&self) -> Option<(Vec<Vec<(usize, usize)>>, usize)> {
        let result = astar_bag_collect(
            &self.map.start,
            |position| {
                let successors = self
                    .next_positions(position)
                    .iter()
                    .map(|(nx, ny)| ((*nx, *ny), 1))
                    .collect::<Vec<_>>();

                successors
            },
            |&(x, y)| {
                let (ex, ey) = self.map.end;

                (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                    .sqrt() as usize
            },
            |position| position == &self.map.end,
        );

        result
    }
}

fn find_cheat_positions(map: &Map, path: &[(usize, usize)]) -> Vec<(usize, usize)> {
    path.iter()
        .filter(|&(x, y)| map.tiles[*y][*x] == Tile::Wall)
        .copied()
        .collect()
}

fn find_cheats(map: Map) -> (usize, HashMap<usize, usize>) {
    let baseline = Run {
        map: map.clone(),
        remaining_cheats: 0,
        forbidden_cheat_positions: HashMap::new(),
    };

    let baseline = baseline.find_shortest_paths().unwrap().1;

    let mut result = HashMap::new();
    let mut forbidden_cheat_positions: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();

    loop {
        let cheat_run = Run {
            map: map.clone(),
            remaining_cheats: 2,
            forbidden_cheat_positions: forbidden_cheat_positions.clone(),
        };

        let paths = cheat_run.find_shortest_paths();

        let Some((paths, cost)) = paths else {
            break;
        };

        if cost == baseline {
            break;
        }

        let entry = result.entry(baseline.abs_diff(cost)).or_default();

        *entry += paths.len();

        for path in paths {
            let cheat_positions = find_cheat_positions(&map, &path);
            let mut cheat_positions = cheat_positions.iter();

            if let Some(position) = cheat_positions.next() {
                let entry = forbidden_cheat_positions.entry(1).or_default();
                entry.insert(*position);
            }

            if let Some(position) = cheat_positions.next() {
                let entry = forbidden_cheat_positions.entry(2).or_default();
                entry.insert(*position);
            }
        }
    }

    (baseline, result)
}

fn main() {
    let input = fs::read_to_string("./inputs/day20.txt").expect("Failed to read file");

    let map = Map::parse(&input);

    let result = 0;

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

        dbg!(cheats);
    }
}
