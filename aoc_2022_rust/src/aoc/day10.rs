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
    Addx,
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
    puzzle_input
        .split('\n')
        .map(instruction_from)
        .collect::<Instructions>()
}

type Tick = i64;

type PlannedInst = std::collections::HashMap<Tick, Vec<Inst>>;

#[derive(Debug)]
struct ClockCircuit {
    reg_x: V,
    tick: Tick,
    planned_inst: PlannedInst,
}

type SignalStrength = V;

impl ClockCircuit {
    fn new() -> Self {
        Self {
            reg_x: 1,
            tick: 0,
            planned_inst: PlannedInst::new(),
        }
    }

    fn run(&mut self, inst: &Inst) {
        match inst.0 {
            Op::Addx => {
                self.planned_inst
                    .entry(self.tick + 2)
                    .and_modify(|planned| planned.push(inst.clone()))
                    .or_insert(vec![inst.clone()]);
            }
            Op::Noop => {}
        }

        if let Some(planned) = self.planned_inst.remove(&self.tick) {
            self.reg_x += planned.iter().map(|i| i.1.unwrap()).sum::<V>();
        }
        self.tick += 1;
    }

    fn wait_tick(&mut self) -> bool {
        if self.planned_inst.is_empty() {
            return false;
        }
        if let Some(planned) = self.planned_inst.remove(&self.tick) {
            self.reg_x += planned.iter().map(|i| i.1.unwrap()).sum::<V>();
        }
        self.tick += 1;
        true
    }

    fn signal_strength(&self) -> SignalStrength {
        (self.tick as V) * self.reg_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
noop
addx 3
addx -5";

    const TEST_CONTENT_2: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    fn test_content_2() -> String {
        String::from(TEST_CONTENT_2)
    }

    #[test]
    fn test_larger_clock_program() {
        let instructions = instructions_from_puzzle_input(&test_content_2());
        let mut clock = ClockCircuit::new();
        let mut signal_strength: SignalStrength = 0;
        for inst in instructions {
            clock.run(&inst);
            if clock.tick == 20 || (clock.tick - 20) % 40 == 0 {
                println!("{clock:?}, {}", clock.signal_strength());
                signal_strength += clock.signal_strength();
            }
        }
        while clock.wait_tick() {
            if clock.tick == 20 || (clock.tick - 20) % 40 == 0 {
                println!("{clock:?}, {}", clock.signal_strength());
                signal_strength += clock.signal_strength();
            }
        }

        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn test_simple_clock_program() {
        let instructions = instructions_from_puzzle_input(&test_content());
        let mut clock = ClockCircuit::new();
        for inst in instructions {
            clock.run(&inst);
        }
        while clock.wait_tick() {}

        assert_eq!(clock.reg_x, -1);
        assert_eq!(clock.tick, 5);
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
            // Inst(Op::Noop, None),
            Inst::noop(),
            // Inst(Op::Addx, Some(3)),
            Inst::addx(3),
            // Inst(Op::Addx, Some(-5)),
            Inst::addx(-5),
        ];
        for (inst, expt) in instructions.iter().zip(expected.iter()) {
            assert_eq!(inst, expt);
        }
    }
}
