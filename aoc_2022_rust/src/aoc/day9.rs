use crate::aoc::input;

pub fn run() {
    println!("Day 9");
    let puzzle_input = input::read_day(9);
    let movements = movements_from_puzzle_input(&puzzle_input);
    let solution_part_1 = count_tail_positions_from(&movements, 1);
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type Direction = char;
type Step = i64;
type NSteps = i64;
type Movement = (Direction, NSteps);
type Movements = Vec<Movement>;

fn direction_from(s: &str) -> Direction {
    s.chars().nth(0).unwrap()
}

fn step_from(s: &str) -> NSteps {
    s.parse::<NSteps>().unwrap()
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

fn move_tail_position_with(head_pos: &Pos, prev_head_pos: &Pos, tail_pos: &Pos) -> Pos {
    let (hx, hy) = *head_pos;
    let (tx, ty) = *tail_pos;
    if (tx - hx).abs() <= 1 && (ty - hy).abs() <= 1 {
        *tail_pos
    } else {
        *prev_head_pos
    }
}

fn move_position(head_pos: &Pos, tail_pos: &Pos, movement: &Movement) -> (Pos, Pos) {
    let (dir, n_steps) = movement;
    let mut new_tail_pos = *tail_pos;
    let mut new_head_pos = *head_pos;
    for _ in 0..*n_steps {
        let (hx, hy) = new_head_pos;
        let (x, y) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            d => panic!("Unknown movement direction: {d}"),
        };
        new_head_pos = (hx + x, hy + y);
        let prev_head_pos = (hx, hy);
        new_tail_pos = move_tail_position_with(&new_head_pos, &prev_head_pos, &new_tail_pos);
    }

    (new_head_pos, new_tail_pos)
}

fn submovements_from(movement: &Movement) -> Movements {
    let mut submovements = Movements::new();
    let (dir, n_steps) = movement;
    for _ in 0..*n_steps {
        submovements.push((*dir, 1));
    }

    submovements
}

fn visualize_positions(positions: &std::collections::HashSet<Pos>) {
    let min_x = positions.iter().map(|pos| pos.0).min().unwrap() - 1;
    let max_x = positions.iter().map(|pos| pos.0).max().unwrap() + 2;
    let min_y = positions.iter().map(|pos| pos.1).min().unwrap() - 1;
    let max_y = positions.iter().map(|pos| pos.1).max().unwrap() + 2;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let pos = (x, y);
            if positions.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

type Rope = Vec<Pos>;

fn count_tail_positions_from(movements: &Movements, rope_length: usize) -> usize {
    let mut head_pos = (0, 0);
    let mut rope = Rope::new();
    rope.push(head_pos);
    for i in 1..=rope_length {
        rope.push((0, 0));
    }
    let mut tail_pos = (0, 0);
    let mut tail_positions = std::collections::HashSet::<Pos>::new();
    tail_positions.insert(tail_pos);
    let mut next_head_pos = rope[1];
    for movement in movements {
        for submovement in submovements_from(&movement) {
            (head_pos, next_head_pos) = move_position(&head_pos, &next_head_pos, &submovement);
            let (hx, hy) = head_pos;
            let mut prev_head_pos = (hx, hy);
            rope[0] = head_pos;
            rope[1] = next_head_pos;
            prev_head_pos = head_pos;
            tail_pos = next_head_pos;
            for ix in 2..rope_length {
                let next_tail_pos = rope[ix];
                let new_tail_pos =
                    move_tail_position_with(&next_head_pos, &prev_head_pos, &next_tail_pos);
                tail_pos = new_tail_pos;
                prev_head_pos = next_head_pos;
                next_head_pos = new_tail_pos;
                rope[ix] = new_tail_pos;
            }
            tail_positions.insert(tail_pos);
        }
    }

    visualize_positions(&tail_positions);
    tail_positions.len()
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

    const TEST_CONTENT_2: &str = "\
R 14
U 24
L 13
D 11";

    const TEST_CONTENT_3: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    fn test_content_2() -> String {
        String::from(TEST_CONTENT_2)
    }

    fn test_content_3() -> String {
        String::from(TEST_CONTENT_3)
    }

    #[test]
    fn test_count_tail_positions_rope_length_10() {
        let movements = movements_from_puzzle_input(&test_content());
        let rope_length = 10;
        let n_tail_positions = count_tail_positions_from(&movements, rope_length);
        assert_eq!(n_tail_positions, 1);
    }

    #[test]
    fn test_count_tail_positions_larger_rope_length_10() {
        let movements = movements_from_puzzle_input(&test_content_3());
        let rope_length = 10;
        let n_tail_positions = count_tail_positions_from(&movements, rope_length);
        assert_eq!(n_tail_positions, 36);
    }

    #[test]
    fn test_parse_input_digits_gt_10() {
        let movements = movements_from_puzzle_input(&test_content_2());
        let expected = vec![('R', 14), ('U', 24), ('L', 13), ('D', 11)];
        assert_eq!(Vec::from(&movements[..]), expected);
    }

    #[test]
    fn test_submovements() {
        let right_4 = ('R', 4);
        let submovements = submovements_from(&right_4);
        let expected = vec![('R', 1), ('R', 1), ('R', 1), ('R', 1)];
        assert_eq!(expected, submovements);
        let left_5 = ('L', 5);
        let submovements = submovements_from(&left_5);
        let expected = vec![('L', 1), ('L', 1), ('L', 1), ('L', 1), ('L', 1)];
        assert_eq!(expected, submovements);
    }

    #[test]
    fn test_count_tail_positions_rope_length_1() {
        let movements = movements_from_puzzle_input(&test_content());
        let rope_length = 1;
        let n_tail_positions = count_tail_positions_from(&movements, rope_length);
        assert_eq!(n_tail_positions, 13);
    }

    #[test]
    fn test_move_head_and_tail_submovements() {
        let movements = movements_from_puzzle_input(&test_content());
        let expected_positions = vec![
            // R 4
            vec![
                ((1, 0), (0, 0)),
                ((2, 0), (1, 0)),
                ((3, 0), (2, 0)),
                ((4, 0), (3, 0)),
            ],
            // U 4
            vec![
                ((4, -1), (3, 0)),
                ((4, -2), (4, -1)),
                ((4, -3), (4, -2)),
                ((4, -4), (4, -3)),
            ],
            // L 3
            vec![((3, -4), (4, -3)), ((2, -4), (3, -4)), ((1, -4), (2, -4))],
            // D 1
            vec![((1, -3), (2, -4))],
            // R 4
            vec![
                ((2, -3), (2, -4)),
                ((3, -3), (2, -4)),
                ((4, -3), (3, -3)),
                ((5, -3), (4, -3)),
            ],
            // D 1
            vec![((5, -2), (4, -3))],
            // L 5
            vec![
                ((4, -2), (4, -3)),
                ((3, -2), (4, -3)),
                ((2, -2), (3, -2)),
                ((1, -2), (2, -2)),
                ((0, -2), (1, -2)),
            ],
            // R 2
            vec![((1, -2), (1, -2)), ((2, -2), (1, -2))],
        ];
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        for (movement, expected_pos) in movements.iter().zip(expected_positions.iter()) {
            for (submovement, expected_sub_pos) in
                submovements_from(&movement).iter().zip(expected_pos.iter())
            {
                (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, &submovement);
                let (expected_head_pos, expected_tail_pos) = expected_sub_pos;
                assert_eq!(
                    head_pos.0, expected_head_pos.0,
                    "head position x - movement: {submovement:?} - expected pos: {expected_head_pos:?}"
                );
                assert_eq!(
                    head_pos.1, expected_head_pos.1,
                    "head position y - movement: {submovement:?} - expected pos: {expected_head_pos:?}"
                );
                assert_eq!(
                    tail_pos.0, expected_tail_pos.0,
                    "tail position x - movement: {submovement:?} - expected pos: {expected_tail_pos:?}"
                );
                assert_eq!(
                    tail_pos.1, expected_tail_pos.1,
                    "tail position y - movement: {submovement:?} - expected pos: {expected_tail_pos:?}"
                );
            }
        }
    }

    #[test]
    fn test_move_head_and_tail() {
        let movements = movements_from_puzzle_input(&test_content());
        let expected_positions = vec![
            ((4, 0), (3, 0)),
            ((4, -4), (4, -3)),
            ((1, -4), (2, -4)),
            ((1, -3), (2, -4)),
            ((5, -3), (4, -3)),
            ((5, -2), (4, -3)),
            ((0, -2), (1, -2)),
            ((2, -2), (1, -2)),
        ];
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        for (movement, expected_pos) in movements.iter().zip(expected_positions.iter()) {
            (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, movement);
            let (expected_head_pos, expected_tail_pos) = expected_pos;
            assert_eq!(
                head_pos.0, expected_head_pos.0,
                "head position x - movement: {movement:?}"
            );
            assert_eq!(
                head_pos.1, expected_head_pos.1,
                "head position y - movement: {movement:?}"
            );
            assert_eq!(
                tail_pos.0, expected_tail_pos.0,
                "tail position x - movement: {movement:?}"
            );
            assert_eq!(
                tail_pos.1, expected_tail_pos.1,
                "tail position y - movement: {movement:?}"
            );
        }
    }

    #[test]
    fn test_move_tail_after_move_head_right() {
        let movement: Movement = ('R', 1);
        let expected_pos = ((1, 0), (0, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, expected_tail_pos) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
        assert_eq!(tail_pos.0, expected_tail_pos.0);
        assert_eq!(tail_pos.1, expected_tail_pos.1);
    }

    #[test]
    fn test_move_tail_after_move_head_right_multiple() {
        let movement: Movement = ('R', 4);
        let expected_pos = ((4, 0), (3, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, expected_tail_pos) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
        assert_eq!(tail_pos.0, expected_tail_pos.0);
        assert_eq!(tail_pos.1, expected_tail_pos.1);
    }

    #[test]
    fn test_move_head_right() {
        let movement: Movement = ('R', 4);
        let expected_pos = ((4, 0), (0, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, _) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, _) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
    }

    #[test]
    fn test_move_head_up() {
        let movement: Movement = ('U', 4);
        let expected_pos = ((0, -4), (0, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, _) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, _) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
    }

    #[test]
    fn test_move_head_left() {
        let movement: Movement = ('L', 4);
        let expected_pos = ((-4, 0), (0, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, _) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, _) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
    }

    #[test]
    fn test_move_head_down() {
        let movement: Movement = ('D', 4);
        let expected_pos = ((0, 4), (0, 0));
        let head_pos = (0, 0);
        let tail_pos = (0, 0);
        let (head_pos, _) = move_position(&head_pos, &tail_pos, &movement);
        let (expected_head_pos, _) = expected_pos;
        assert_eq!(head_pos.0, expected_head_pos.0);
        assert_eq!(head_pos.1, expected_head_pos.1);
    }

    #[test]
    fn test_parse_input() {
        let movements = movements_from_puzzle_input(&test_content());
        let expected = vec![('R', 4), ('U', 4), ('L', 3)];
        assert_eq!(Vec::from(&movements[0..3]), expected);
    }
}
