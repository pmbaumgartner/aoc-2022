use std::collections::HashMap;

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

// We need to implement a next_direction for Direction so that we can
// cycle through them in the order we've declared them. The order is
// North, South, West, East
impl Direction {
    fn next_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn comparison_vectors(&self) -> Vec<IVec2> {
        match self {
            Direction::North => vec![IVec2::new(0, -1), IVec2::new(1, -1), IVec2::new(-1, -1)],
            Direction::South => vec![IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(-1, 1)],
            Direction::West => vec![IVec2::new(-1, 0), IVec2::new(-1, 1), IVec2::new(-1, -1)],
            Direction::East => vec![IVec2::new(1, 0), IVec2::new(1, 1), IVec2::new(1, -1)],
        }
    }
    fn direction_cycle(&self) -> Vec<Direction> {
        let mut directions = vec![*self];
        let mut next_direction = self.next_direction();
        while next_direction != *self {
            directions.push(next_direction);
            next_direction = next_direction.next_direction();
        }
        directions
    }
}

// For an elves location, check whether there is another elf in the neighboring positoins
// For example, if the direction is North, check whether there is an elf in the N, NE, or NW position
// relative to that elf's location. If there is an elf in that position, check the next direction. If
// there aren't any neighbors in that direction, return the position they would move to if the headed
// in that cardinal direction. If there are no available moves, because that elf is surrounded,
// return None. Remember for each direction we need to check that direction and the two neighboring diagonals.
// Also remember that if we cycle throug all directions and this elf can't go anywhere, return None.
fn check_direction(elf: &IVec2, direction: &Direction, locations: &[IVec2]) -> Option<IVec2> {
    // check if there are no neighbors surrounding this elf's position
    // Check in all directions first
    let any_direction = direction
        .direction_cycle()
        .iter()
        .flat_map(|d| d.comparison_vectors())
        .any(|v| locations.contains(&(*elf + v)));
    if !any_direction {
        return None;
    }
    let directions = direction.direction_cycle();
    'direction: for direction in directions {
        let comparison_vectors = direction.comparison_vectors();
        for comparison_vector in comparison_vectors {
            let comparison_location = *elf + comparison_vector;
            match locations.contains(&comparison_location) {
                true => continue 'direction,
                false => {}
            }
        }
        return Some(*elf + direction.comparison_vectors()[0]);
    }
    None
}

// The input is a grid of locations of elves, represented with
// a '#' and a '.' for open space. Example:
// ....#..
// ..###.#
// #...#.#
// .#...##
// #.###..
// ##.#.##
// .#..#..
//
// We need to parse them into IVec2
fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(IVec2::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect()
}

fn get_boundaries(locations: &Vec<IVec2>) -> (IVec2, IVec2) {
    let mut min = IVec2::new(std::i32::MAX, std::i32::MAX);
    let mut max = IVec2::new(std::i32::MIN, std::i32::MIN);
    for location in locations {
        if location.x < min.x {
            min.x = location.x;
        }
        if location.y < min.y {
            min.y = location.y;
        }
        if location.x > max.x {
            max.x = location.x;
        }
        if location.y > max.y {
            max.y = location.y;
        }
    }
    (min, max)
}

fn get_size(corners: (IVec2, IVec2)) -> (u32, u32) {
    let (min, max) = corners;
    ((max.x - min.x + 1) as u32, (max.y - min.y + 1) as u32)
}

fn display_vectors(vectors: &Vec<IVec2>) {
    let (min, max) = get_boundaries(vectors);
    let (width, height) = get_size((min, max));
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    // draw the grid. remember that the coordinates are signed integers,
    // but the grid must be created with unsigned integers, so we have to do
    // some math to "shift" the grid to the right place
    for vector in vectors {
        grid[(vector.y - min.y) as usize][(vector.x - min.x) as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let starting_locations = parse(input);
    let rounds = 10;
    let mut direction_cycle = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle();
    let mut locations = starting_locations;
    let mut round_direction = direction_cycle.next().unwrap();
    for round in 0..rounds {
        let mut proposed_locations: HashMap<IVec2, IVec2> = locations
            .iter()
            .filter_map(|location| {
                check_direction(location, &round_direction, &locations)
                    .map(|proposed_location| (*location, proposed_location))
            })
            .collect();
        // dbg!(&proposed_locations);
        // if there are duplicates in proposed locations, the elves at those positions don't move
        // identify the duplicates and remove them from the proposed locations
        let mut duplicate_locations = vec![];
        for (location, proposed_location) in proposed_locations.iter() {
            if proposed_locations
                .values()
                .filter(|x| x == &proposed_location)
                .count()
                > 1
            {
                duplicate_locations.push(*location);
            }
        }
        // dbg!(&duplicate_locations);
        for duplicate_location in duplicate_locations {
            proposed_locations.remove(&duplicate_location);
        }
        // update the locations by moving any elf that has a proposed location to that new location
        for (location, proposed_location) in proposed_locations.iter() {
            locations.retain(|x| x != location);
            locations.push(*proposed_location);
        }
        round_direction = direction_cycle.next().unwrap();
        println!("round: {}", round);
        display_vectors(&locations);
        println!("size: {}", {
            let (min, max) = get_boundaries(&locations);
            let (width, height) = get_size((min, max));
            width * height
        });
    }

    Some(
        {
            let (min, max) = get_boundaries(&locations);
            let (width, height) = get_size((min, max));
            width * height
        } - locations.len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let starting_locations = parse(input);
    let mut direction_cycle = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle();
    let mut locations = starting_locations;
    let mut round_direction = direction_cycle.next().unwrap();
    let mut no_proposed_locations = false;
    let mut round = 0;
    while !no_proposed_locations {
        let mut proposed_locations: HashMap<IVec2, IVec2> = locations
            .iter()
            .filter_map(|location| {
                check_direction(location, &round_direction, &locations)
                    .map(|proposed_location| (*location, proposed_location))
            })
            .collect();
        no_proposed_locations = proposed_locations.is_empty();
        println!(
            "round: {}, proposed_locations: {}",
            round,
            proposed_locations.len()
        );

        // dbg!(&proposed_locations);
        // if there are duplicates in proposed locations, the elves at those positions don't move
        // identify the duplicates and remove them from the proposed locations
        let mut duplicate_locations = vec![];
        for (location, proposed_location) in proposed_locations.iter() {
            if proposed_locations
                .values()
                .filter(|x| x == &proposed_location)
                .count()
                > 1
            {
                duplicate_locations.push(*location);
            }
        }
        // dbg!(&duplicate_locations);
        for duplicate_location in duplicate_locations {
            proposed_locations.remove(&duplicate_location);
        }
        // update the locations by moving any elf that has a proposed location to that new location
        for (location, proposed_location) in proposed_locations.iter() {
            locations.retain(|x| x != location);
            locations.push(*proposed_location);
        }
        round_direction = direction_cycle.next().unwrap();
        round += 1;
    }

    Some(round as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
