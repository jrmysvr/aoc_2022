use crate::aoc::input;

pub fn run() {
    println!("Day 8");
    //let _ = input::read_day(_);
    //let _ = parse_input(_);
    let solution_part_1 = "";
    let solution_part_2 = "";
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
    let mut row = TreeLine::new();
    tree_grid[row_ix].clone()
}

fn tree_col_from(tree_grid: &TreeGrid, col_ix: usize) -> TreeLine {
    tree_grid
        .iter()
        .map(|row| *row.iter().nth(col_ix).unwrap())
        .collect::<TreeLine>()
}

fn check_visible_from_edge_in(tree_grid: &TreeGrid, pos: (usize, usize)) -> bool {
    let (r, c) = pos;
    true // TODO!
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

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
30373
25512
65332
33549
35390";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
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
    fn test_tree_rows() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(vec![3, 0, 3, 7, 3], tree_row_from(&tree_grid, 0));
    }

    #[test]
    fn test_tree_cols() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(vec![3, 2, 6, 3, 3], tree_col_from(&tree_grid, 0));
    }

    #[test]
    fn test_visible_from_edge() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert!(check_visible_from_edge_in(&tree_grid, (1, 1)));
    }

    #[test]
    fn test_count_visible_trees() {
        let tree_grid = trees_from_puzzle_input(&test_content());
        assert_eq!(count_visible_trees_in(&tree_grid), 21);
    }
}
