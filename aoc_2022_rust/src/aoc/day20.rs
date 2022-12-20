use crate::aoc::input;

pub fn run() {
    println!("Day 20");
    let puzzle_input = input::read_day(20);
    let encrypted = encrypted_file_from_puzzle_input(&puzzle_input);
    let solution_part_1 = part_1(&encrypted);
    let solution_part_2 = "";
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

fn part_1(encrypted: &Nums) -> Num {
    let mixed = mix_nums(encrypted);

    find_nth_num_in(&mixed, 1000, 0)
        + find_nth_num_in(&mixed, 2000, 0)
        + find_nth_num_in(&mixed, 3000, 0)
}

type Num = i64;
type Nums = Vec<Num>;
fn encrypted_file_from_puzzle_input(puzzle_input: &String) -> Nums {
    puzzle_input
        .trim()
        .split('\n')
        .map(|line| line.parse::<Num>().unwrap())
        .collect::<Nums>()
}

fn find_in(encrypted: &Nums, num: Num) -> usize {
    for ix in 0..encrypted.len() {
        if encrypted[ix] == num {
            return ix;
        }
    }

    panic!("Couldn't find `{num}`!");
}

fn calc_new_index(current_index: usize, num: Num, encrypted_length: usize) -> usize {
    let dx = num % (encrypted_length as Num);
    let ix = current_index as Num;
    let len = encrypted_length as Num;
    let new_ix = ix + dx;

    if new_ix <= 0 {
        (new_ix + len - 1) as usize
    } else if new_ix > len - 1 {
        (new_ix - len + 1) as usize
    } else {
        new_ix as usize
    }
}

// Assume the values in encrypted are unique
fn move_num_in(encrypted: &Nums, num: Num) -> Nums {
    let ix = find_in(encrypted, num);
    let new_ix = calc_new_index(ix, num, encrypted.len());
    let mut temp = encrypted.clone();
    temp.remove(ix);
    let (left, right) = temp.split_at(new_ix);
    vec![left, right].join(&num)
}

fn mix_nums(encrypted: &Nums) -> Nums {
    let mut mixed = encrypted.clone();
    for num in encrypted {
        mixed = move_num_in(&mixed, *num);
    }

    mixed
}

fn find_nth_num_in(encrypted: &Nums, nth: usize, after: Num) -> Num {
    let after_ix = find_in(encrypted, after);
    let ix = (after_ix + nth) % encrypted.len();
    encrypted[ix]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "\
1
2
-3
3
-2
0
4";
    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_part_1() {
        let encrypted = encrypted_file_from_puzzle_input(&test_content());
        let expected = 3;
        assert_eq!(part_1(&encrypted), expected);
    }

    #[test]
    fn find_nth_num() {
        let encrypted = encrypted_file_from_puzzle_input(&test_content());
        let mixed = mix_nums(&encrypted);
        assert_eq!(find_nth_num_in(&mixed, 1000, 0), 4);
        assert_eq!(find_nth_num_in(&mixed, 2000, 0), -3);
        assert_eq!(find_nth_num_in(&mixed, 3000, 0), 2);
    }

    #[test]
    fn test_mixing() {
        let encrypted = encrypted_file_from_puzzle_input(&test_content());
        let mixed = mix_nums(&encrypted);
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(mixed, expected);
    }

    #[test]
    fn test_move_num_forward() {
        let encrypted = vec![1, 2, 3];
        let num = 1;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![2, 1, 3];
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_move_num_forward_wrapped() {
        let encrypted = vec![1, 2, -3, 0, 3, 4, -2];
        let num = 4;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_move_num_backward() {
        let encrypted = vec![1, 2, -1];
        let num = -1;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![1, -1, 2];
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_move_num_backward_wrapped() {
        let encrypted = vec![1, -3, 2, 3, -2, 0, 4];
        let num = -3;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![1, 2, 3, -2, -3, 0, 4];
        assert_eq!(moved, expected);

        let encrypted = vec![1, 2, -2, -3, 0, 3, 4];
        let num = -2;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![1, 2, -3, 0, 3, 4, -2];
        assert_eq!(moved, expected);

        let encrypted = vec![4, -2, 5, 6, 7, 8, 9];
        let num = -2;
        let moved = move_num_in(&encrypted, num);
        let expected = vec![4, 5, 6, 7, 8, -2, 9];
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_calc_new_index_forward() {
        assert_eq!(calc_new_index(0, 1, 3), 1);
        assert_eq!(calc_new_index(0, 2, 4), 2);
    }

    #[test]
    fn test_calc_new_index_forward_wrapped() {
        assert_eq!(calc_new_index(2, 1, 3), 1);
        assert_eq!(calc_new_index(2, 2, 4), 1);
    }

    #[test]
    fn test_calc_new_index_backward() {
        assert_eq!(calc_new_index(2, -1, 3), 1);
        assert_eq!(calc_new_index(3, -2, 4), 1);
    }

    #[test]
    fn test_calc_new_index_backward_wrapped() {
        assert_eq!(calc_new_index(1, -3, 7), 4);
    }

    #[test]
    fn test_parse_input() {
        let encrypted = encrypted_file_from_puzzle_input(&test_content());
        let expected = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(encrypted, expected);
    }
}
