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
type Count = usize;

fn monkey_id_from(line: &str) -> MonkeyId {
    line.trim()
        .trim_start_matches("Monkey ")
        .trim_end_matches(":")
        .parse::<MonkeyId>()
        .unwrap()
}

fn starting_items_from(line: &str) -> Vec<WorryLevel> {
    line.trim()
        .trim_start_matches("Starting items: ")
        .split(", ")
        .map(|item| item.parse::<WorryLevel>().unwrap())
        .collect::<Vec<WorryLevel>>()
}

#[derive(Debug, PartialEq)]
type Op = impl Fn(WorryLevel) -> WorryLevel;

fn square_worry() -> Op {
    move |worry| worry * worry
}

fn double_worry() -> Op {
    move |worry| worry + worry
}

fn add_to_worry(value: WorryLevel) -> Op {
    move |worry| worry + value
}

fn multiply_worry(value: WorryLevel) -> Op {
    move |worry| worry * value
}

struct Operation {
    op: Op,
    op_str: String,
    rval_str: String,
}

impl Operation {
    fn new(op_str: &str, rval_str: &str) -> Self {
        let op = match op_str {
            "+" => match rval_str {
                "old" => double_worry(),
                _ => add_to_worry(rval_str.parse::<WorryLevel>().unwrap()),
            },
            "*" => match rval_str {
                "old" => square_worry(),
                _ => multiply_worry(rval_str.parse::<WorryLevel>().unwrap()),
            },
            _ => panic!("Unsupported operation char: {op_str}"),
        };

        Self { op: op, op_str: op_str.to_string(), rval_str: rval_str }
    }
}

fn operation_from(line: &str) -> Operation {
    /*
    let (op_str, rval_str, _) = line
        .trim()
        .trim_start_matches("Operation: new = old ")
        .split(' ')
        .collect::<Vec<&str>>();
        */
    let op_str = "*";
    let rval_str = "old";
    Operation::new(op_str, rval_str)
}

type Test = fn(WorryLevel) -> MonkeyId;

struct Monkey {
    id: MonkeyId,
    starting_items: Vec<WorryLevel>,
    operation: Operation,
    test: Test,
    inspection_count: Count,
}

fn monkey_from(lines: Vec<&str>) -> Monkey {
    Monkey {
        id: monkey_id_from(lines[0]),
        starting_items: starting_items_from(lines[1]),
        operation: operation_from(lines[2]),
        test: |worry| -> MonkeyId { 1 },
        inspection_count: 0,
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
    fn test_parse_operation() {
        let example_operation = "Operation: new = old * 19";
        let cleaned_operation = example_operation
            .trim()
            .trim_start_matches("Operation: new =")
            .replace(" ", "")
            .as_str();

        let op_str = "+";
        assert_eq!(op_str, "*");

        let lval_str = "new";
        assert_eq!(lval_str, "old");
    }

    #[test]
    fn test_parse_starting_items() {
        let starting_items = starting_items_from("Starting items: 74");
        let expected = vec![74];
        assert_eq!(starting_items, expected);

        let starting_items = starting_items_from("Starting items: 79, 98");
        let expected = vec![79, 98];
        assert_eq!(starting_items, expected);
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
