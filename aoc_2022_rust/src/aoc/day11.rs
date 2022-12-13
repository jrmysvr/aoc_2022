use crate::aoc::input;

pub fn run() {
    println!("Day _");
    //let _ = input::read_day(_);
    //let _ = parse_input(_);
    let solution_part_1 = "";
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type WorryLevel = u64;
type MonkeyId = u64;

#[derive(Debug, PartialEq)]
struct Monkey {
    id: MonkeyId,
    starting_items: Vec<WorryLevel>,
    operation: fn(WorryLevel) -> WorryLevel,
    test: fn(WorryLevel) -> MonkeyId,
}

fn monkey_from(lines: Vec<&str>) -> Monkey {

    Monkey {
        id: 0,
        starting_items: Vec::<WorryLevel>::new(),
        operation: |worry| -> WorryLevel { 0 },
        test: |worry| -> MonkeyId { 1 },
    }
}

type Monkeys = Vec<Monkey>;
fn monkeys_from_puzzle_input(puzzle_input: &String) -> Monkeys {
    puzzle_input
        .trim()
        .split("\n\n")
        .map(|block| block.split('\n').collect::<Vec<&str>>())
        .map(monkey_from)
        .collect::<Monkeys>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_parse_input() {
        let monkeys = monkeys_from_puzzle_input(&test_content());
        let expected_monkeys = vec![Monkey {
            id: 0,
            starting_items: vec![79, 98],
            operation: |old_worry| -> WorryLevel { old_worry * 19 },
            test: |worry| -> MonkeyId {
                if worry % 23 == 0 {
                    0
                } else {
                    1
                }
            },
        }];
        for (monkey, expected) in monkeys.iter().zip(expected_monkeys.iter()) {
            assert_eq!(monkey, expected);
        }
    }
}
