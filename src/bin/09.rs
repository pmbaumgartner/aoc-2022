use std::collections::HashSet;
enum MoveInstruction {
    // These should really be u32, but this will simplify the number of conversions
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone)]
struct Knot {
    x: i32,
    y: i32,
    history: Vec<(i32, i32)>,
}

impl Knot {
    fn new() -> Self {
        Knot {
            x: 0,
            y: 0,
            history: vec![(0, 0)],
        }
    }
    fn is_adjacent(&self, other: &Knot) -> bool {
        // Check if we're adjacent to a knot including diagonal adjacency
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        if x_diff <= 1 && y_diff <= 1 {
            return true;
        }
        false
    }
    // Move
    // For movement we have to follow the other knots position
    // First: Check if we are in an adjacent position to the other knot (diagonal included).
    // If we are we don't need to move
    // Second: check if we are in the same x or y axis as other knot. If we are not, make
    // that move first. I.e. if other knot is one row over and 3 up, move diagonal (all at once)
    // 1 row over and 1 up, then move 1 up.
    // the knots can't overlap, so the knot will always stop moving when it's one space adjacent to the other
    // We have to store all the locations (x, y) we've visited in history, not just the last one.
    fn move_relative(&mut self, other: &Knot) {
        loop {
            if self.is_adjacent(other) {
                self.history.push((self.x, self.y));
                break;
            }
            if self.x != other.x {
                if self.x < other.x {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            }
            if self.y != other.y {
                if self.y < other.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
            self.history.push((self.x, self.y));
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            knots: vec![Knot::new(); length],
        }
    }
    fn follow_up(&mut self) {
        for i in 1..self.knots.len() {
            // We have to clone here because we can't borrow both as mutable and immutable
            let temp_knot = self.knots[i - 1].clone();
            self.knots[i].move_relative(&temp_knot);
        }
    }
    fn movement(&mut self, instruction: &MoveInstruction) {
        // We need to move the rope. We do this by moving the first knot following the instruction
        // with knot.move_direct(instruction). Then we move the rest of the rope following the first knot
        // with knot.move_relative(&self.knots[i - 1])
        match instruction {
            MoveInstruction::Up(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].y += 1;
                    self.follow_up();
                }
            }
            MoveInstruction::Down(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].y -= 1;
                    self.follow_up();
                }
            }
            MoveInstruction::Left(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].x -= 1;
                    self.follow_up();
                }
            }
            MoveInstruction::Right(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].x += 1;
                    self.follow_up();
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<MoveInstruction> {
    // Input is of the following form:
    // R 4
    // U 4
    // L 3
    // D 1
    input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut line| {
            let direction = line.next().unwrap();
            let distance = line.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "R" => MoveInstruction::Right(distance),
                "L" => MoveInstruction::Left(distance),
                "U" => MoveInstruction::Up(distance),
                "D" => MoveInstruction::Down(distance),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut rope = Rope::new(2);
    for (i, instruction) in instructions.iter().enumerate() {
        rope.movement(instruction);
        // dbg!(&i + 1, &rope);
    }
    // Get unique locations in tail.history without using .sort()
    let tail = &rope.knots[rope.knots.len() - 1];
    dbg!(&tail.history);
    let unique_locations: HashSet<(i32, i32)> = tail.history.iter().cloned().collect();
    Some(dbg!(unique_locations).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // 2525 = too low
    let instructions = parse_input(input);
    let mut rope = Rope::new(10);
    for (i, instruction) in instructions.iter().enumerate() {
        rope.movement(instruction);
        // dbg!(&i + 1, &rope);
    }
    // Get unique locations in tail.history without using .sort()
    let tail = &rope.knots[rope.knots.len() - 1];
    dbg!(&tail.history);
    let unique_locations: HashSet<(i32, i32)> = tail.history.iter().cloned().collect();
    Some((unique_locations).len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
