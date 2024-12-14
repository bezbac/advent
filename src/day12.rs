use std::{collections::HashSet, fs};

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<char>>,
}

impl Map {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn get_tile(&self, x: usize, y: usize) -> char {
        self.tiles[y][x]
    }

    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        Self { tiles }
    }
}

fn find_unvisited(
    visited: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    for y in 0..height {
        for x in 0..width {
            if !visited.contains(&(x, y)) {
                return Some((x, y));
            }
        }
    }

    None
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Area {
    area: usize,
    perimiter: usize,
    tiles: HashSet<(usize, usize)>,
}

fn get_areas(input: &Map) -> Vec<Area> {
    let mut result = Vec::new();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    let mut current_area = Area {
        area: 0,
        perimiter: 0,
        tiles: HashSet::new(),
    };

    loop {
        let mut stack = vec![(x, y)];

        while let Some((x1, y1)) = stack.pop() {
            x = x1;
            y = y1;

            if visited.contains(&(x, y)) {
                continue;
            }

            let c = input.get_tile(x, y);

            visited.insert((x, y));
            current_area.tiles.insert((x, y));
            current_area.area += 1;

            if x < 1 {
                // Out of bounds
                current_area.perimiter += 1;
            } else if input.get_tile(x - 1, y) != c {
                current_area.perimiter += 1;
            } else if !visited.contains(&(x - 1, y)) {
                stack.push((x - 1, y))
            }

            if y < 1 {
                // Out of bounds
                current_area.perimiter += 1;
            } else if input.get_tile(x, y - 1) != c {
                current_area.perimiter += 1;
            } else if !visited.contains(&(x, y - 1)) {
                stack.push((x, y - 1))
            }

            if x >= input.width() - 1 {
                // Out of bounds
                current_area.perimiter += 1;
            } else if input.get_tile(x + 1, y) != c {
                current_area.perimiter += 1;
            } else if !visited.contains(&(x + 1, y)) {
                stack.push((x + 1, y))
            }

            if y >= input.height() - 1 {
                // Out of bounds
                current_area.perimiter += 1;
            } else if input.get_tile(x, y + 1) != c {
                current_area.perimiter += 1;
            } else if !visited.contains(&(x, y + 1)) {
                stack.push((x, y + 1))
            }
        }

        result.push(current_area);
        current_area = Area {
            area: 0,
            perimiter: 0,
            tiles: HashSet::new(),
        };

        if let Some((x1, y1)) = find_unvisited(&visited, input.width(), input.height()) {
            x = x1;
            y = y1;
        } else {
            break;
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day12.txt").expect("Failed to read file");

    let map = Map::parse(&input);

    let data = get_areas(&map);

    let result: usize = data.iter().map(|area| area.area * area.perimiter).sum();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area_and_perimiter_first() {
        let input = r#"
AAAA
BBCD
BBCC
EEEC
        "#;

        let map = Map::parse(input);

        let areas = get_areas(&map);

        assert_eq!(areas.len(), 5);

        let mut values = areas
            .iter()
            .map(|area| (area.area, area.perimiter))
            .collect::<Vec<_>>();

        values.sort();

        let mut expected = vec![
            (4, 10), // A
            (4, 8),  // B
            (4, 10), // C
            (1, 4),  // D
            (3, 8),  // E
        ];

        expected.sort();

        dbg!(&areas);
        assert_eq!(values, expected);
    }

    #[test]
    fn test_get_area_and_perimiter_second() {
        let input = r#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
        "#;

        let map = Map::parse(input);

        let areas = get_areas(&map);

        assert_eq!(areas.len(), 5);

        let mut values = areas
            .iter()
            .map(|area| (area.area, area.perimiter))
            .collect::<Vec<_>>();

        values.sort();

        let mut expected = vec![
            (1, 4),   // X
            (1, 4),   // X
            (1, 4),   // X
            (1, 4),   // X
            (21, 36), // O
        ];

        expected.sort();

        dbg!(&areas);
        assert_eq!(values, expected);
    }

    #[test]
    fn test_get_area_and_perimiter_third() {
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "#;

        let map = Map::parse(input);

        let areas = get_areas(&map);

        assert_eq!(areas.len(), 11);

        let mut values = areas
            .iter()
            .map(|area| (area.area, area.perimiter))
            .collect::<Vec<_>>();

        values.sort();

        let mut expected = vec![
            (12, 18), // R
            (4, 8),   // I
            (14, 28), // C
            (10, 18), // F
            (13, 20), // V
            (11, 20), // J
            (1, 4),   // C
            (13, 18), // E
            (14, 22), // I
            (5, 12),  // M
            (3, 8),   // S
        ];

        expected.sort();

        dbg!(&areas);
        assert_eq!(values, expected);
    }
}
