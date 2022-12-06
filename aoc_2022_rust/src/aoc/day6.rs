use crate::aoc::input;
use std::collections::HashSet;

pub fn run() {
    println!("Day 6");
    let content = input::read_day(6);
    let solution_part_1 = find_marker_index_by_length(&content, 4);
    let solution_part_2 = find_marker_index_by_length(&content, 14);
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

fn find_marker_index_by_length(buf: &String, marker_length: usize) -> usize {
    let buf = buf.chars().collect::<Vec<char>>();
    for ix in marker_length..buf.len() {
        let marker_set: HashSet<&char> = HashSet::from_iter(&buf[ix - marker_length..ix]);
        if marker_set.len() == marker_length {
            return ix;
        }
    }

    panic!("No marker found!");
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_marker_index_finding() {
        assert_eq!(find_marker_index_by_length(&test_content(), 4), 7);
    }
}
