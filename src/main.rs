use std::fs;

use pathfinding::prelude::astar;

struct MemorySpace {
    size: usize,
    tiles: Vec<Vec<bool>>,
}

impl MemorySpace {
    fn parse(size: usize, input: &str, limit: usize) -> Self {
        let input = input.trim();

        let mut tiles = vec![vec![false; size]; size];

        for (i, line) in input.lines().enumerate() {
            if i >= limit {
                break;
            }

            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();

            tiles[y][x] = true;
        }

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
                let next_positions = [
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

                    return Some((x, y));
                })
                .map(|x| (x, 1))
                .collect::<Vec<_>>();

                next_positions
            },
            |&(x, y)| {
                let (ex, ey) = *end;

                (((ex as isize - x as isize).pow(2) + (ey as isize - y as isize).pow(2)) as f64)
                    .sqrt() as usize
            },
            |pos| pos == end,
        );

        result.map(|(_, cost)| cost as usize)
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day18.txt").expect("Failed to read file");

    let memspace = MemorySpace::parse(71, &input, 1024);

    let shortest_path = memspace.find_shortest_path_len(&(0, 0), &(70, 70)).unwrap();

    println!("Result (Part 1): {shortest_path}");
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

        let memspace = MemorySpace::parse(7, input, 12);

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
