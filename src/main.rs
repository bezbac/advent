use std::fs;

#[derive(Debug, PartialEq)]
struct Robot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn step(&mut self, world_width: usize, world_height: usize) {
        let x = self.x as isize + self.vx;
        let y = self.y as isize + self.vy;

        let x = if x < 0 {
            (world_width as isize + x) as usize
        } else if x >= world_width as isize {
            x as usize - world_width
        } else {
            x as usize
        };

        let y = if y < 0 {
            (world_height as isize + y) as usize
        } else if y >= world_height as isize {
            y as usize - world_height
        } else {
            y as usize
        };

        self.x = x;
        self.y = y;
    }

    fn parse(input: &str) -> Robot {
        let input = input.trim();
        let input = input.replace("p=", "");
        let input = input.replace("v=", "");
        let input = input.replace(" ", ",");

        let mut parts = input.split(",").into_iter();

        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let vx = parts.next().unwrap().parse().unwrap();
        let vy = parts.next().unwrap().parse().unwrap();

        Self { x, y, vx, vy }
    }
}

struct World {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl World {
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.step(self.width, self.height);
        }
    }

    fn safety_factor(&self) -> usize {
        let q1 = self
            .robots
            .iter()
            .filter(|robot| (robot.x < self.width / 2 && robot.y < self.height / 2))
            .count();

        let q2 = self
            .robots
            .iter()
            .filter(|robot| (robot.x > self.width / 2 && robot.y < self.height / 2))
            .count();

        let q3 = self
            .robots
            .iter()
            .filter(|robot| (robot.x < self.width / 2 && robot.y > self.height / 2))
            .count();

        let q4 = self
            .robots
            .iter()
            .filter(|robot| (robot.x > self.width / 2 && robot.y > self.height / 2))
            .count();

        q1 * q2 * q3 * q4
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day14.txt").expect("Failed to read file");

    let world_width = 101;
    let world_height = 103;

    let robots = input.lines().map(Robot::parse).collect::<Vec<_>>();

    let mut world = World {
        width: world_width,
        height: world_height,
        robots,
    };

    for _ in 0..100 {
        world.step();
    }

    let result = world.safety_factor();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Robot::parse("p=2,4 v=2,-3"),
            Robot {
                x: 2,
                y: 4,
                vx: 2,
                vy: -3
            }
        );
    }

    #[test]
    fn test_step() {
        let mut robot = Robot::parse("p=2,4 v=2,-3");

        robot.step(11, 7);

        assert_eq!(
            robot,
            Robot {
                x: 4,
                y: 1,
                vx: 2,
                vy: -3
            }
        );

        robot.step(11, 7);

        assert_eq!(
            robot,
            Robot {
                x: 6,
                y: 5,
                vx: 2,
                vy: -3
            }
        );

        robot.step(11, 7);

        assert_eq!(
            robot,
            Robot {
                x: 8,
                y: 2,
                vx: 2,
                vy: -3
            }
        );

        robot.step(11, 7);

        assert_eq!(
            robot,
            Robot {
                x: 10,
                y: 6,
                vx: 2,
                vy: -3
            }
        );

        robot.step(11, 7);

        assert_eq!(
            robot,
            Robot {
                x: 1,
                y: 3,
                vx: 2,
                vy: -3
            }
        );
    }

    #[test]
    fn test_safety_factor() {
        let mut w = World {
            width: 11,
            height: 7,
            robots: [
                (6, 0),
                (6, 0),
                (9, 0),
                (0, 2),
                (1, 3),
                (2, 3),
                (5, 4),
                (3, 5),
                (4, 5),
                (4, 5),
                (1, 6),
                (6, 6),
            ]
            .iter()
            .map(|(x, y)| Robot {
                x: *x,
                y: *y,
                vx: 0,
                vy: 0,
            })
            .collect(),
        };

        assert_eq!(w.safety_factor(), 12);
        assert_eq!(w.safety_factor(), 12);
    }
}
