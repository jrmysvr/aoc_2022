use crate::aoc::input;

pub fn run() {
    println!("Day 8");
    let puzzle_input = input::read_day(8);
    let tree_grid = trees_from_puzzle_input(&puzzle_input);
    let solution_part_1 = count_visible_trees_in(&tree_grid);
    let solution_part_2 = calc_max_scenic_score_in(&tree_grid);
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type Tree = u32;
type TreeLine = Vec<Tree>;
type TreeGrid = Vec<TreeLine>;

fn trees_from_puzzle_input(puzzle_input: &String) -> TreeGrid {
    puzzle_input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<TreeLine>()
        })
        .collect::<TreeGrid>()
}

fn tree_row_from(tree_grid: &TreeGrid, row_ix: usize) -> TreeLine {
    tree_grid[row_ix].clone()
}

fn tree_col_from(tree_grid: &TreeGrid, col_ix: usize) -> TreeLine {
    tree_grid
        .iter()
        .map(|row| *row.iter().nth(col_ix).unwrap())
        .collect::<TreeLine>()
}

fn check_visible_from_end_in(tree_line: &TreeLine, ix: usize) -> bool {
    let height = tree_line[ix];
    let visible_from_one_end = tree_line[0..ix].iter().max().unwrap() < &height;
    let visible_from_other_end = tree_line[ix + 1..tree_line.len()].iter().max().unwrap() < &height;
    visible_from_one_end || visible_from_other_end
}

fn check_visible_from_edge_in(tree_grid: &TreeGrid, pos: (usize, usize)) -> bool {
    let (r, c) = pos;
    let tree_row = tree_row_from(tree_grid, r);
    let tree_col = tree_col_from(tree_grid, c);
    check_visible_from_end_in(&tree_row, c) || check_visible_from_end_in(&tree_col, r)
}

