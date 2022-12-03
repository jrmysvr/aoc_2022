use crate::aoc::input;
use std::ops::Range;

pub fn run() {
    println!("Day 4");
    //let _ = input::read_day(_);
    //let _ = parse_input(_);
    let solution_part_1 = "";
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type Ranges = (Range<u32>, Range<u32>);

fn parse_input(content: String) -> Vec<Ranges> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_parse_input() {

        let expected = vec![
            (2..4, 6..8),
            (2..3, 4..5),
            (5..7, 7..9),
            (2..8, 3..7),
            (6..6, 4..6),
            (2..6, 4..8)
        ];
        assert_eq!(expected, parse_input(test_content()));
    }
}
