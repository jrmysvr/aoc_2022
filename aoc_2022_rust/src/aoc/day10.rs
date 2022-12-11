use crate::aoc::input;

pub fn run() {
    println!("Day 10");
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Noop,
    Addx
}


type V = i64;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Inst(Op, Option<V>);

impl Inst {
    fn noop() -> Inst {
        Inst(Op::Noop, None)
    }

    fn addx(v: V) -> Inst {
        Inst(Op::Addx, Some(v))
    }
}
type Instructions = Vec<Inst>;

fn instruction_from(line: &str) -> Inst {
    let tokens = line.split(" ").collect::<Vec<&str>>();
    let inst = match tokens[0] {
        "noop" => Inst(Op::Noop, None),
        "addx" => Inst(Op::Addx, Some(tokens[1].parse::<V>().unwrap())),
        _ => panic!("Unsupport Instruction"),
    };

    inst
}

fn instructions_from_puzzle_input(puzzle_input: &String) -> Instructions {
    puzzle_input.split('\n').map(instruction_from).collect::<Instructions>()
}

type Tick = usize;

type PlannedInst = std::collections::HashMap::<Tick, Vec<Inst>>;

#[derive(Debug)]
struct ClockCircuit {
    reg_x: V,
    tick: Tick,
    planned_inst: PlannedInst, 
}

impl ClockCircuit {
    fn new() -> Self {
        Self {
            reg_x: 1,
            tick: 0,
            planned_inst: PlannedInst::new(),
        }
    }

    fn run(&mut self, inst: &Inst) {
        self.tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
noop
addx 3
addx -5";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_simple_clock_program() {
        let instructions = instructions_from_puzzle_input(&test_content());
        let mut clock = ClockCircuit::new();
        for inst in instructions {
            clock.run(&inst);
        }
    }

    #[test]
    fn test_noop_instruction() {
        let mut clock = ClockCircuit::new();
        clock.run(&Inst::noop());
        assert_eq!(clock.tick, 1);
        assert_eq!(clock.reg_x, 1);
    }

    #[test]
    fn test_parse_input() {
        let instructions = instructions_from_puzzle_input(&test_content());
        let expected = vec![
            Inst::noop(),
            Inst::addx(3),
            Inst::addx(-5),
            // Inst(Op::Noop, None),
            // Inst(Op::Addx, Some(3)),
            // Inst(Op::Addx, Some(-5)),
        ];
        for (inst, expt) in instructions.iter().zip(expected.iter()) {
            assert_eq!(inst, expt);
        }

    }
}
