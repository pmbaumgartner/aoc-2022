use std::str::FromStr;

struct SectionRange {
    start: usize,
    end: usize,
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split('-');
        if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
            let start = start.parse::<usize>().map_err(|e| e.to_string())?;
            let end = end.parse::<usize>().map_err(|e| e.to_string())?;
            Ok(SectionRange { start, end })
        } else {
            Err(format!("Invalid section range: {}", value))
        }
    }
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

struct SectionRangePair {
    first: SectionRange,
    second: SectionRange,
}

impl SectionRangePair {
    fn either_contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlap_exists(&self) -> bool {
        // We only need to check one beacuse this relatinoship is symmetric
        // If one overlaps the other, then the other overlaps the first
        self.first.overlaps(&self.second)
    }
}

fn parse_part_one(input: &str) -> Result<Vec<SectionRangePair>, String> {
    input
        .lines()
        .map(|line| line.split(','))
        .map(|mut parts| {
            if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                Ok(SectionRangePair {
                    first: first.parse::<SectionRange>()?,
                    second: second.parse::<SectionRange>()?,
                })
            } else {
                Err(format!("Invalid section range pair: {:?}", parts))
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let assignments = parse_part_one(input).ok()?;
    let mut overlaps = 0;
    for pair in assignments.iter() {
        if pair.either_contains() {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let assignments = parse_part_one(input).ok()?;
    let mut overlaps = 0;
    for pair in assignments.iter() {
        if pair.overlap_exists() {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
