use std::fs;

fn count_word_occurences_in_iter<I: Iterator<Item = char>>(chars: &mut I, word: &str) -> usize {
    let mut result = 0;

    let mut matched = 0;
    let mut matched_rev = 0;
    for c in chars.by_ref() {
        if Some(c) == word.chars().nth(matched) {
            matched += 1;
        } else {
            matched = 0;

            if Some(c) == word.chars().nth(0) {
                matched += 1;
            }
        }

        if matched == word.len() {
            result += 1;
            matched = 0;
        }

        if Some(c) == word.chars().nth(word.len() - 1 - matched_rev) {
            matched_rev += 1;
        } else {
            matched_rev = 0;

            if Some(c) == word.chars().nth(word.len() - 1) {
                matched_rev += 1;
            }
        }

        if matched_rev == word.len() {
            result += 1;
            matched_rev = 0;
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
enum DiagonalDirection {
    TopLeftToBottomRight,
    TopRightToBottomLeft,
    BottomLeftToTopRight,
    BottomRightToTopLeft,
}

#[derive(Debug, Clone)]
struct DiagonalIter<'a> {
    chars: &'a [char],
    direction: DiagonalDirection,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    finished: bool,
}

impl<'a> DiagonalIter<'a> {
    fn new(
        chars: &'a [char],
        direction: DiagonalDirection,
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
    ) -> Self {
        Self {
            chars,

            direction,

            width,
            height,

            x: start_x,
            y: start_y,

            finished: false,
        }
    }
}

impl<'a> Iterator for DiagonalIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let result = self.chars.get(self.x + self.y * self.width);

        if self.direction == DiagonalDirection::TopLeftToBottomRight
            || self.direction == DiagonalDirection::TopRightToBottomLeft
        {
            // Bottom row
            if self.y == self.height - 1 {
                self.finished = true;
                return result.copied();
            }
        }

        if self.direction == DiagonalDirection::BottomLeftToTopRight
            || self.direction == DiagonalDirection::BottomRightToTopLeft
        {
            // Top row
            if self.y == 0 {
                self.finished = true;
                return result.copied();
            }
        }

        if self.direction == DiagonalDirection::TopLeftToBottomRight
            || self.direction == DiagonalDirection::BottomLeftToTopRight
        {
            // Right column
            if self.x == self.width - 1 {
                self.finished = true;
                return result.copied();
            }
        }

        if self.direction == DiagonalDirection::TopRightToBottomLeft
            || self.direction == DiagonalDirection::BottomRightToTopLeft
        {
            // Left column
            if self.x == 0 {
                self.finished = true;
                return result.copied();
            }
        }

        if self.direction == DiagonalDirection::TopLeftToBottomRight
            || self.direction == DiagonalDirection::BottomLeftToTopRight
        {
            self.x += 1;
        } else {
            self.x -= 1;
        }

        if self.direction == DiagonalDirection::TopLeftToBottomRight
            || self.direction == DiagonalDirection::TopRightToBottomLeft
        {
            self.y += 1;
        } else {
            self.y -= 1;
        }

        result.copied()
    }
}

fn count_word_occurences(input: &str, word: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();

    let create_diagonal_iter = |direction: DiagonalDirection, x: usize, y: usize| {
        DiagonalIter::new(&chars, direction, width, height, x, y)
    };

    let mut result = 0;

    // Horizontal
    for line in input.lines() {
        result += count_word_occurences_in_iter(&mut line.chars(), word);
    }

    // Vertical
    for i in 0..width {
        let mut iter = input.chars().filter(|c| c != &'\n').skip(i).step_by(width);
        result += count_word_occurences_in_iter(&mut iter, word);
    }

    // Diagonal (top left to bottom right) starting at left
    for i in 0..height {
        let mut iter = create_diagonal_iter(DiagonalDirection::TopLeftToBottomRight, 0, i);
        result += count_word_occurences_in_iter(&mut iter, word);
    }

    // Diagonal (top left to bottom right) starting at top
    // Skip the first line, as it was already covered by the previous loop
    for i in 1..width {
        let mut iter = create_diagonal_iter(DiagonalDirection::TopLeftToBottomRight, i, 0);
        result += count_word_occurences_in_iter(&mut iter, word);
    }

    // Diagonal (top right to bottom left) starting at right
    for i in 0..height {
        let mut iter = create_diagonal_iter(DiagonalDirection::TopRightToBottomLeft, width - 1, i);
        result += count_word_occurences_in_iter(&mut iter, word);
    }

    // Diagonal (top right to bottom left) starting at top
    // Skip the first line, as it was already covered by the previous loop
    for i in 0..width - 1 {
        let mut iter = create_diagonal_iter(DiagonalDirection::TopRightToBottomLeft, i, 0);
        result += count_word_occurences_in_iter(&mut iter, word);
    }

    result
}

fn find_cross_occurences(input: &str, word: &str) -> usize {
    assert!(word.len() % 2 == 1, "Word length must be odd");

    let middle_character = word.chars().nth(word.len() / 2).unwrap();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();

    let mut occurences = 0;

    for y in 0..height {
        'outer: for x in 0..width {
            let character = chars.get(x + y * width).unwrap();

            if character != &middle_character {
                continue;
            }

            for (d1, d2) in [
                (
                    DiagonalDirection::TopLeftToBottomRight,
                    DiagonalDirection::BottomRightToTopLeft,
                ),
                (
                    DiagonalDirection::TopRightToBottomLeft,
                    DiagonalDirection::BottomLeftToTopRight,
                ),
            ] {
                let mut diagonal: Vec<char> = vec![];

                diagonal.extend(
                    DiagonalIter::new(&chars, d1, width, height, x, y)
                        .skip(1)
                        .take(word.len() / 2),
                );
                diagonal.push(*character);
                diagonal.extend(
                    DiagonalIter::new(&chars, d2, width, height, x, y)
                        .skip(1)
                        .take(word.len() / 2),
                );

                if count_word_occurences_in_iter(&mut diagonal.into_iter(), word) == 0 {
                    continue 'outer;
                }
            }

            occurences += 1;
        }
    }

    occurences
}

