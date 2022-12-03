use std::collections::HashSet;

pub trait Priority {
    fn to_priority(&self) -> u32;
}

impl Priority for char {
    fn to_priority(&self) -> u32 {
        // Convert the character to a value where
        // Lowercase item types a through z have values 1 through 26.
        // Uppercase item types A through Z have values 27 through 52.
        // All other characters have value 0.
        match *self {
            'a'..='z' => *self as u32 - 'a' as u32 + 1,
            'A'..='Z' => *self as u32 - 'A' as u32 + 27,
            _ => 0,
        }
    }
}

fn split_vector(v: &mut Vec<char>) -> (Vec<char>, Vec<char>) {
    // split the vector in half, throw an error if the vector is empty or contains an odd number of elements
    let mid = v.len() / 2;
    if mid == 0 {
        panic!("Vector must contain at least one element");
    }
    if v.len() % 2 != 0 {
        panic!("Vector must contain an even number of elements");
    }
    let (left, right) = v.split_at_mut(mid);
    (left.to_vec(), right.to_vec())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    // parse the input string into a vector of vectors of characters
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_common_elements(left: &Vec<char>, right: &Vec<char>) -> HashSet<char> {
    // find a single element in common in both vectors
    let mut common = HashSet::new();
    for item in left {
        if right.contains(item) {
            common.insert(*item);
        }
    }
    common
}

fn find_common_elements_all(vectors: Vec<Vec<char>>) -> HashSet<char> {
    // Find a single element in common with all of the vectors
    let mut common = HashSet::new();
    for item in &vectors[0] {
        if vectors.iter().all(|v| v.contains(item)) {
            common.insert(*item);
        }
    }
    common
}

fn parse_input_two(input: &str) -> Vec<Vec<Vec<char>>> {
    // parse the input so that lines are grouped into elf groups
    // Every third line is a new group
    input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut groups, (i, line)| {
            if i % 3 == 0 {
                groups.push(Vec::new());
            }
            groups.last_mut().unwrap().push(line.chars().collect());
            groups
        })
}
pub fn part_one(input: &str) -> Option<u32> {
    let both_sacks = parse_input(input);
    let mut total_priority = 0;
    for sack in both_sacks {
        let (left, right) = split_vector(&mut sack.clone());
        let common = find_common_elements(&left, &right);
        for item in common {
            total_priority += item.to_priority();
        }
    }
    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups = parse_input_two(input);
    let mut total_priority = 0;
    for group in groups {
        let common = find_common_elements_all(group);
        for item in common {
            total_priority += item.to_priority();
        }
    }
    Some(total_priority)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
