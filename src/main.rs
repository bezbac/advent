use std::fs;

fn count_words_in_iter<I: Iterator<Item = char>>(chars: &mut I, word: &str) -> usize {
    let word_chars = word.chars().collect::<Vec<_>>();

    let mut result = 0;
    let mut current = vec![];
    for c in chars.by_ref() {
        if c == word_chars[current.len()] {
            current.push(c);
        } else {
            current.clear();

            if c == word_chars[0] {
                current.push(c);
            }
        }

        if current.len() == word_chars.len() {
            result += 1;
            current.clear();
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
enum DiagonalDirection {
    TopLeftToBottomRight,
    TopRightToBottomLeft,
}

#[derive(Debug, Clone)]
struct DiagonalIter {
    chars: Vec<char>,
    direction: DiagonalDirection,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    finished: bool,
}

impl DiagonalIter {
    fn new(input: &str, direction: DiagonalDirection, start_x: usize, start_y: usize) -> Self {
        let chars: Vec<char> = input.chars().filter(|c| c != &'\n').collect();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

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

impl Iterator for DiagonalIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let result = self.chars.get(self.x + self.y * self.width);

        // Bottom row
        if self.y == self.height - 1 {
            self.finished = true;
            return result.copied();
        }

        if self.direction == DiagonalDirection::TopLeftToBottomRight {
            // Right column
            if self.x == self.width - 1 {
                self.finished = true;
            } else {
                self.x += 1;
                self.y += 1;
            }
        }

        if self.direction == DiagonalDirection::TopRightToBottomLeft {
            // Left column
            if self.x == 0 {
                self.finished = true;
            } else {
                self.x -= 1;
                self.y += 1;
            }
        }

        result.copied()
    }
}

fn count_word_occurences(input: &str, word: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut result = 0;

    for line in input.lines() {
        // Horizontal forward
        let forward_count = count_words_in_iter(&mut line.chars().clone(), word);

        // Horizontal backward
        let backward_count = count_words_in_iter(&mut line.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    // Vertical
    for i in 0..width {
        let str: String = input
            .chars()
            .filter(|c| c != &'\n')
            .skip(i)
            .step_by(width)
            .collect();

        let forward_count = count_words_in_iter(&mut str.chars().clone(), word);
        let backward_count = count_words_in_iter(&mut str.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    // Diagonal (top left to bottom right) starting at left
    for i in 0..height {
        let start_x = 0;
        let start_y = i;
        let diagonal_iter = DiagonalIter::new(
            input,
            DiagonalDirection::TopLeftToBottomRight,
            start_x,
            start_y,
        );

        let str: String = diagonal_iter.collect();

        let forward_count = count_words_in_iter(&mut str.chars().clone(), word);
        let backward_count = count_words_in_iter(&mut str.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    // Diagonal (top left to bottom right) starting at top
    // Skip the first line, as it was already covered by the previous loop
    for i in 1..width {
        let start_x = i;
        let start_y = 0;
        let diagonal_iter = DiagonalIter::new(
            input,
            DiagonalDirection::TopLeftToBottomRight,
            start_x,
            start_y,
        );

        let str: String = diagonal_iter.collect();

        let forward_count = count_words_in_iter(&mut str.chars().clone(), word);
        let backward_count = count_words_in_iter(&mut str.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    // Diagonal (top right to bottom left) starting at right
    for i in 0..height {
        let start_x = width - 1;
        let start_y = i;
        let diagonal_iter = DiagonalIter::new(
            input,
            DiagonalDirection::TopRightToBottomLeft,
            start_x,
            start_y,
        );

        let str: String = diagonal_iter.collect();

        let forward_count = count_words_in_iter(&mut str.chars().clone(), word);
        let backward_count = count_words_in_iter(&mut str.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    // Diagonal (top right to bottom left) starting at top
    // Skip the first line, as it was already covered by the previous loop
    for i in 0..width - 1 {
        let start_x = i;
        let start_y = 0;
        let diagonal_iter = DiagonalIter::new(
            input,
            DiagonalDirection::TopRightToBottomLeft,
            start_x,
            start_y,
        );

        let str: String = diagonal_iter.collect();

        let forward_count = count_words_in_iter(&mut str.chars().clone(), word);
        let backward_count = count_words_in_iter(&mut str.chars().rev().clone(), word);

        result += forward_count + backward_count;
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day4.txt").expect("Failed to read file");

    let result = count_word_occurences(&input, "XMAS");
    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_in_iter() {
        assert_eq!(count_words_in_iter(&mut "....XXMAS.".chars(), "XMAS"), 1);
        assert_eq!(count_words_in_iter(&mut "XMASAS".chars(), "XMAS"), 1);
        assert_eq!(count_words_in_iter(&mut "AAXMAS".chars(), "XMAS"), 1);
    }

    #[test]
    fn test_diagonal_iter_top_left_square() {
        let input = "ABC\nDEF\nGHI";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 2)
                .collect::<Vec<_>>(),
            vec!['G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 1)
                .collect::<Vec<_>>(),
            vec!['D', 'H']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A', 'E', 'I']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B', 'F']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 2, 0)
                .collect::<Vec<_>>(),
            vec!['C']
        );
    }

    #[test]
    fn test_diagonal_iter_top_right_square() {
        let input = "ABC\nDEF\nGHI";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 2, 2)
                .collect::<Vec<_>>(),
            vec!['I']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 2, 1)
                .collect::<Vec<_>>(),
            vec!['F', 'H']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 2, 0)
                .collect::<Vec<_>>(),
            vec!['C', 'E', 'G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B', 'D']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A']
        );
    }

    #[test]
    fn test_diagonal_iter_top_left_tall() {
        let input = "AB\nCD\nEF\nGH";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 3)
                .collect::<Vec<_>>(),
            vec!['G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 2)
                .collect::<Vec<_>>(),
            vec!['E', 'H']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 1)
                .collect::<Vec<_>>(),
            vec!['C', 'F']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A', 'D']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B']
        );
    }

    #[test]
    fn test_diagonal_iter_top_right_tall() {
        let input = "AB\nCD\nEF\nGH";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B', 'C']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 1)
                .collect::<Vec<_>>(),
            vec!['D', 'E']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 2)
                .collect::<Vec<_>>(),
            vec!['F', 'G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 3)
                .collect::<Vec<_>>(),
            vec!['H']
        );
    }

    #[test]
    fn test_diagonal_iter_top_left_wide() {
        let input = "ABCD\nEFGH";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 1)
                .collect::<Vec<_>>(),
            vec!['E']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A', 'F']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B', 'G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 2, 0)
                .collect::<Vec<_>>(),
            vec!['C', 'H']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopLeftToBottomRight, 3, 0)
                .collect::<Vec<_>>(),
            vec!['D']
        );
    }

    #[test]
    fn test_diagonal_iter_top_right_wide() {
        let input = "ABCD\nEFGH";
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 0, 0)
                .collect::<Vec<_>>(),
            vec!['A']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 1, 0)
                .collect::<Vec<_>>(),
            vec!['B', 'E']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 2, 0)
                .collect::<Vec<_>>(),
            vec!['C', 'F']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 3, 0)
                .collect::<Vec<_>>(),
            vec!['D', 'G']
        );
        assert_eq!(
            DiagonalIter::new(input, DiagonalDirection::TopRightToBottomLeft, 3, 1)
                .collect::<Vec<_>>(),
            vec!['H']
        );
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
}
