use crate::aoc::input;

pub fn run() {
    println!("Day 22");
    //let _ = input::read_day(_);
    //let _ = parse_input(_);
    let solution_part_1 = "";
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type Num = i64;
enum Path {
    Move(Num),
    Left,
    Right
}

struct Notes {
    path: Vec<Path>,
}

fn notes_from_puzzle_input(puzzle_input: &String) -> Notes {
    N
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_parse_input() {
        let notes = notes_from_puzzle_input(&test_content());
        /*
                let expected_board_map =
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.
                */
        // let expected_path = "10R5L5R10L4R5L5";
        let expected_path = vec![
            Path::Move(10),
            Path::Right,
            Path::Move(5),
            Path::Left,
            Path::Move(5),
            Path::Right,
            Path::Move(10),
            Path::Left,
            Path::Move(4),
            Path::Right,
            Path::Move(5),
            Path::Left,
            Path::Move(5),
        ];
        // assert_eq!(board_map, expected_board_map);
        assert_eq!(notes.path, expected_path);
    }
}
