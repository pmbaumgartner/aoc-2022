use num::integer::lcm;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    starting_items: VecDeque<u64>,
    // Add or Multiply by a number
    operation: Operation,
    divisibility_test: u64,
    // if true: send to monkey id, if false send to other monkey id
    true_branch: u64,
    false_branch: u64,
    // need this for the puzzle
    inspection_count: u64,
}

impl Monkey {
    fn from_hashmap(dict: BTreeMap<String, String>) -> Self {
        let id: u64 = dict["monkey_id"].parse().unwrap();
        let starting_items: VecDeque<u64> = dict["starting_items"]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        // example operation: new = old * 19
        // we need the operation and the digit
        let operation_string = dict["operation"].split(' ').rev().collect::<Vec<&str>>();
        let operation = match operation_string[0..=1] {
            ["old", "*"] => Operation::Square,
            [digit, "+"] => Operation::Add(digit.parse().unwrap()),
            [digit, "*"] => Operation::Multiply(digit.parse().unwrap()),
            _ => panic!("Invalid operation"),
        };
        // capture = 'divisible by 23'
        let divisibility_test: u64 = dict["test"].split(' ').last().unwrap().parse().unwrap();
        // capture = 'throw to monkey 2'
        let true_branch: u64 = dict["true"].split(' ').last().unwrap().parse().unwrap();
        let false_branch: u64 = dict["false"].split(' ').last().unwrap().parse().unwrap();
        Self {
            id,
            starting_items,
            operation,
            divisibility_test,
            true_branch,
            false_branch,
            inspection_count: 0,
        }
    }
    fn calculate_worry(&self, item: u64) -> u64 {
        let post_inspection_value = match &self.operation {
            Operation::Add(value) => item + value,
            Operation::Multiply(value) => item * value,
            Operation::Square => item * item,
        };
        // divide post_inspection_value by 3 and round down to the nearest integer
        let relief_value = (post_inspection_value as f32 / 3.0).floor() as u64;
        relief_value
    }
    fn calculate_worry_no_relief(&self, item: u64, modulo: u64) -> u64 {
        let post_inspection_value = match &self.operation {
            Operation::Add(value) => (item + value) % modulo,
            Operation::Multiply(value) => (item * value) % modulo,
            Operation::Square => (item * item) % modulo,
        };
        post_inspection_value
    }

    // Returns the next monkey id and the item to send to that monkey
    fn inspect_item(&mut self, item: u64) -> (u64, u64) {
        self.inspection_count += 1;
        let new_item = self.calculate_worry(item);
        if new_item % self.divisibility_test == 0 {
            (self.true_branch, new_item)
        } else {
            (self.false_branch, new_item)
        }
    }

    fn inspect_item_no_relief(&mut self, item: u64, modulo: u64) -> (u64, u64) {
        self.inspection_count += 1;
        let new_item = self.calculate_worry_no_relief(item, modulo);
        if new_item % self.divisibility_test == 0 {
            (self.true_branch, new_item)
        } else {
            (self.false_branch, new_item)
        }
    }
}

fn named_captures_to_hashmap(
    re: regex::Regex,
    captures: regex::Captures,
) -> BTreeMap<String, String> {
    let dict: BTreeMap<String, String> = re
        .capture_names()
        .flatten()
        .filter_map(|n| Some((n.to_string(), captures.name(n)?.as_str().to_string())))
        .collect();
    dict
}

fn parse_monkey_data(input: &str) -> BTreeMap<String, String> {
    // In retrospect, a simpler parser would be to split each line on a colon and
    // then take the last element as the string
    // but I guess we get some additional validation here in the case where our regex fails
    let re = Regex::new(r"Monkey (?P<monkey_id>\d):\n  Starting items: (?P<starting_items>[\d, ]+)\n  Operation: (?P<operation>.*)\n  Test: (?P<test>.*)\n    If true: (?P<true>.*)\n    If false: (?P<false>.*)").unwrap();
    let captures = re.captures(input).unwrap();
    // https://stackoverflow.com/a/54259908
    let dict = named_captures_to_hashmap(re, captures);
    dict
}

pub fn part_one(input: &str) -> Option<u64> {
    let rounds: u64 = 20;
    let mut monkeys: BTreeMap<u64, RefCell<Monkey>> = BTreeMap::new();
    for monkey_string in input.split("\n\n") {
        let monkey_data = parse_monkey_data(monkey_string);
        let monkey = Monkey::from_hashmap(monkey_data);
        monkeys.insert(monkey.id, RefCell::new(monkey));
    }
    for round in 0..rounds {
        for monkey in monkeys.values() {
            let mut monkey = monkey.borrow_mut();
            for _ in 0..monkey.starting_items.len() {
                let item = monkey.starting_items.pop_front().unwrap();
                let (next_monkey_id, new_item) = monkey.inspect_item(item);
                let next_monkey = monkeys.get(&next_monkey_id).unwrap();
                let mut next_monkey = next_monkey.borrow_mut();
                next_monkey.starting_items.push_back(new_item);
            }
        }
    }
    // println!("{:#?}", monkeys);
    // build a vector of the monkeys and their inspection counts, sorted greatest to least
    let mut monkey_inspection_counts: Vec<(u64, u64)> = monkeys
        .values()
        .map(|m| {
            let m = m.borrow();
            (m.id, m.inspection_count)
        })
        .collect();
    monkey_inspection_counts.sort_by(|a, b| b.1.cmp(&a.1));
    // multiply the inspection counts of the top 2 monkeys
    let top_two = &monkey_inspection_counts[0..2];
    let answer = top_two.iter().fold(1, |acc, x| acc * x.1);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rounds: u64 = 10_000;
    let mut monkeys: BTreeMap<u64, RefCell<Monkey>> = BTreeMap::new();
    let mut divisors: Vec<u64> = Vec::new();
    for monkey_string in input.split("\n\n") {
        let monkey_data = parse_monkey_data(monkey_string);
        let monkey = Monkey::from_hashmap(monkey_data);
        divisors.push(monkey.divisibility_test.to_owned());
        monkeys.insert(monkey.id, RefCell::new(monkey));
    }
    // collect all the divisibility rules from monkeys
    let modulo = divisors.iter().product();
    for round in 0..rounds {
        for monkey in monkeys.values() {
            let mut monkey = monkey.borrow_mut();
            for _ in 0..monkey.starting_items.len() {
                let item = monkey.starting_items.pop_front().unwrap();
                let (next_monkey_id, new_item) = monkey.inspect_item_no_relief(item, modulo);
                let next_monkey = monkeys.get(&next_monkey_id).unwrap();
                let mut next_monkey = next_monkey.borrow_mut();
                next_monkey.starting_items.push_back(new_item);
            }
        }
    }
    // println!("{:#?}", monkeys);
    // build a vector of the monkeys and their inspection counts, sorted greatest to least
    let mut monkey_inspection_counts: Vec<(u64, u64)> = monkeys
        .values()
        .map(|m| {
            let m = m.borrow();
            (m.id, m.inspection_count)
        })
        .collect();
    monkey_inspection_counts.sort_by(|a, b| b.1.cmp(&a.1));
    // multiply the inspection counts of the top 2 monkeys
    let top_two = &monkey_inspection_counts[0..2];
    let answer = top_two.iter().fold(1, |acc, x| acc * x.1);
    Some(answer)
    // dbg!(monkeys);
    // dbg!(&top_two);
    // None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
