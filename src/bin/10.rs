use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
struct Register {
    history: Vec<i32>,
}

impl Register {
    fn new() -> Self {
        Self { history: vec![] }
    }
}
#[derive(Debug)]
enum Instruction {
    Add(i32),
    Noop,
}

fn parse_instruction(input: &str) -> Instruction {
    let input_split = input.split(' ').collect::<Vec<&str>>();
    match input_split[..] {
        [_instr, amount] => {
            let value = i32::from_str(amount).unwrap();
            Instruction::Add(value)
        }
        ["noop"] => Instruction::Noop,
        _ => panic!("Invalid instruction"),
    }
}

#[derive(Debug)]
struct Operation {
    value: i32,
    cycles: u32,
}

impl Operation {
    fn new(value: i32, cycles: u32) -> Self {
        Self { value, cycles }
    }
    fn complete(&self) -> bool {
        self.cycles == 0
    }
}
impl From<&Instruction> for Operation {
    fn from(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Add(value) => Self::new(*value, 2),
            Instruction::Noop => Self::new(0, 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = input
        .lines()
        .map(parse_instruction)
        .collect::<Vec<Instruction>>();
    let mut operations_queue: VecDeque<Operation> = VecDeque::new();
    let mut register = Register::new();
    let mut cycle = 1;
    while !instructions.is_empty() || !operations_queue.is_empty() {
        let instruction = instructions.get(cycle - 1);
        if let Some(instruction) = instruction {
            operations_queue.push_back(instruction.into());
        }
        dbg!(&cycle);
        if operations_queue.is_empty() {
            break;
        }
        let mut operation = operations_queue.pop_front().unwrap();
        operation.cycles -= 1;

        dbg!(cycle, &operation, register.history.last());
        if operation.complete() {
            let value = operation.value;
            let history = register.history.last().unwrap_or(&1);
            register.history.push(*history + value);
        } else {
            operations_queue.push_front(operation);
            let history = register.history.last().unwrap_or(&1);
            register.history.push(*history);
        }
        cycle += 1;
    }
    let mut sum = 0;
    for cycle in (1..register.history.len()).skip(19).step_by(40) {
        let reg_value = dbg!(register.history.get(cycle - 2).unwrap());
        sum += reg_value * cycle as i32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> String {
    let instructions = input
        .lines()
        .map(parse_instruction)
        .collect::<Vec<Instruction>>();
    let mut operations_queue: VecDeque<Operation> = VecDeque::new();
    let mut register = Register::new();
    let mut cycle = 1;
    while !instructions.is_empty() || !operations_queue.is_empty() {
        let instruction = instructions.get(cycle - 1);
        if let Some(instruction) = instruction {
            operations_queue.push_back(instruction.into());
        }
        if operations_queue.is_empty() {
            break;
        }
        let mut operation = operations_queue.pop_front().unwrap();
        operation.cycles -= 1;
        if operation.complete() {
            let value = operation.value;
            let history = register.history.last().unwrap_or(&1);
            register.history.push(*history + value);
        } else {
            operations_queue.push_front(operation);
            let history = register.history.last().unwrap_or(&1);
            register.history.push(*history);
        }
        cycle += 1;
    }
    let output: String = "".to_string();
    // So if we were better, we would have come up with an abstraction
    // That does these things in order (sprite draw, then register value)
    // But we didn't, so we're just going to add a 1 to our history vector
    let mut history_copy = register.history.clone();
    let mut new_history = vec![1];
    new_history.append(&mut history_copy);
    for (i, x) in new_history.iter().enumerate() {
        let sprite_range = x - 1..=x + 1;
        let horizontal_position = i % 40;
        match sprite_range.contains(&(horizontal_position as i32)) {
            true => print!("#"),
            false => print!("."),
        }
        if horizontal_position == 39 {
            println!();
        }
    }
    output
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    part_two(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        part_two(&input);
    }
}
