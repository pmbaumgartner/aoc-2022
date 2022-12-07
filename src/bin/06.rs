use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    // Iterate through the input, storing it 4 characters at a time. Store
    // this buffer in a Vec or VecDeque. We need to check if the current 4 characters
    // in the buffer are unique. If they are unique, we want to return the index
    // when that occurred.
    let mut buffer = VecDeque::new();
    for (index, character) in input.chars().enumerate() {
        buffer.push_back(character);
        if buffer.len() == 4 {
            let set: HashSet<&char> = HashSet::from_iter(buffer.iter());
            if set.len() == 4 {
                return Some((index as u32) + 1);
            }
            buffer.pop_front();
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    // same as pt 1, but check for 14 unique characters
    let mut buffer = VecDeque::new();
    for (index, character) in input.chars().enumerate() {
        buffer.push_back(character);
        if buffer.len() == 14 {
            let set: HashSet<&char> = HashSet::from_iter(buffer.iter());
            if set.len() == 14 {
                return Some((index as u32) + 1);
            }
            buffer.pop_front();
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