fn count_visible_trees_in(tree_grid: &TreeGrid) -> usize {
    let n_rows = tree_grid.len();
    let n_cols = tree_grid.iter().nth(1).unwrap().len();
    let edge_count = 2 * n_cols + 2 * (n_rows - 2);
    let mut visible_count = edge_count;

    for rx in 1..n_rows - 1 {
        for cx in 1..n_cols - 1 {
            if check_visible_from_edge_in(tree_grid, (rx, cx)) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

type Score = usize;

fn calc_scenic_score_in_line(tree_line: &TreeLine, ix: usize) -> Score {
    let height = tree_line[ix];
    let mut one_score = 0;
    for jx in (0..ix).rev() {
        one_score += 1;
        if tree_line[jx] >= height {
            break;
        }
    }
    let mut other_score = 0;
    for jx in (ix + 1)..tree_line.len() {
        other_score += 1;
        if tree_line[jx] >= height {
            break;
        }
    }

    one_score * other_score
}

fn calc_scenic_score_in(tree_grid: &TreeGrid, pos: (usize, usize)) -> Score {
    let (r, c) = pos;
    let tree_row = tree_row_from(tree_grid, r);
    let tree_col = tree_col_from(tree_grid, c);
    calc_scenic_score_in_line(&tree_row, c) * calc_scenic_score_in_line(&tree_col, r)
}

fn calc_max_scenic_score_in(tree_grid: &TreeGrid) -> Score {
    let n_rows = tree_grid.len();
    let n_cols = tree_grid.iter().nth(1).unwrap().len();
    let mut score = 0;
    for rx in 1..n_rows - 1 {
        for cx in 1..n_cols - 1 {
            score = std::cmp::max(calc_scenic_score_in(tree_grid, (rx, cx)), score);
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
30373
25512
65332
33549
35390";

    const TEST_CONTENT_2: &str = "\
30373
25512
65336
33549
35390";

    const TEST_CONTENT_3: &str = "\
3037325512
6533633549
3539030373
2551265336
3354935390";

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
    fn test_parse_input() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(tree_grid, expected);
    }

    #[test]
    fn test_parse_input_2() {
        let tree_grid = trees_from_puzzle_input(&test_content_2());
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 6],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(tree_grid, expected);
    }

    #[test]
    fn test_tree_rows() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(vec![3, 0, 3, 7, 3], tree_row_from(&tree_grid, 0));
        assert_eq!(vec![2, 5, 5, 1, 2], tree_row_from(&tree_grid, 1));
    }

    #[test]
    fn test_tree_cols() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(vec![3, 2, 6, 3, 3], tree_col_from(&tree_grid, 0));
        assert_eq!(vec![0, 5, 5, 3, 5], tree_col_from(&tree_grid, 1));
    }

    #[test]
    fn test_tree_rows_non_square() {
        let tree_grid = trees_from_puzzle_input(&test_content_3());
        assert_eq!(
            vec![3, 0, 3, 7, 3, 2, 5, 5, 1, 2],
            tree_row_from(&tree_grid, 0)
        );
        assert_eq!(
            vec![6, 5, 3, 3, 6, 3, 3, 5, 4, 9],
            tree_row_from(&tree_grid, 1)
        );
    }

    #[test]
    fn test_tree_cols_non_square() {
        let tree_grid = trees_from_puzzle_input(&test_content_3());
        assert_eq!(vec![3, 6, 3, 2, 3], tree_col_from(&tree_grid, 0));
        assert_eq!(vec![0, 5, 5, 5, 3], tree_col_from(&tree_grid, 1));
    }

    #[test]
    fn test_visible_from_end_rows() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        let tree_line = tree_row_from(&tree_grid, 1);
        assert!(check_visible_from_end_in(&tree_line, 1));
        assert!(check_visible_from_end_in(&tree_line, 2));
        assert!(!check_visible_from_end_in(&tree_line, 3));
        let tree_grid = trees_from_puzzle_input(&test_content());
        let tree_line = tree_row_from(&tree_grid, 2);
        assert!(check_visible_from_end_in(&tree_line, 1));
        assert!(!check_visible_from_end_in(&tree_line, 2));
        assert!(check_visible_from_end_in(&tree_line, 3));
        let tree_grid = trees_from_puzzle_input(&test_content());
        let tree_line = tree_row_from(&tree_grid, 3);
        assert!(!check_visible_from_end_in(&tree_line, 1));
        assert!(check_visible_from_end_in(&tree_line, 2));
        assert!(!check_visible_from_end_in(&tree_line, 3));
    }

    #[test]
    fn test_visible_from_end_cols() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        let tree_line = tree_col_from(&tree_grid, 1);
        assert!(check_visible_from_end_in(&tree_line, 1));
        assert!(!check_visible_from_end_in(&tree_line, 2));
        assert!(!check_visible_from_end_in(&tree_line, 3));
        let tree_line = tree_col_from(&tree_grid, 2);
        assert!(check_visible_from_end_in(&tree_line, 1));
        assert!(!check_visible_from_end_in(&tree_line, 2));
        assert!(check_visible_from_end_in(&tree_line, 3));
        let tree_line = tree_col_from(&tree_grid, 3);
        assert!(!check_visible_from_end_in(&tree_line, 1));
        assert!(!check_visible_from_end_in(&tree_line, 2));
        assert!(!check_visible_from_end_in(&tree_line, 3));
    }

    #[test]
    fn test_visible_from_edge() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert!(check_visible_from_edge_in(&tree_grid, (1, 1)));
        assert!(check_visible_from_edge_in(&tree_grid, (1, 2)));
        assert!(!check_visible_from_edge_in(&tree_grid, (1, 3)));
        assert!(check_visible_from_edge_in(&tree_grid, (2, 1)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 2)));
        assert!(check_visible_from_edge_in(&tree_grid, (2, 3)));
        assert!(!check_visible_from_edge_in(&tree_grid, (3, 1)));
        assert!(check_visible_from_edge_in(&tree_grid, (3, 2)));
        assert!(!check_visible_from_edge_in(&tree_grid, (3, 3)));
    }

    #[test]
    fn test_visible_from_edge_non_square() {
        let tree_grid = trees_from_puzzle_input(&test_content_3());
        assert!(check_visible_from_edge_in(&tree_grid, (1, 1)));
        assert!(!check_visible_from_edge_in(&tree_grid, (1, 2)));
        assert!(!check_visible_from_edge_in(&tree_grid, (1, 3)));
        assert!(check_visible_from_edge_in(&tree_grid, (1, 4)));
        assert!(check_visible_from_edge_in(&tree_grid, (1, 5)));
        assert!(!check_visible_from_edge_in(&tree_grid, (1, 6)));
        assert!(check_visible_from_edge_in(&tree_grid, (1, 7)));
        assert!(check_visible_from_edge_in(&tree_grid, (1, 8)));

        assert!(check_visible_from_edge_in(&tree_grid, (2, 1)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 2)));
        assert!(check_visible_from_edge_in(&tree_grid, (2, 3)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 4)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 5)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 6)));
        assert!(!check_visible_from_edge_in(&tree_grid, (2, 7)));
        assert!(check_visible_from_edge_in(&tree_grid, (2, 8)));
    }

    #[test]
    fn test_visible_from_end_non_square() {
        let tree_grid = trees_from_puzzle_input(&test_content_3());
        let tree_line = tree_col_from(&tree_grid, 1);
        assert!(check_visible_from_end_in(&tree_line, 1));
        let tree_line = tree_row_from(&tree_grid, 1);
        assert!(!check_visible_from_end_in(&tree_line, 1));
    }

    #[test]
    fn test_count_visible_trees() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(count_visible_trees_in(&tree_grid), 21);
    }

    #[test]
    fn test_count_visible_trees_2() {
        let tree_grid = trees_from_puzzle_input(&test_content_2());
        assert_eq!(count_visible_trees_in(&tree_grid), 19);
    }

    #[test]
    fn test_count_visible_trees_3() {
        let tree_grid = trees_from_puzzle_input(&test_content_3());
        assert_eq!(count_visible_trees_in(&tree_grid), 37);
    }

    #[test]
    fn test_scenic_scores_in_lines() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        let tree_row = tree_row_from(&tree_grid, 1);
        assert_eq!(calc_scenic_score_in_line(&tree_row, 2), 2);
        let tree_col = tree_col_from(&tree_grid, 2);
        assert_eq!(calc_scenic_score_in_line(&tree_col, 1), 2);
    }

    #[test]
    fn test_scenic_scores() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(calc_scenic_score_in(&tree_grid, (1, 2)), 4);
        assert_eq!(calc_scenic_score_in(&tree_grid, (3, 2)), 8);
    }

    #[test]
    fn test_max_scenic_score() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(calc_max_scenic_score_in(&tree_grid), 8);
    }
}
