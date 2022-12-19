use crate::aoc::input;

pub fn run() {
    println!("Day 14");
    // let puzzle_input = input::read_day(14);
    // let _ = parse_input(_);
    let puzzle_input = String::from(
        "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    let solution_part_2 = "";
    println!("Part 1: {}", part_1(&puzzle_input));
    println!("Part 2: {}", solution_part_2);
}

fn part_1(puzzle_input: &String) -> usize {
    visualize_lines(multiple_lines_from_puzzle_input(puzzle_input));
    0
}

type Unit = i64;
type Point = (Unit, Unit);
type Points = Vec<Point>;
type Line = (Point, Point);
type Lines = Vec<Line>;

fn units_to_point(v: Vec<Unit>) -> Point {
    (v[0], v[1])
}

fn points_from_puzzle_input(puzzle_input: &String) -> Vec<Points> {
    puzzle_input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    units_to_point(
                        point
                            .split(',')
                            .map(|unit| unit.parse::<Unit>().unwrap())
                            .collect::<Vec<Unit>>(),
                    )
                })
                .collect::<Points>()
        })
        .collect::<Vec<Points>>()
}

fn multiple_lines_from_puzzle_input(puzzle_input: &String) -> Vec<Lines> {
    let points = points_from_puzzle_input(puzzle_input);
    points
        .iter()
        .map(|sub_points| {
            sub_points[..]
                .windows(2)
                .map(|window| (window[0], window[1]))
                .collect::<Lines>()
        })
        .collect::<Vec<Lines>>()
}

fn points_in(line: Line) -> Vec<Point> {
    let (start, end) = line;
    let (startx, starty) = start;
    let (endx, endy) = end;
    let min_y = std::cmp::min(starty, endy);
    let min_x = std::cmp::min(startx, endx);
    let max_y = std::cmp::max(starty, endy);
    let max_x = std::cmp::max(startx, endx);

    let mut points = Vec::<Point>::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            points.push((x, y));
        }
    }

    points
}

fn visualize_lines(multi_lines: Vec<Lines>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = Unit::MAX;
    let mut min_y = Unit::MAX;

    for lines in multi_lines.iter() {
        for line in lines {
            let (a, b) = line;
            let (ax, ay) = a;
            let (bx, by) = b;
            println!("{a:?}");
            println!("{b:?}");
            max_x = std::cmp::max(max_x, *ax);
            max_x = std::cmp::max(max_x, *bx);
            max_y = std::cmp::max(max_y, *ay);
            max_y = std::cmp::max(max_y, *by);
            min_x = std::cmp::min(min_x, *ax);
            min_x = std::cmp::min(min_x, *bx);
            min_y = std::cmp::min(min_y, *ay);
            min_y = std::cmp::min(min_y, *by);
        }
    }

    min_y = 0;

    let mut grid = Vec::<Vec<char>>::new();
    for ry in 0..=(max_y - min_y) {
        let mut row = Vec::<char>::new();
        for rx in 0..=(max_x - min_x) {
            let ch = '.';
            row.push(ch);
        }
        grid.push(row);
    }

    for lines in multi_lines {
        for line in lines {
            for point in points_in(line) {
                let (x, y) = point;
                let x = (x - min_x) as usize;
                let y = (y - min_y) as usize;
                grid[y][x] = '#';
            }
        }
    }

    for row in grid {
        for ch in row {
            print!("{ch}");
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_points_to_lines() {
        let multiple_lines = multiple_lines_from_puzzle_input(&test_content());
        let expected_multiple_lines = vec![
            vec![((498, 4), (498, 6)), ((498, 6), (496, 6))],
            vec![
                ((503, 4), (502, 4)),
                ((502, 4), (502, 9)),
                ((502, 9), (494, 9)),
            ],
        ];
        for (lines, expected_lines) in multiple_lines.iter().zip(expected_multiple_lines.iter()) {
            for (line, expected) in lines.iter().zip(expected_lines.iter()) {
                let (aa, ab) = line;
                let (ea, eb) = expected;
                assert_eq!(aa, ea);
                assert_eq!(ab, eb);
            }
        }
    }

    #[test]
    fn test_parse_input() {
        let points = points_from_puzzle_input(&test_content());
        let expected_points = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];
        for (point, expected) in points.iter().zip(expected_points.iter()) {
            assert_eq!(point, expected);
        }
    }
}
