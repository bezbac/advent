use std::fs;

use pathfinding::prelude::astar;

#[derive(Clone)]
struct MemorySpace {
    size: usize,
    tiles: Vec<Vec<bool>>,
}

impl MemorySpace {
    fn set(&mut self, coordinate: &(usize, usize), value: bool) {
        self.tiles[coordinate.1][coordinate.0] = value;
    }

    fn new(size: usize) -> Self {
        let tiles = vec![vec![false; size]; size];
        Self { size, tiles }
    }

    fn find_shortest_path_len(
        &self,
        start: &(usize, usize),
        end: &(usize, usize),
    ) -> Option<usize> {
        let result = astar(
            start,
            |&(x, y)| {
                [
                    (x as isize, y as isize - 1),
                    (x as isize, y as isize + 1),
                    (x as isize - 1, y as isize),
                    (x as isize + 1, y as isize),
                ]
                .into_iter()
                .filter_map(|(x, y)| -> Option<(usize, usize)> {
                    let is_within_bounds =
                        x >= 0 && y >= 0 && (x as usize) < self.size && (y as usize) < self.size;

                    if !is_within_bounds {
                        return None;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    if self.tiles[y][x] {
                        return None;
                    }

                    Some((x, y))
                })
                .map(|x| (x, 1))
                .collect::<Vec<_>>()
            },
            |&(x, y)| {
                let (ex, ey) = *end;

                (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                    .sqrt() as usize
            },
            |pos| pos == end,
        );

        result.map(|(_, cost)| cost)
    }
}

fn parse_coordinates(input: &str) -> Vec<(usize, usize)> {
    let input = input.trim();

    let mut result = vec![];

    for line in input.lines() {
        let mut parts = line.split(',');
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        result.push((x, y));
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day18.txt").expect("Failed to read file");

    let coordinates = parse_coordinates(&input);

    let mut memspace = MemorySpace::new(71);

    for coordinate in &coordinates[0..1024] {
        memspace.set(coordinate, true);
    }

    let shortest_path = memspace.find_shortest_path_len(&(0, 0), &(70, 70)).unwrap();

    println!("Result (Part 1): {shortest_path}");

    let mut memspace = memspace.clone();

    let mut result = None;
    for coordinate in coordinates.iter().skip(1024) {
        memspace.set(coordinate, true);

        if memspace
            .find_shortest_path_len(&(0, 0), &(70, 70))
            .is_none()
        {
            result = Some(coordinate);
            break;
        }
    }

    let result = result.unwrap();

    println!("Result (Part 2): {},{}", result.0, result.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "#;

        let coordinates = parse_coordinates(&input);

        let mut memspace = MemorySpace::new(7);

        for coordinate in &coordinates[0..12] {
            memspace.set(coordinate, true);
        }

        assert_eq!(
            memspace.tiles,
            vec![
                vec!['.', '.', '.', '#', '.', '.', '.'],
                vec!['.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '.', '.', '.', '#', '.', '.'],
                vec!['.', '.', '.', '#', '.', '.', '#'],
                vec!['.', '.', '#', '.', '.', '#', '.'],
                vec!['.', '#', '.', '.', '#', '.', '.'],
                vec!['#', '.', '#', '.', '.', '.', '.']
            ]
            .into_iter()
            .map(|row| row.into_iter().map(|char| char == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>()
        );

        let shortest_path = memspace.find_shortest_path_len(&(0, 0), &(6, 6));

        assert_eq!(shortest_path, Some(22));
    }
}
