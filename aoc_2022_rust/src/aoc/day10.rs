use crate::aoc::input;

pub fn run() {
    println!("Day 10");
    let puzzle_input = input::read_day(10);
    let instructions = instructions_from_puzzle_input(&puzzle_input);
    let solution_part_1 = part_1(&instructions);
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

fn part_1(instructions: &Vec<Inst>) -> SignalStrength {
    let mut circuit = ClockCircuit::new();
    let cycle_filter = |tick| -> bool { tick == 20 || (tick - 20) % 40 == 0 };

    circuit.calc_signal_strength(&instructions, cycle_filter)
}


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
    let tokens = line.trim().split(" ").collect::<Vec<&str>>();
    let inst = match tokens[0] {
        "noop" => Inst(Op::Noop, None),
        "addx" => Inst(Op::Addx, Some(tokens[1].parse::<V>().unwrap())),
        _ => panic!("Unsupported Instruction: {tokens:?}"),
    };

    inst
}

fn instructions_from_puzzle_input(puzzle_input: &String) -> Instructions {
    puzzle_input
        .trim()
        .split('\n')
        .map(instruction_from)
        .collect::<Instructions>()
}

type Tick = i64;
type PlannedInst = std::collections::HashMap<Tick, Vec<Inst>>;
type SignalStrength = V;

#[derive(Debug)]
struct ClockCircuit {
    tick: Tick,
    reg_x: V,
    signal_strength: SignalStrength,
}

impl ClockCircuit {
    fn new() -> Self {
        Self {
            reg_x: 1,
            tick: 0,
            signal_strength: 0,
        }
    }

    fn run(&mut self, instructions: &Vec<Inst>) -> Vec<ClockCircuit> {
        let mut states = Vec::<ClockCircuit>::new();
        for instruction in instructions {
            match instruction.0 {
                Op::Addx => {
                    self.tick += 1;
                    states.push(self.to_state());
                    self.tick += 1;
                    states.push(self.to_state());
                    self.reg_x += instruction.1.unwrap();
                }
                Op::Noop => {
                    self.tick += 1;
                    states.push(self.to_state());
                }
            }
        }

        states
    }

    fn signal_strength(&self) -> SignalStrength {
        (self.tick as V) * self.reg_x
    }

    fn to_state(&self) -> ClockCircuit {
        ClockCircuit {
            tick: self.tick,
            reg_x: self.reg_x,
            signal_strength: self.signal_strength(),
        }
    }

    fn calc_signal_strength(
        &mut self,
        instructions: &Vec<Inst>,
        cycle_filter: fn(Tick) -> bool,
    ) -> SignalStrength {
        let mut signal_strength = 0;
        for state in self.run(&instructions) {
            if cycle_filter(state.tick) {
                signal_strength += state.signal_strength;
            }
        }

        signal_strength
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
    fn test_signal_strength_for_cycles() {
        let instructions = instructions_from_puzzle_input(&test_content_2());
        let mut circuit = ClockCircuit::new();
        let cycle_filter = |tick| -> bool { tick == 20 || (tick - 20) % 40 == 0 };

        let signal_strength = circuit.calc_signal_strength(&instructions, cycle_filter);

        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn test_larger_clock_program() {
        let instructions = instructions_from_puzzle_input(&test_content_2());
        let mut circuit = ClockCircuit::new();
        let mut signal_strength: SignalStrength = 0;
        for state in circuit.run(&instructions) {
            if state.tick == 20 || (state.tick - 20) % 40 == 0 {
                println!("{state:?}");
                signal_strength += state.signal_strength;
            }
        }
        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn test_simple_clock_program() {
        let instructions = instructions_from_puzzle_input(&test_content());
        let mut circuit = ClockCircuit::new();
        let _ = circuit.run(&instructions);
        assert_eq!(circuit.reg_x, -1);
        assert_eq!(circuit.tick, 5);
    }

    #[test]
    fn test_noop_instruction() {
        let mut circuit = ClockCircuit::new();
        circuit.run(&vec![Inst::noop()]);
        assert_eq!(circuit.tick, 1);
        assert_eq!(circuit.reg_x, 1);
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