fn main() {
    let input = fs::read_to_string("./inputs/day4.txt").expect("Failed to read file");

    let result = count_word_occurences(&input, "XMAS");
    println!("Result (Part 1): {result}");

    let result = find_cross_occurences(&input, "MAS");
    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_in_iter() {
        assert_eq!(
            count_word_occurences_in_iter(&mut "XMAS".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "SAMX".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "....XXMAS.".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "XMASAS".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "AAXMAS".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "..SAMX".chars(), "XMAS"),
            1
        );
        assert_eq!(
            count_word_occurences_in_iter(&mut "XMASAMX".chars(), "XMAS"),
            2
        );
    }

    #[test]
    fn test_diagonal_iter_top_left_square() {
        let input = "ABC\nDEF\nGHI";
        let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();

        for (x, y, expected) in [
            (0, 2, vec!['G']),
            (0, 1, vec!['D', 'H']),
            (0, 0, vec!['A', 'E', 'I']),
            (1, 0, vec!['B', 'F']),
            (2, 0, vec!['C']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopLeftToBottomRight, 3, 3, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }

        for (x, y, expected) in [
            (2, 2, vec!['I']),
            (2, 1, vec!['F', 'H']),
            (2, 0, vec!['C', 'E', 'G']),
            (1, 0, vec!['B', 'D']),
            (0, 0, vec!['A']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopRightToBottomLeft, 3, 3, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn test_diagonal_iter_tall() {
        let input = "AB\nCD\nEF\nGH";
        let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();

        for (x, y, expected) in [
            (0, 3, vec!['G']),
            (0, 2, vec!['E', 'H']),
            (0, 1, vec!['C', 'F']),
            (0, 0, vec!['A', 'D']),
            (1, 0, vec!['B']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopLeftToBottomRight, 2, 4, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }

        for (x, y, expected) in [
            (0, 0, vec!['A']),
            (1, 0, vec!['B', 'C']),
            (1, 1, vec!['D', 'E']),
            (1, 2, vec!['F', 'G']),
            (1, 3, vec!['H']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopRightToBottomLeft, 2, 4, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn test_diagonal_iter_wide() {
        let input = "ABCD\nEFGH";
        let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();

        for (x, y, expected) in [
            (0, 1, vec!['E']),
            (0, 0, vec!['A', 'F']),
            (1, 0, vec!['B', 'G']),
            (2, 0, vec!['C', 'H']),
            (3, 0, vec!['D']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopLeftToBottomRight, 4, 2, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }

        for (x, y, expected) in [
            (0, 0, vec!['A']),
            (1, 0, vec!['B', 'E']),
            (2, 0, vec!['C', 'F']),
            (3, 0, vec!['D', 'G']),
            (3, 1, vec!['H']),
        ] {
            assert_eq!(
                DiagonalIter::new(&chars, DiagonalDirection::TopRightToBottomLeft, 4, 2, x, y)
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn test_count_word_occurences_top_left() {
        let input = r#"
XMAS
MM..
A.A.
S..S
        "#
        .trim();

        assert_eq!(count_word_occurences(input, "XMAS"), 3);
    }

    #[test]
    fn test_count_word_occurences_top_right() {
        let input = r#"
SAMX
..MM
.A.A
S..S
        "#
        .trim();

        assert_eq!(count_word_occurences(input, "XMAS"), 3);
    }

    #[test]
    fn test_count_word_occurences_bottom_right() {
        let input = r#"
S..S
.A.A
..MM
SAMX
        "#
        .trim();

        assert_eq!(count_word_occurences(input, "XMAS"), 3);
    }

    #[test]
    fn test_count_word_occurences_bottom_left() {
        let input = r#"
S..S
A.A.
MM..
XMAS
        "#
        .trim();

        assert_eq!(count_word_occurences(input, "XMAS"), 3);
    }

    #[test]
    fn test_count_word_occurences_small() {
        let input = r#"
..X...
.SAMX.
.A..A.
XMAS.S
.X....
        "#
        .trim();

        let horizontal_forward = 1;
        let horizontal_backward = 1;
        let vertical_forward = 0;
        let vertical_backward = 1;
        let diagonal_forward = 1;

        let total = horizontal_forward
            + horizontal_backward
            + vertical_forward
            + vertical_backward
            + diagonal_forward;

        assert_eq!(count_word_occurences(input, "XMAS"), total);
    }

    #[test]
    fn test_count_word_occurences_large() {
        let input = r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
            "#
        .trim();

        assert_eq!(count_word_occurences(input, "XMAS"), 18);
    }

    #[test]
    fn find_crosses_single() {
        let input = r#"
M.S
.A.
M.S
            "#
        .trim();

        assert_eq!(find_cross_occurences(input, "MAS"), 1);
    }

    #[test]
    fn find_crosses_large() {
        let input = r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
            "#
        .trim();

        assert_eq!(find_cross_occurences(input, "MAS"), 9);
    }
}
