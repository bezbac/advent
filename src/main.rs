use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Map {
    width: usize,
    height: usize,
    nodes: HashSet<(usize, usize)>,
}

impl Map {
    fn parse(input: &str, freq: char) -> Self {
        let input = input.trim();

        let mut nodes = HashSet::new();

        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            if y > height {
                height = y;
            }

            for (x, c) in line.chars().enumerate() {
                if x > width {
                    width = x;
                }

                if c == freq {
                    nodes.insert((x, y));
                }
            }
        }

        Self {
            width,
            height,
            nodes,
        }
    }

    fn find_antinodes(&self) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();

        for (i, a) in self.nodes.iter().enumerate() {
            for (j, b) in self.nodes.iter().enumerate() {
                if i == j {
                    continue;
                }

                let points = get_line_points(self.width, self.height, a, b);

                for point in points {
                    if point == *a || point == *b {
                        continue;
                    }

                    let dist_a = (point.0 as isize - a.0 as isize).abs()
                        + (point.1 as isize - a.1 as isize).abs();

                    let dist_b = (point.0 as isize - b.0 as isize).abs()
                        + (point.1 as isize - b.1 as isize).abs();

                    if dist_a == dist_b * 2 || dist_b == dist_a * 2 {
                        antinodes.insert(point);
                    }
                }
            }
        }

        antinodes
    }
}

fn is_in_bounds(width: usize, height: usize, x: isize, y: isize) -> bool {
    x >= 0 && x <= width as isize && y >= 0 && y <= height as isize
}

fn get_line_points(
    width: usize,
    height: usize,
    a: &(usize, usize),
    b: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let mut x_step = *x2 as isize - *x1 as isize;
    let mut y_step = *y2 as isize - *y1 as isize;

    while x_step % 2 == 0 && y_step % 2 == 0 {
        x_step /= 2;
        y_step /= 2;
    }

    let mut points = HashSet::new();

    let mut i = 0;
    loop {
        let new_x = *x1 as isize + x_step * i;
        let new_y = *y1 as isize + y_step * i;

        if !is_in_bounds(width, height, new_x, new_y) {
            break;
        }

        i += 1;
        points.insert((new_x as usize, new_y as usize));
    }

    let mut i = 0;
    loop {
        let new_x = *x1 as isize - x_step * i;
        let new_y = *y1 as isize - y_step * i;

        if !is_in_bounds(width, height, new_x, new_y) {
            break;
        }

        i += 1;
        points.insert((new_x as usize, new_y as usize));
    }

    points
}

struct MultiFreqMap {
    freqs: HashMap<char, Map>,
}

impl MultiFreqMap {
    fn parse(input: &str, freqs: &HashSet<char>) -> Self {
        let mut result = HashMap::new();

        for freq in freqs {
            result.insert(*freq, Map::parse(input, *freq));
        }

        Self { freqs: result }
    }

    fn find_antinodes(&self) -> HashSet<(char, usize, usize)> {
        let mut antinodes = HashSet::new();

        for (freq, map) in &self.freqs {
            antinodes.extend(
                map.find_antinodes()
                    .iter()
                    .map(|point| (*freq, point.0, point.1)),
            );
        }

        antinodes
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./inputs/day8.txt").expect("Failed to read file");

    let freqs = input
        .chars()
        .filter(|c| c != &'.' && c != &'\n')
        .collect::<HashSet<_>>();

    let map = MultiFreqMap::parse(&input, &freqs);

    let antinodes = map.find_antinodes();

    let unique_positions = antinodes
        .iter()
        .map(|(_, x, y)| (x, y))
        .collect::<HashSet<_>>();

    let result = unique_positions.len();

    println!("Result (Part 1): {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let input = r#"
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
.......... 
        "#;

        let map = Map::parse(input, 'a');

        assert_eq!(
            map.nodes,
            [(4, 3), (5, 5)].iter().cloned().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_get_line_points() {
        assert_eq!(
            get_line_points(9, 9, &(4, 3), &(5, 5)),
            [(4, 3), (5, 5), (6, 7), (7, 9), (3, 1)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_find_antinodes_first() {
        let input = r#"
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.......... 
        "#;

        let map = Map::parse(input, 'a');

        assert_eq!(
            map.find_antinodes(),
            [(3, 1), (6, 7)].iter().cloned().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_find_antinodes_second() {
        let input = r#"
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.......... 
        "#;

        let map = Map::parse(input, 'a');

        assert_eq!(
            map.find_antinodes(),
            [(3, 1), (6, 7), (2, 6), (0, 2)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_find_antinodes_multiple_frequencies() {
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;

        let map = MultiFreqMap::parse(input, &{
            let mut set = HashSet::new();
            set.insert('A');
            set.insert('0');
            set
        });

        assert_eq!(
            map.find_antinodes(),
            [
                ('0', 3, 6),
                ('0', 10, 2),
                ('0', 11, 0),
                ('0', 6, 0),
                ('0', 6, 5),
                ('0', 9, 4),
                ('0', 1, 5),
                ('0', 3, 1),
                ('0', 2, 3),
                ('0', 0, 7),
                ('A', 7, 7),
                ('A', 4, 2),
                ('A', 10, 11),
                ('A', 10, 10),
                ('A', 3, 1)
            ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
        );
    }
}
