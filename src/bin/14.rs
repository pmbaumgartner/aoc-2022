use std::{cmp::Ordering, collections::HashSet};

struct Cave {
    objects: HashSet<(i64, u64)>,
    height: u64,
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut rock_locations: Vec<(u64, u64)> = Vec::new();
    for line in input.lines() {
        let endpoints = line
            .split("->")
            .map(|s| s.trim())
            .map(|s| s.split(',').collect::<Vec<_>>())
            .map(|v| (v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()))
            .collect::<Vec<_>>();
        for pair in endpoints.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            match (x1.cmp(&x2), y1.cmp(&y2)) {
                (Ordering::Equal, Ordering::Less) => {
                    for y in y1..=y2 {
                        rock_locations.push((x1, y));
                    }
                }
                (Ordering::Equal, Ordering::Greater) => {
                    for y in y2..=y1 {
                        rock_locations.push((x1, y));
                    }
                }
                (Ordering::Less, Ordering::Equal) => {
                    for x in x1..=x2 {
                        rock_locations.push((x, y1));
                    }
                }
                (Ordering::Greater, Ordering::Equal) => {
                    for x in x2..=x1 {
                        rock_locations.push((x, y1));
                    }
                }
                _ => panic!("invalid line"),
            }
        }
    }
    rock_locations
}

#[derive(PartialEq)]
enum StoppingCriteria {
    Overflow,
    Spout,
}

use StoppingCriteria::*;

impl Cave {
    fn new(rock_locations: Vec<(u64, u64)>) -> Cave {
        let mut objects = HashSet::new();
        for (x, y) in rock_locations {
            objects.insert((x as i64, y));
        }
        let height = objects.iter().map(|(_, y)| y).max().unwrap() + 1;
        Cave { objects, height }
    }

    fn fill_sand(&mut self, floor: u64, criteria: StoppingCriteria) -> u64 {
        let mut sand_count: u64 = 0;
        let mut x: i64 = 500;
        let mut y = 0;
        loop {
            match criteria {
                Overflow => {
                    if y + 1 >= self.height {
                        return sand_count;
                    }
                }
                Spout => {
                    if y + 1 == self.height + floor {
                        self.objects.insert((x, y));
                        x = 500;
                        y = 0;
                    }
                }
            }
            let point_below = (x, y + 1);
            let point_diagonal_left = (x - 1, y + 1);
            let point_diagonal_right = (x + 1, y + 1);

            match self.objects.contains(&point_below) {
                true => match self.objects.contains(&point_diagonal_left) {
                    true => match self.objects.contains(&point_diagonal_right) {
                        true => {
                            sand_count += 1;
                            if criteria == Spout && (x, y) == (500, 0) {
                                return sand_count;
                            }
                            self.objects.insert((x, y));
                            x = 500;
                            y = 0;
                        }
                        false => {
                            x += 1;
                            y += 1;
                        }
                    },
                    false => {
                        x -= 1;
                        y += 1;
                    }
                },
                false => {
                    y += 1;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rock_locations = parse_input(input);
    let mut cave = Cave::new(rock_locations);
    // cave.print();
    let dropped_sand = cave.fill_sand(0, Overflow);
    // cave.print();
    Some(dropped_sand)
    // None
}

pub fn part_two(input: &str) -> Option<u64> {
    let rock_locations = parse_input(input);
    let mut cave = Cave::new(rock_locations);
    let dropped_sand = cave.fill_sand(2, Spout);
    Some(dropped_sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
