use crate::aoc::input;
use std::collections::HashSet;

pub fn run() {
    println!("Day 3");
    let content = input::read_day(3);
    let rucksacks = parse_input(content);
    let solution_part_1 = part_1(&rucksacks);
    let solution_part_2 = part_2(&rucksacks);
    println!("Part 1: {}", solution_part_1);
    println!("Part 2: {}", solution_part_2);
}

type HalfRucksack = String;
type Rucksack = (HalfRucksack, HalfRucksack);
type Priority = u32;
type Item = char;

fn part_1(rucksacks: &Vec<Rucksack>) -> Priority {
    rucksacks
        .iter()
        .map(find_common_item_in_rucksack)
        .map(item_priority)
        .sum::<Priority>()
}

fn part_2(rucksacks: &Vec<Rucksack>) -> Priority {
    let mut items = Vec::<Item>::new();
    let n_rucksacks = 3;
    for i in (0..rucksacks.len() - 2).step_by(n_rucksacks) {
        let some_rucksacks = Vec::from(&rucksacks[i..i + n_rucksacks]);
        items.push(find_common_item_in_3_rucksacks(&some_rucksacks));
    }

    items.iter().map(|item| item_priority(*item)).sum()
}

fn parse_input(content: String) -> Vec<Rucksack> {
    content
        .trim()
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|half| (HalfRucksack::from(half.0), HalfRucksack::from(half.1)))
        .collect::<Vec<Rucksack>>()
}

fn find_common_item_in_rucksack(rucksack: &Rucksack) -> Item {
    let (a, b) = rucksack;
    *a.chars()
        .collect::<HashSet<Item>>()
        .intersection(&(b.chars().collect::<HashSet<Item>>()))
        .last()
        .unwrap()
}

fn find_common_item_in_3_rucksacks(rucksacks: &Vec<Rucksack>) -> Item {
    let (_a, _aa) = &rucksacks[0];
    let (_b, _bb) = &rucksacks[1];
    let (_c, _cc) = &rucksacks[2];
    let a = _a.clone() + &_aa;
    let b = _b.clone() + &_bb;
    let c = _c.clone() + &_cc;
    let aset = a.chars().collect::<HashSet<Item>>();
    let bset = b.chars().collect::<HashSet<Item>>();
    let cset = c.chars().collect::<HashSet<Item>>();
    let ab = aset.intersection(&bset).collect::<HashSet<&Item>>();
    let bc = bset.intersection(&cset).collect::<HashSet<&Item>>();
    let common_item = **(ab.intersection(&bc).last().unwrap());
    common_item
}

fn item_priority(item: Item) -> Priority {
    if item < 'a' {
        (item as Priority) - ('A' as Priority) + 27
    } else {
        (item as Priority) - ('a' as Priority) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    fn test_content() -> String {
        String::from(TEST_CONTENT).trim().to_string()
    }

    #[test]
    fn test_parse_input() {
        let rucksacks = parse_input(test_content());
        let _expected_rucksacks = vec![("vJrwpWtwJgWr", "hcsFMMfFFhFp")];
        let expected_rucksacks = _expected_rucksacks
            .iter()
            .map(|ruck| (String::from(ruck.0), String::from(ruck.1)));
        for (expected, actual) in expected_rucksacks.into_iter().zip(rucksacks) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_item_priority_lower() {
        for (ch, pr) in ('a'..'z').zip(1..26) {
            assert_eq!(item_priority(ch), pr);
        }
    }

    #[test]
    fn test_item_priority_upper() {
        for (ch, pr) in ('A'..'Z').zip(27..52) {
            assert_eq!(item_priority(ch), pr);
        }
    }

    #[test]
    fn test_find_common_item_in_single_rucksack() {
        let items = vec!['p', 'L', 'P', 'v', 't', 's'];
        for (item, rucksack) in items.into_iter().zip(parse_input(test_content())) {
            assert_eq!(item, find_common_item_in_rucksack(&rucksack));
        }
    }

    #[test]
    fn test_find_common_item_in_rucksacks() {
        let rucksacks = parse_input(test_content());
        assert_eq!(
            'r',
            find_common_item_in_3_rucksacks(&Vec::from(&rucksacks[0..3]))
        );
        assert_eq!(
            'Z',
            find_common_item_in_3_rucksacks(&Vec::from(&rucksacks[3..]))
        );
    }

    #[test]
    fn test_part_1() {
        let rucksacks = parse_input(test_content());
        assert_eq!(157, part_1(&rucksacks));
    }

    #[test]
    fn test_part_2() {
        let rucksacks = parse_input(test_content());
        assert_eq!(70, part_2(&rucksacks));
    }
}
