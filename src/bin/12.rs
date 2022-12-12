use pathfinding::directed::astar::astar;

#[derive(Debug, Clone)]

struct Grid {
    positions: Vec<Vec<Position>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    value: u32,
}

// We need a way to find the neighbors of a position
impl Grid {
    fn new(positions: Vec<Vec<Position>>) -> Self {
        Grid { positions }
    }
    fn get(&self, x: i32, y: i32) -> Option<&Position> {
        // get should return a reference to the position at the given x and y coordinates
        // if the position is out of bounds, return None
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if y >= self.positions.len() {
            return None;
        }
        let row = &self.positions[y];
        if x >= row.len() {
            return None;
        }
        Some(&row[x])
    }
    fn neighbors(&self, position: &Position) -> Vec<(Position, u32)> {
        // neighbors should return a list of neighbors plus the cost of moving to that neighbor
        // This should NOT include diagonal neighbors, we can only move up, down, left, right
        // We can only move to positions that have a value of at most this `position.value + 1`
        // if we can move there, the cost is 1
        // if the position we're asking to move to has a value greater than that,
        // It's not a neighbor and we can't move there
        // Note: We can NOT move diagonally

        let mut neighbors = Vec::new();
        for (x, y) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let x_lookup = position.x + x;
            let y_lookup = position.y + y;
            if let Some(neighbor) = self.get(x_lookup, y_lookup) {
                if neighbor.value <= position.value + 1 {
                    neighbors.push((*neighbor, 1));
                }
            }
        }
        neighbors
    }
    fn print_path(&self, path: &Vec<Position>) {
        // print the board filling each space with a '.' character
        // then print the path from start to finish by filling each space with a '#'
        println!();
        let mut grid = self.positions.clone();
        for row in grid {
            for position in row {
                print!(
                    "{}",
                    match path.contains(&position) {
                        true => '#',
                        false => '.',
                    }
                );
            }
            println!();
        }
        println!();
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Position>>, Position, Position) {
    // The input is a grid of characters, where 'a' = height 1, 'b' = height 2, 'z' = height 26
    // We need to convert this to a grid of positions, where each position has an x, y, and value
    // We also need a special case to note the start and end positions
    // Start is designated with a capital 'S' and has a height of 'a'
    // End is designated with a capital 'E' and has a height of 'z'

    let mut positions: Vec<Vec<Position>> = Vec::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Position> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let value = match c {
                'a'..='z' => c as u32 - 'a' as u32 + 1,
                'S' => {
                    start = Some(Position {
                        x: x as i32,
                        y: y as i32,
                        value: 1,
                    });
                    1
                }
                'E' => {
                    end = Some(Position {
                        x: x as i32,
                        y: y as i32,
                        value: 26,
                    });
                    26
                }
                _ => panic!("Invalid character in input: {}", c),
            };
            row.push(Position {
                x: x as i32,
                y: y as i32,
                value,
            });
        }
        positions.push(row);
    }
    (positions, start.unwrap(), end.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (positions, start, end) = parse_input(input);
    let board = Grid::new(positions);

    let result = astar(
        &start,
        |p| board.neighbors(p),
        |p| ((p.x - end.x).abs() + (p.y - end.y).abs()) as u32,
        |p| *p == end,
    );
    board.print_path(&result.clone().unwrap().0);
    Some(result.unwrap().0.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (positions, _, end) = parse_input(input);
    let board = Grid::new(positions);
    // we need to find all positions with value 1 as possible starting points
    let mut starts = Vec::new();
    for row in board.positions {
        for position in row {
            if position.value == 1 {
                starts.push(position);
            }
        }
    }
    // we need to find which starting position has the shortest path to the end
    let mut shortest = u32::MAX;
    for start in starts {
        let (positions, _, end) = parse_input(input);
        let board = Grid::new(positions);
        let result = astar(
            &start,
            |p| board.neighbors(p),
            |p| ((p.x - end.x).abs() + (p.y - end.y).abs()) as u32,
            |p| *p == end,
        );
        if let Some(result) = result {
            board.print_path(&result.clone().0);
            if result.0.len() as u32 - 1 < shortest {
                shortest = result.0.len() as u32 - 1;
            }
        }
        // get the length only if a result exists
    }
    Some(shortest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
