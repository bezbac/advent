use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize),
}

fn discmap_to_blocks(discmap: &str) -> Vec<Block> {
    let mut result = vec![];
    let mut is_file = true;
    let mut file_id = 0;

    for char in discmap.chars() {
        if char == '\n' {
            continue;
        }

        let num = char.to_digit(10).unwrap();

        if is_file {
            for _ in 0..num {
                result.push(Block::File(file_id));
            }
            is_file = false;
            file_id += 1;
        } else {
            for _ in 0..num {
                result.push(Block::Empty);
            }
            is_file = true;
        }
    }

    result
}

fn compact_blocks_fragmented(blocks: &[Block]) -> Vec<Block> {
    let mut result = blocks.to_vec();

    let mut i = 0;
    let mut j = blocks.len() - 1;

    loop {
        while result[i] != Block::Empty {
            i += 1;
        }

        while result[j] == Block::Empty {
            j -= 1;
        }

        if i >= j {
            break;
        }

        result.swap(i, j);
    }

    result
}

fn compact_blocks_whole(blocks: &[Block]) -> Vec<Block> {
    let mut result = blocks.to_vec();

    let mut j = blocks.len() - 1;
    'outer: loop {
        if j == 0 {
            break;
        }

        while result[j] == Block::Empty {
            j -= 1;
        }

        let Block::File(file_id) = result[j] else {
            panic!("Expected file block");
        };

        let mut required_blocks = 1;
        while j >= required_blocks && result[j - required_blocks] == Block::File(file_id) {
            required_blocks += 1;
        }

        if j + 1 - required_blocks == 0 {
            break;
        }

        for i in 0..j {
            if result[i] != Block::Empty {
                continue;
            }

            let mut available_blocks = 0;
            while result[i + available_blocks] == Block::Empty {
                available_blocks += 1;
            }

            if available_blocks >= required_blocks {
                for k in 0..required_blocks {
                    result.swap(i + k, j - k);
                }

                continue 'outer;
            }
        }

        j -= required_blocks;
    }

    result
}

fn checksum(compacted: &[Block]) -> usize {
    let mut result = 0;

    for (index, block) in compacted.iter().enumerate() {
        match block {
            Block::Empty => continue,
            Block::File(file_id) => {
                result += index * file_id;
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("./inputs/day9.txt").expect("Failed to read file");

    let result = checksum(&compact_blocks_fragmented(&discmap_to_blocks(&input)));

    println!("Result (Part 1): {result}");

    let result = checksum(&compact_blocks_whole(&discmap_to_blocks(&input)));

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn serialize_blocks(blocks: &[Block]) -> String {
        blocks
            .iter()
            .map(|b| match b {
                Block::Empty => '.',
                Block::File(file_id) => std::char::from_digit(*file_id as u32, 10).unwrap(),
            })
            .collect()
    }

    fn parse_blocks(input: &str) -> Vec<Block> {
        let mut result = vec![];

        for c in input.chars() {
            match c {
                '.' => result.push(Block::Empty),
                _ => result.push(Block::File(c.to_digit(10).unwrap() as usize)),
            }
        }

        result
    }

    #[test]
    fn test_discmap_to_blocks() {
        assert_eq!(discmap_to_blocks("12345"), parse_blocks("0..111....22222"));
        assert_eq!(
            discmap_to_blocks("2333133121414131402"),
            parse_blocks("00...111...2...333.44.5555.6666.777.888899")
        );
    }

    #[test]
    fn test_compact_blocks() {
        assert_eq!(
            compact_blocks_fragmented(&parse_blocks("0..111....22222")),
            parse_blocks("022111222......")
        );
        assert_eq!(
            compact_blocks_fragmented(&parse_blocks("00...111...2...333.44.5555.6666.777.888899")),
            parse_blocks("0099811188827773336446555566..............")
        );
    }

    #[test]
    fn test_compact_blocks_whole() {
        assert_eq!(
            serialize_blocks(&compact_blocks_whole(&parse_blocks(
                "00...111...2...333.44.5555.6666.777.888899"
            ))),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(&parse_blocks("0099811188827773336446555566..............")),
            1928
        );
        assert_eq!(
            checksum(&parse_blocks("00992111777.44.333....5555.6666.....8888..")),
            2858
        );
    }
}
