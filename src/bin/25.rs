fn snafu_to_int(s: &str) -> i64 {
    let mut result = 0;
    let chars: Vec<char> = s.chars().rev().collect();
    for (i, c) in chars.iter().enumerate() {
        let base = 5i64.pow(i as u32);
        match c {
            '2' => result += 2 * base,
            '1' => result += 1 * base,
            '0' => (),
            '-' => result -= 1 * base,
            '=' => result -= 2 * base,
            _ => panic!("Invalid character: {}", c),
        }
    }
    result
}
// Definitely needed help on this one
// https://github.com/ephemient/aoc2022/blob/main/rs/src/day25.rs#L20
fn int_to_snafu(mut num: i64) -> String {
    let base = 5;
    let mut result = String::new();

    while num != 0 {
        let digit = (num + 2) % base;
        result.push(match digit {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => panic!("Invalid digit"),
        });
        num = (num + 2) / base;
    }
    result.chars().rev().collect()
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(snafu_to_int).collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let values = parse_input(input);
    let total = values.iter().sum::<i64>();
    println!("total: {}", total);
    let total_snafu = int_to_snafu(total);
    Some(total_snafu)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
