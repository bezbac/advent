use std::fs;

struct ClawGame {
    target: (isize, isize),
    move_a: (isize, isize),
    move_b: (isize, isize),
}

impl ClawGame {
    fn min_token_cost(&self) -> Option<isize> {
        let (ax, ay) = self.move_a;
        let (bx, by) = self.move_b;
        let (tx, ty) = self.target;

        let nb = (ty * ax - tx * ay) / (by * ax - bx * ay);
        let na = (tx - nb * bx) / ax;

        if (ax * na + bx * nb, ay * na + by * nb) != (tx, ty) {
            return None;
        }

        Some(na * 3 + nb)
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.trim().lines();

        let move_a = {
            let line = lines.next().unwrap();
            let line = line.trim_start_matches("Button A: ");
            let line = line.replace("X+", "");
            let line = line.replace("Y+", "");

            let mut move_a = line.split(", ").into_iter();

            (
                move_a.next().unwrap().parse().unwrap(),
                move_a.next().unwrap().parse().unwrap(),
            )
        };

        let move_b = {
            let line = lines.next().unwrap();
            let line = line.trim_start_matches("Button B: ");
            let line = line.replace("X+", "");
            let line = line.replace("Y+", "");

            let mut move_a = line.split(", ").into_iter();

            (
                move_a.next().unwrap().parse().unwrap(),
                move_a.next().unwrap().parse().unwrap(),
            )
        };

        let target = {
            let line = lines.next().unwrap();
            let line = line.trim_start_matches("Prize: ");
            let line = line.replace("X=", "");
            let line = line.replace("Y=", "");

            let mut target = line.split(", ").into_iter();

            (
                target.next().unwrap().parse().unwrap(),
                target.next().unwrap().parse().unwrap(),
            )
        };

        Self {
            target,
            move_a,
            move_b,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day13.txt").expect("Failed to read file");

    let mut result = 0;

    for game_input in input.trim().split("\n\n") {
        let game = ClawGame::parse(game_input.trim());

        if let Some(cost) = game.min_token_cost() {
            result += cost;
        }
    }

    println!("Result (Part 1): {result}");

    let mut result = 0;

    for game_input in input.trim().split("\n\n") {
        let mut game = ClawGame::parse(game_input.trim());

        game.target = (
            game.target.0 + 10000000000000,
            game.target.1 + 10000000000000,
        );

        if let Some(cost) = game.min_token_cost() {
            result += cost;
        }
    }

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use crate::ClawGame;

    #[test]
    fn test_game_1() {
        let input = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
        "#;

        let game = ClawGame::parse(input);

        assert_eq!(game.min_token_cost(), Some(280));
    }

    #[test]
    fn test_game_2() {
        let input = r#"
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
        "#;

        let game = ClawGame::parse(input);

        assert_eq!(game.min_token_cost(), None);
    }

    #[test]
    fn test_game_3() {
        let input = r#"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
        "#;

        let game = ClawGame::parse(input);

        assert_eq!(game.min_token_cost(), Some(200));
    }

    #[test]
    fn test_game_4() {
        let input = r#"
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "#;

        let game = ClawGame::parse(input);

        assert_eq!(game.min_token_cost(), None);
    }
}
