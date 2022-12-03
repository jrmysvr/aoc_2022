use crate::aoc::input;

pub fn run() {
    println!("Day 2");
    let content = input::read_day(2);
    let rounds = parse_input(content);

    let solution_part_1 = calc_score_for_rounds(&rounds);
    let solution_part_2 = calc_score_for_rounds_part_2(&rounds);
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

type Letter = String;
type Round = (Shape, Shape);
type Score = u32;

fn letter_to_shape(letter: Letter) -> Shape {
    let letter = &letter[..];
    match letter {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Aaaaaah"),
    }
}

fn parse_input(content: String) -> Vec<Round> {
    content
        .trim()
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(String::from)
                .map(letter_to_shape)
                .collect::<Vec<Shape>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect::<Vec<Round>>()
}

fn calc_score_for_round(round: Round) -> Score {
    let (lose, draw, win) = (0, 3, 6);
    let (rock, paper, scissors) = (1, 2, 3);
    match round {
        (Shape::Rock, Shape::Rock) => rock + draw,
        (Shape::Paper, Shape::Rock) => rock + lose,
        (Shape::Scissors, Shape::Rock) => rock + win,
        (Shape::Rock, Shape::Paper) => paper + win,
        (Shape::Paper, Shape::Paper) => paper + draw,
        (Shape::Scissors, Shape::Paper) => paper + lose,
        (Shape::Rock, Shape::Scissors) => scissors + lose,
        (Shape::Paper, Shape::Scissors) => scissors + win,
        (Shape::Scissors, Shape::Scissors) => scissors + draw,
    }
}

fn calc_score_for_rounds(rounds: &Vec<Round>) -> Score {
    rounds
        .into_iter()
        .fold(0, |acc, round| acc + calc_score_for_round(*round))
}

fn calc_score_for_round_part_2(round: Round) -> Score {
    let (lose, draw, win) = (0, 3, 6);
    let (rock, paper, scissors) = (1, 2, 3);
    match round {
        (Shape::Rock, Shape::Rock) => scissors + lose,
        (Shape::Paper, Shape::Rock) => rock + lose,
        (Shape::Scissors, Shape::Rock) => paper + lose,
        (Shape::Rock, Shape::Paper) => rock + draw,
        (Shape::Paper, Shape::Paper) => paper + draw,
        (Shape::Scissors, Shape::Paper) => scissors + draw,
        (Shape::Rock, Shape::Scissors) => paper + win,
        (Shape::Paper, Shape::Scissors) => scissors + win,
        (Shape::Scissors, Shape::Scissors) => rock + win,
    }
}

fn calc_score_for_rounds_part_2(rounds: &Vec<Round>) -> Score {
    rounds
        .into_iter()
        .fold(0, |acc, round| acc + calc_score_for_round_part_2(*round))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "A Y\n\
                                B X\n\
                                C Z";

    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_parse_input() {
        let expected_rounds = vec![("A", "Y"), ("B", "X"), ("C", "Z")]
            .into_iter()
            .map(|line| (String::from(line.0), String::from(line.1)))
            .map(|round| (letter_to_shape(round.0), letter_to_shape(round.1)))
            .collect::<Vec<Round>>();
        for (expected, actual) in expected_rounds.into_iter().zip(parse_input(test_content())) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_score_for_round_rock_vs_paper() {
        let round = (Shape::Rock, Shape::Paper);
        let score = calc_score_for_round(round);
        let expected = 8;
        assert_eq!(expected, score);
    }

    #[test]
    fn test_score_rounds() {
        let rounds = parse_input(test_content());
        let score = calc_score_for_rounds(&rounds);
        let expected = 15;
        assert_eq!(expected, score);
    }

    #[test]
    fn test_score_for_round_rock_vs_paper_part_2() {
        let round = (Shape::Rock, Shape::Scissors); // Win
        let score = calc_score_for_round_part_2(round);
        let expected = 8;
        assert_eq!(expected, score);
    }
}
