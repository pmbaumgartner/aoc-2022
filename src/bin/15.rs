use std::collections::HashSet;

use itertools::Itertools;
use regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sensor {
    x: i32,
    y: i32,
    nearest: Beacon,
}

impl Sensor {
    fn distance(&self, location: (i32, i32)) -> u32 {
        ((self.x - location.0).abs() + (self.y - location.1).abs()) as u32
    }
    fn distance_to_nearest(&self) -> u32 {
        self.distance((self.nearest.x, self.nearest.y))
    }

    fn perimeter(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let distance = self.distance_to_nearest() as i32;
        for y in 0..distance {
            let x = distance - y;
            points.push((self.x + x, self.y + y));
            points.push((self.x + x, self.y - y));
            points.push((self.x - x, self.y + y));
            points.push((self.x - x, self.y - y));
        }
        // given the distance from the central point, we want to generate
        // the points on the perimeter using manhattan distance
        // this means we can go all the way out to the x boundaries
        // given by `starting`, then we iterate over those values
        // and generate the y values

        points.iter().unique().cloned().collect()
    }
    fn range_given_row(&self, y: i32) -> Option<(i32, i32)> {
        // given a row (y), return all points that are within that row
        // using the perimeter (which only contains the boundaries)
        let binding = self.perimeter();
        // println!("binding: {:?}", binding);
        let perimeter = binding
            .iter()
            .filter(|(_, y1)| *y1 == y)
            .collect::<Vec<_>>();
        // println!("perimeter: {:?}", perimeter);
        let min_x = perimeter.iter().map(|(x, _)| x).min();
        let max_x = perimeter.iter().map(|(x, _)| x).max();

        // println!("min_x: {:?}, max_x: {:?}", min_x, max_x);
        if let Some(min_x) = min_x {
            if let Some(max_x) = max_x {
                return Some((*min_x, *max_x));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    // We need to write a regex to capture the sensor and beacon locations
    // the input looks like this:
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    // Sensor at x=9, y=16: closest beacon is at x=10, y=16
    // Sensor at x=13, y=2: closest beacon is at x=15, y=3
    // Sensor at x=12, y=14: closest beacon is at x=10, y=16
    // Sensor at x=10, y=20: closest beacon is at x=10, y=16
    // Sensor at x=14, y=17: closest beacon is at x=10, y=16
    // Sensor at x=8, y=7: closest beacon is at x=2, y=10
    // Don't forget to capture negative numbers!
    let sensor_re = regex::Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for line in input.lines() {
        let caps = sensor_re.captures(line).unwrap();
        let x = caps.name("sx").unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.name("sy").unwrap().as_str().parse::<i32>().unwrap();
        let bx = caps.name("bx").unwrap().as_str().parse::<i32>().unwrap();
        let by = caps.name("by").unwrap().as_str().parse::<i32>().unwrap();
        let beacon = Beacon { x: bx, y: by };
        sensors.push(Sensor {
            x,
            y,
            nearest: beacon,
        });
        beacons.push(beacon);
    }
    (sensors, beacons)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (sensors, beacons) = parse_input(input);

    // let all_ranges = sensors
    //     .iter()
    //     .flat_map(|s| s.range())
    //     .unique()
    //     .collect::<Vec<_>>();
    // find how many range values are in row 10
    // Test input is row 10, real input is row 2000000
    let beacon_positions = beacons.iter().map(|b| (b.x, b.y)).collect::<Vec<_>>();
    let row = 10;
    let all_ranges = sensors
        .iter()
        .filter_map(|s| (s.range_given_row(row)))
        .collect::<Vec<_>>();
    let mut row_locations: HashSet<(i32, i32)> = HashSet::new();
    for (start, end) in all_ranges {
        for x in start..=end {
            if beacon_positions.contains(&(x, row)) {
                continue;
            }
            row_locations.insert((x, row));
            // println!("{}", row_locations.len())
        }
    }

    // let total_row = all_ranges
    //     .iter()
    //     .filter(|(_, y)| *y == row)
    //     .inspect(|pos| {
    //         println!("{:?}", pos);
    //     })
    //     .filter(|pos| !beacon_positions.contains(pos))
    //     .count();

    Some(row_locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
