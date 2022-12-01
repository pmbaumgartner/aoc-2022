fn parse(input: &str) -> Vec<u32> {
    let items = input.split("\n\n").collect::<Vec<&str>>();
    let parsed_items: Vec<u32> = items
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    parsed_items
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_calories = parse(input);
    elf_calories.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories = parse(input);
    // get sum of the largest 3 elements in vector
    elf_calories.sort();
    elf_calories.reverse();
    elf_calories.truncate(3);
    Some(elf_calories.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
