use crate::aoc::input;

pub fn run() {
    println!("Day 9");
    let puzzle_input = input::read_day(9);
    let movements = movements_from_puzzle_input(&puzzle_input);
    let solution_part_1 = count_tail_positions_from(&movements);
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
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .nth(0)
        .unwrap()
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
    if (tx - hx).abs() == 1 && (ty - hy).abs() == 1 {
        return *tail_pos;
    }

    *prev_head_pos
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

fn count_tail_positions_from(movements: &Movements) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_positions = std::collections::HashSet::<Pos>::new();
    for movement in movements {
        let submovements = submovements_from(&movement);
        for submovement in submovements {
            (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, &submovement);
            tail_positions.insert(tail_pos);
        }
    }

    println!("{tail_positions:?}");
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
    fn test_content() -> String {
        String::from(TEST_CONTENT)
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
    fn test_count_tail_positions() {
        let movements = movements_from_puzzle_input(&test_content());
        let n_tail_positions = count_tail_positions_from(&movements);
        assert_eq!(n_tail_positions, 13);
    }

    #[test]
    fn test_move_head_and_tail_submovements() {
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
        for movement in movements {
            for (submovement, expected_pos) in
                submovements_from(&movement).iter().zip(expected_positions.iter())
            {
                (head_pos, tail_pos) = move_position(&head_pos, &tail_pos, &submovement);
                let (expected_head_pos, expected_tail_pos) = expected_pos;
                assert_eq!(head_pos.0, expected_head_pos.0);
                assert_eq!(head_pos.1, expected_head_pos.1);
                assert_eq!(tail_pos.0, expected_tail_pos.0);
                assert_eq!(tail_pos.1, expected_tail_pos.1);
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
