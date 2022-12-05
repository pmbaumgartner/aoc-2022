use regex::Regex;
use std::collections::VecDeque;

fn parse_stacks(input: &str) -> Result<Vec<VecDeque<char>>, String> {
    /* Parse a stack which comes in as a string input where the first line has
    containers at the top of the stack, and the last line is the integer ID of the stack.

    The input of stacks means each line is a fixed width, and since each conteainer takes up
    3 character widths, and then has a space before the next container. This means we have to first learn
    how many stacks there with the formula `length = 3 * num_containers + (num_containers - 1)`. If we solve this
    equation for num_containers, we get `num_containers = (length + 1) / 4`.

    Example input is of this form:
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

    In this case, there are 3 stacks. Stack 1 is two containers high, stack 2 is three containers high,
    and stack 3 is one container high. The first container in each stack is the top of the stack. The label of the container
    is inbetween square brackets, e.g. [N] is a container with label (char) N. Container labels appear between
    square brackets and are always a single uppercase character.
    */

    // Collect the input lines in reverse
    let lines = input.lines().rev().collect::<Vec<_>>();
    let num_containers: usize = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(num_containers);
    for _ in 0..num_containers {
        stacks.push(VecDeque::new());
    }
    for line in lines[1..].iter() {
        // Skipping the first character, generate an iterator that returns
        // every 4th character, which is the container label
        let chars = line[1..].chars().step_by(4);
        for (stack_id, container) in chars.enumerate() {
            match container {
                ' ' => continue,
                // match case to check if the container is a valid uppercase character
                _ if container.is_ascii_uppercase() => stacks[stack_id].push_front(container),
                _ => return Err(format!("Invalid container label: {}", container)),
            }
        }
    }
    Ok(stacks)
}

struct Move {
    number: usize,
    source: usize,
    target: usize,
}

fn parse_moves(input: &str) -> Result<Vec<Move>, String> {
    /* Using a regex, parse input moves into a Move. Input is in the form 'move <number> from <source> to <target>'

    example input:
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    */
    let re = Regex::new(r"move (?P<number>\d+) from (?P<source>\d+) to (?P<target>\d+)").unwrap();
    let mut moves = Vec::new();
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            moves.push(Move {
                number: captures["number"].parse().unwrap(),
                source: captures["source"].parse().unwrap(),
                target: captures["target"].parse().unwrap(),
            });
        } else {
            return Err(format!("Invalid input move: {}", line));
        }
    }
    Ok(moves)
}

fn parse_part_one(input: &str) -> (Vec<VecDeque<char>>, Vec<Move>) {
    /* Parse the input of stacks and moves using the associated helper functions.

    There are two newlines between the stacks and the moves, so we can split the input on that to get the two parts.

    Example input is of this form:
        [D]
    [N] [C]
    [Z] [M] [P]
    1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    <EOF>
    */
    let mut parts = input.split("\n\n");
    let stacks = parse_stacks(parts.next().unwrap()).unwrap();
    let moves = parse_moves(parts.next().unwrap()).unwrap();
    (stacks, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    // Given the puzzle input, we need to move the containers from one stack to the other
    // and return the container at the top of each stack
    let (mut stacks, moves) = parse_part_one(input);
    for move_ in moves {
        for _ in 0..move_.number {
            let container = stacks[move_.source - 1].pop_front().unwrap();
            stacks[move_.target - 1].push_front(container);
        }
    }
    let mut labels = Vec::new();
    for stack in stacks {
        labels.push(stack[0]);
    }
    Some(labels.iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    // Apply the same logic as `part_one`, except this time time we make a move
    // the containers on the stack stay in the same order, so rather than popping
    // and pushing each individual container, we need to collect the number of containers
    // in their original order then stack them in that same order on top of the new stack
    let (mut stacks, moves) = parse_part_one(input);
    for move_ in moves {
        let mut containers = Vec::new();
        for _ in 0..move_.number {
            let container = stacks[move_.source - 1].pop_front().unwrap();
            containers.push(container);
        }
        for container in containers.into_iter().rev() {
            stacks[move_.target - 1].push_front(container);
        }
    }
    let mut labels = Vec::new();
    for stack in stacks {
        labels.push(stack[0]);
    }
    Some(labels.iter().collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
