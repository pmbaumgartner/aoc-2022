use std::collections::HashMap;

use rayon::prelude::*;
#[derive(Debug, Clone, PartialEq)]

enum Operation<'a> {
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Equal(&'a str, &'a str),
}

#[derive(Debug, Clone, PartialEq)]
enum State<'a> {
    Number(u64),
    Operation(Operation<'a>),
    Equal(bool),
}

// Parse inputs, which are the name of the monkey, followed by a colon,
// then either a number, or two other monkey's names and an operation
// between them.
// Examples:
// root: pppw + sjmn
// dbpl: 5
// cczh: sllz + lgvd
// zczc: 2
// ptdq: humn - dvpt
// dvpt: 3
// lfqf: 4
// humn: 5
// ljgn: 2
// sjmn: drzm * dbpl
// sllz: 4
// pppw: cczh / lfqf
// lgvd: ljgn * ptdq
// drzm: hmdt - zczc
// hmdt: 32
fn parse_input(input: &str) -> HashMap<&str, State> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();
        let value = if let Ok(number) = value.parse::<u64>() {
            State::Number(number)
        } else {
            let mut parts = value.split_whitespace();
            let left = parts.next().unwrap();
            let operation = parts.next().unwrap();
            let right = parts.next().unwrap();
            let operation = match operation {
                "+" => Operation::Add(left, right),
                "-" => Operation::Subtract(left, right),
                "*" => Operation::Multiply(left, right),
                "/" => Operation::Divide(left, right),
                _ => panic!("Unknown operation: {}", operation),
            };

            State::Operation(operation)
        };
        map.insert(key, value);
    }
    map
}

fn parse_input_part_two(input: &str) -> HashMap<&str, State> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();
        let value = if let Ok(number) = value.parse::<u64>() {
            State::Number(number)
        } else {
            let mut parts = value.split_whitespace();
            let left = parts.next().unwrap();
            let operation = parts.next().unwrap();
            let right = parts.next().unwrap();
            let operation = match (key, operation) {
                ("root", _) => Operation::Equal(left, right),
                (_, "+") => Operation::Add(left, right),
                (_, "-") => Operation::Subtract(left, right),
                (_, "*") => Operation::Multiply(left, right),
                (_, "/") => Operation::Divide(left, right),
                _ => panic!("Unknown operation: {}", operation),
            };

            State::Operation(operation)
        };
        map.insert(key, value);
    }
    map
}

fn find_human_value(mut map: HashMap<&str, State>, human_value: u64) -> Result<u64, String> {
    let human = State::Number(human_value);
    map.insert("humn", human.clone());

    let mut root_value: State = map.get("root").unwrap().clone();

    'outer: while matches!(root_value, State::Operation(_)) {
        let map_copy = map.clone();
        for (key, value) in map_copy.iter() {
            match value {
                State::Number(_) => {}
                State::Operation(operation) => match operation {
                    Operation::Add(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left + *right));
                        }
                    }
                    Operation::Subtract(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            // check if left - right is positive or would result in overflow
                            if left < right {
                                break 'outer;
                            }
                            map.insert(*key, State::Number(*left - *right));
                        }
                    }
                    Operation::Multiply(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left * *right));
                        }
                    }
                    Operation::Divide(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            // check if left and right can be evenly divided,
                            // as they're both integers
                            if left % right != 0 {
                                break 'outer;
                            }
                            map.insert(*key, State::Number(*left / *right));
                        }
                    }
                    Operation::Equal(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            if key != &"root" {
                                panic!("Key not Root with Equal")
                            }
                            map.insert(*key, State::Equal(*left == *right));
                        }
                    }
                },
                State::Equal(_) => break 'outer,
            }
        }
        root_value = map.get("root").unwrap().clone();
    }

    // if root-value is an equal true state,
    // return human value

    match root_value {
        State::Equal(true) => Ok(human_value),
        _ => Err(format!("No human value found for {}", human_value)),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut root_value: &State = map.get("root").unwrap();
    while matches!(root_value, State::Operation(_)) {
        let map_copy = map.clone();
        for (key, value) in map_copy.iter() {
            match value {
                State::Number(_) => {}
                State::Operation(operation) => match operation {
                    Operation::Add(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left + *right));
                        }
                    }
                    Operation::Subtract(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left - *right));
                        }
                    }
                    Operation::Multiply(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left * *right));
                        }
                    }
                    Operation::Divide(left, right) => {
                        let left_value = map.get(left).unwrap();
                        let right_value = map.get(right).unwrap();
                        if let (State::Number(left), State::Number(right)) =
                            (left_value, right_value)
                        {
                            map.insert(*key, State::Number(*left / *right));
                        }
                    }
                    _ => panic!("Equal in Part 1"),
                },
                _ => panic!("Unknown state in Part 1"),
            }
        }
        root_value = map.get("root").unwrap();
    }
    Some(match root_value {
        State::Number(number) => *number,
        State::Operation(_) => panic!("Root value is still an operation"),
        State::Equal(_) => panic!("Root value is still an operation"),
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input_part_two(input);
    let mut start_value = 1;
    loop {
        let map_clone = map.clone();
        match find_human_value(map_clone, start_value) {
            Ok(human_value) => return Some(human_value),
            Err(_) => start_value += 1,
        }
        if start_value % 100_000_000 == 0 {
            println!("Start value: {}", start_value);
        }
    }
    // (0..u64::MAX).into_par_iter().for_each(|start_value| {
    //     let map_clone = map.clone();
    //     match find_human_value(map_clone, start_value) {
    //         Ok(human_value) => return Some(human_value),
    //         Err(_) => start_value += 1,
    //     }
    //     if start_value % 1000 == 0 {
    //         println!("Start value: {}", start_value);
    //     }
    // });
    // None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
