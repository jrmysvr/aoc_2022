use crate::aoc::input;

pub fn run() {
    println!("Day 1");
    let content = input::read_day(1);
    let food_calories = parse_input(content);
    let most_calories = calc_most_calories(&food_calories);
    let calories_of_top_3_most = calc_most_calories_from_n_elves(&food_calories, 3);
    println!("Part 1: {most_calories:?}");
    println!("Part 2: {calories_of_top_3_most:?}");
}

type Calories = u32;

fn parse_input(content: String) -> Vec<Vec<Calories>> {
    content
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|ss| ss.parse::<Calories>().unwrap())
                .collect::<Vec<Calories>>()
        })
        .collect()
}

fn calc_most_calories(food_calories: &Vec<Vec<Calories>>) -> Calories {
    calc_most_calories_from_n_elves(food_calories, 1)
}

fn calc_most_calories_from_n_elves(food_calories: &Vec<Vec<Calories>>, n_elves: u32) -> Calories {
    let mut calorie_sums = food_calories
        .into_iter()
        .map(|foods| foods.iter().sum::<Calories>())
        .collect::<Vec<Calories>>();

    // Sort in reverse order a < b
    calorie_sums.sort_by(|a, b| b.cmp(a));
    calorie_sums
        .into_iter()
        .take(n_elves as usize)
        .sum::<Calories>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "1000\n\
                                2000\n\
                                3000\n\n\
                                4000\n\n\
                                5000\n\
                                6000\n\n\
                                7000\n\
                                8000\n\
                                9000\n\n\
                                10000";

    fn test_content() -> String {
        String::from(TEST_CONTENT)
    }

    #[test]
    fn test_parse_input() {
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(expected, parse_input(test_content()));
    }

    #[test]
    fn test_calculate_most_calories() {
        let food_calories = parse_input(test_content());
        assert_eq!(24000, calc_most_calories(&food_calories));
    }

    #[test]
    fn test_calculate_sum_of_top_three_most_calories() {
        let food_calories = parse_input(test_content());
        assert_eq!(45000, calc_most_calories_from_n_elves(&food_calories, 3));
    }
}
