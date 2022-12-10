use crate::aoc::input;

pub fn run() {
    println!("Day 9");
    //let _ = input::read_day(_);
    //let _ = parse_input(_);
    let solution_part_1 = "";
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

/*
fn parse_input(content: String) -> _ {
}
*/

type Direction = char;
type Step = i64;
type NSteps = i64;
type Movement = (Direction, NSteps);
type Movements = Vec<Movement>;

fn direction_from(s: &str) -> Direction {
    s.chars().nth(0).unwrap()
}

fn step_from(s: &str) -> NSteps {
    s.chars().map(|c| c.to_digit(10).unwrap() as i64).nth(0).unwrap()
}

fn movement_from(mvnt: Vec<&str>) -> Movement {
    (direction_from(mvnt[0]), step_from(mvnt[1]))
}

fn movements_from_puzzle_input(puzzle_input: &String) -> Movements {
    puzzle_input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(movement_from)
        .collect::<Movements>()
}

type Pos = (Step, Step);

fn move_tail_position_with(head_pos: &Pos, tail_pos: &Pos) -> Pos {
    (0, 0)
}

fn move_position(head_pos: &Pos, tail_pos: &Pos, movement: &Movement) -> (Pos, Pos) {
    let (hx, hy) = head_pos;
    let (dir, n_steps) = movement;
    let (x, y) = match dir {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, -1),
        'D' => (0, 1),
        d => panic!("Unknown movement direction: {d}"),
    };
    let new_head_pos = (hx + x, hy + y);
    let new_tail_pos = move_tail_position_with(&new_head_pos, tail_pos);

    (new_head_pos, new_tail_pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_move_head() {
        let movements = movements_from_puzzle_input(&test_content());
        let expected_positions = vec![((0, 1), (0, 0)), ((0, 2), (0, 1))];
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        for (movement, expected_pos) in movements.iter().zip(expected_positions.iter()) {
            let (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, movement);
            let (expected_head_pos, expected_tail_pos) = expected_pos;
            assert_eq!(head_pos.0, expected_head_pos.0);
            assert_eq!(head_pos.1, expected_head_pos.1);
            assert_eq!(tail_pos.0, expected_tail_pos.0);
            assert_eq!(tail_pos.1, expected_tail_pos.1);
        }
    }

    #[test]
    fn test_parse_input() {
        let movements = movements_from_puzzle_input(&test_content());
        let expected = vec![('R', 4), ('U', 4), ('L', 3)];
        assert_eq!(Vec::from(&movements[0..3]), expected);
    }
}
