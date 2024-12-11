use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Height(usize),
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        if char == '.' || char == '\n' {
                            Tile::Empty
                        } else {
                            Tile::Height(
                                char.to_digit(10)
                                    .expect(&format!("Could not parse {char} a number"))
                                    as usize,
                            )
                        }
                    })
                    .collect()
            })
            .collect();
        Self { map }
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.map.get(y).and_then(|row| row.get(x)).copied()
    }

    fn width(&self) -> usize {
        self.map.get(0).map_or(0, |row| row.len())
    }

    fn height(&self) -> usize {
        self.map.len()
    }
}

fn get_neighbor_positions(
    x: usize,
    y: usize,
    map_width: usize,
    map_height: usize,
) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if x > 0 {
        result.push((x - 1, y));
    }

    if y > 0 {
        result.push((x, y - 1));
    }

    if x < map_width - 1 {
        result.push((x + 1, y));
    }

    if y < map_height - 1 {
        result.push((x, y + 1));
    }

    result
}

#[derive(Debug)]
struct Graph {
    map: Map,

    // Adjecency list linking from (x, y) to a set of (x, y) positions,
    // that are exactly 1 height above the current position
    edges: HashMap<(usize, usize), HashSet<(usize, usize)>>,
}

impl Graph {
    fn new(map: Map) -> Self {
        let mut edges = Vec::new();

        for (y, row) in map.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let Tile::Height(height) = cell else {
                    continue;
                };

                let neighbors = get_neighbor_positions(x, y, map.width(), map.height());

                for (nx, ny) in neighbors {
                    match map.get(nx, ny) {
                        None | Some(Tile::Empty) => continue,
                        Some(Tile::Height(neighbor_height)) => {
                            if neighbor_height == height + 1 {
                                edges.push(((x, y), (nx, ny)));
                            }
                        }
                    }
                }
            }
        }

        let mut adjecancy_list = HashMap::new();

        for (from, to) in edges {
            let entry = adjecancy_list.entry(from).or_insert(HashSet::new());
            entry.insert(to);
        }

        Self {
            map,
            edges: adjecancy_list,
        }
    }

    fn find_unique_paths(&self) -> usize {
        let mut paths = HashSet::new();

        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                match self.map.get(x, y) {
                    Some(Tile::Height(0)) => {}
                    _ => continue,
                }

                let mut queue = vec![vec![(x, y)]];

                while let Some(path) = queue.pop() {
                    let (x, y) = path.last().unwrap().clone();

                    match self.map.get(x, y) {
                        Some(Tile::Height(h)) if h == 9 => {
                            paths.insert(path.clone());
                        }
                        _ => {}
                    }

                    let Some(next) = self.edges.get(&(x, y)) else {
                        continue;
                    };

                    for (nx, ny) in next {
                        let mut new_path = path.clone();
                        new_path.push((*nx, *ny));
                        queue.push(new_path);
                    }
                }
            }
        }

        paths.len()
    }

    fn get_reachable_positions_with_height(
        &self,
        x: usize,
        y: usize,
        height: usize,
    ) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();

        let mut visited = HashSet::new();
        let mut queue = vec![(x, y)];

        while let Some((x, y)) = queue.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));

            match self.map.get(x, y) {
                Some(Tile::Height(h)) if h == height => {
                    result.insert((x, y));
                }
                _ => {}
            }

            let Some(next) = self.edges.get(&(x, y)) else {
                continue;
            };

            for (nx, ny) in next {
                queue.push((*nx, *ny));
            }
        }

        result
    }

    // Returns a hashmap from a trailhead position with height 0 to a set of reachable positions with height 9
    fn find_all_trailheads(&self) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut result: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                match self.map.get(x, y) {
                    Some(Tile::Height(0)) => {}
                    _ => continue,
                }

                result.insert((x, y), self.get_reachable_positions_with_height(x, y, 9));
            }
        }

        result
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day10.txt").expect("Failed to read file");

    let map = Map::parse(&input);
    let graph = Graph::new(map);

    let trailheads = graph.find_all_trailheads();
    let result: usize = trailheads.values().map(|x| x.len()).sum();

    println!("Result (Part 1): {result}");

    let unique_path_count = graph.find_unique_paths();

    println!("Result (Part 2): {unique_path_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reachable_positions_with_height() {
        let input = r#"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let positions = graph.get_reachable_positions_with_height(3, 0, 9);

        assert_eq!(positions, vec![(0, 6), (6, 6)].into_iter().collect());
    }

    #[test]
    fn find_all_trailheads_first() {
        let input = r#"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let trailheads = graph.find_all_trailheads();

        dbg!(&graph);

        let expected: HashMap<(usize, usize), HashSet<(usize, usize)>> =
            [((3, 0), vec![(0, 6), (6, 6)])]
                .iter()
                .cloned()
                .map(|(start, end)| {
                    (
                        start,
                        end.iter().cloned().collect::<HashSet<(usize, usize)>>(),
                    )
                })
                .collect();

        assert_eq!(trailheads, expected);
    }

    #[test]
    fn find_all_trailheads_second() {
        let input = r#"
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let trailheads = graph.find_all_trailheads();

        let expected: HashMap<(usize, usize), HashSet<(usize, usize)>> =
            [((1, 0), vec![(3, 5)]), ((5, 6), vec![(3, 5), (4, 0)])]
                .iter()
                .cloned()
                .map(|(start, end)| {
                    (
                        start,
                        end.iter().cloned().collect::<HashSet<(usize, usize)>>(),
                    )
                })
                .collect();

        assert_eq!(trailheads, expected);
    }

    #[test]
    fn find_all_trailheads_third() {
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let trailheads = graph.find_all_trailheads();

        let result: usize = trailheads.values().map(|x| x.len()).sum();

        assert_eq!(result, 36);
    }

    #[test]
    fn find_all_unique_paths_first() {
        let input = r#"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let unique_paths = graph.find_unique_paths();

        assert_eq!(unique_paths, 2);
    }

    #[test]
    fn find_all_unique_paths_second() {
        let input = r#"
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let unique_paths = graph.find_unique_paths();

        assert_eq!(unique_paths, 3);
    }

    #[test]
    fn find_all_unique_paths_third() {
        let input = r#"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let unique_paths = graph.find_unique_paths();

        assert_eq!(unique_paths, 13);
    }

    #[test]
    fn find_all_unique_paths_fourth() {
        let input = r#"
012345
123456
234567
345678
4.6789
56789.
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let unique_paths = graph.find_unique_paths();

        assert_eq!(unique_paths, 227);
    }

    #[test]
    fn find_all_unique_paths_fifth() {
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "#;

        let map = Map::parse(&input);
        let graph = Graph::new(map);
        let unique_paths = graph.find_unique_paths();

        assert_eq!(unique_paths, 81);
    }
}
