use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use glam::IVec2;
use num::integer::lcm;
use petgraph::{
    algo::{astar, dijkstra, has_path_connecting},
    dot::{Config, Dot},
    prelude::DiGraphMap,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match self {
            Up => write!(f, "^"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
        }
    }
}
impl Direction {
    fn unit_vector(&self) -> IVec2 {
        use Direction::*;
        match self {
            Up => IVec2::new(0, -1),
            Down => IVec2::new(0, 1),
            Left => IVec2::new(-1, 0),
            Right => IVec2::new(1, 0),
        }
    }
    fn step(&self, max_x: i32, max_y: i32, position: IVec2) -> IVec2 {
        let mut next_position = position + self.unit_vector();
        // Check if we're at the edge of the map, if we are, wrap around to the
        // other side.
        match &self {
            Direction::Up => {
                if next_position.y < 0 {
                    next_position.y = max_y;
                }
            }
            Direction::Down => {
                if next_position.y > max_y {
                    next_position.y = 0;
                }
            }
            Direction::Left => {
                if next_position.x < 0 {
                    next_position.x = max_x;
                }
            }
            Direction::Right => {
                if next_position.x > max_x {
                    next_position.x = 0;
                }
            }
        }
        next_position
    }
}

struct Blizzards(Vec<(Direction, IVec2)>);
struct Walls(HashSet<IVec2>);

impl Blizzards {
    fn step(&self, walls: &Walls) -> Self {
        use Direction::*;
        let mut new_blizzards: Vec<(Direction, IVec2)> = Vec::new();
        let (max_x, max_y) = walls.0.iter().fold((0, 0), |(max_x, max_y), pos| {
            (max_x.max(pos.x), max_y.max(pos.y))
        });
        for (direction, position) in self.0.iter() {
            // Propose a move to the next step. Check if there's a wall one space
            // in the next direction, if there is, wrap around to the other side.
            let next_position = match direction {
                Up => {
                    let mut proposed_position = direction.step(max_x, max_y, *position);
                    while walls.0.contains(&proposed_position) {
                        proposed_position = direction.step(max_x, max_y, proposed_position);
                    }
                    proposed_position
                }
                Down => {
                    let mut proposed_position = direction.step(max_x, max_y, *position);
                    while walls.0.contains(&proposed_position) {
                        proposed_position = direction.step(max_x, max_y, proposed_position);
                    }
                    proposed_position
                }
                Left => {
                    let mut proposed_position = direction.step(max_x, max_y, *position);
                    while walls.0.contains(&proposed_position) {
                        proposed_position = direction.step(max_x, max_y, proposed_position);
                    }
                    proposed_position
                }
                Right => {
                    let mut proposed_position = direction.step(max_x, max_y, *position);
                    while walls.0.contains(&proposed_position) {
                        proposed_position = direction.step(max_x, max_y, proposed_position);
                    }
                    proposed_position
                }
            };
            new_blizzards.push((*direction, next_position));
        }
        Self(new_blizzards)
    }
}

impl Walls {
    fn boundaries(&self) -> (i32, i32) {
        self.0.iter().fold((0, 0), |(max_x, max_y), pos| {
            (max_x.max(pos.x), max_y.max(pos.y))
        })
    }
}

// Input is either a wall '#' or a blizzard '<', '>', '^', or 'v'.
// Empty space is a '.'
// Example:
// #.######
// #>>.<^<#
// #.<..<<#
// #>v.><>#
// #<^v^^>#
// ######.#
fn parse(input: &str) -> (Blizzards, Walls) {
    let mut blizzards: Vec<(Direction, IVec2)> = Vec::new();
    let mut walls: HashSet<IVec2> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = IVec2::new(x as i32, y as i32);
            match c {
                '#' => {
                    walls.insert(position);
                }
                '>' => {
                    blizzards.push((Direction::Right, position));
                }
                '<' => {
                    blizzards.push((Direction::Left, position));
                }
                '^' => {
                    blizzards.push((Direction::Up, position));
                }
                'v' => {
                    blizzards.push((Direction::Down, position));
                }
                _ => {}
            }
        }
    }
    (Blizzards(blizzards), Walls(walls))
}

// We want to display the state. Represent walls with '#', blizzards with
// '<', '>', '^', or 'v', and empty space with '.'. If multiple blizzards are
// on the same space, display the count of the number of blizzards on that space.
// If a single blizzard is at a position, use the directional representation.
fn display_state(blizzards: &Blizzards, walls: &Walls) {
    let (max_x, max_y) = walls.0.iter().fold((0, 0), |(max_x, max_y), pos| {
        (max_x.max(pos.x), max_y.max(pos.y))
    });
    for y in 0..=max_y {
        for x in 0..=max_x {
            let position = IVec2::new(x, y);
            let is_wall = walls.0.contains(&position);
            let has_blizzards = blizzards.0.iter().any(|(_, pos)| *pos == position);
            if is_wall {
                print!("#");
            } else if has_blizzards {
                let count = blizzards
                    .0
                    .iter()
                    .filter(|(_, pos)| *pos == position)
                    .count();
                if count == 1 {
                    let direction = blizzards
                        .0
                        .iter()
                        .find(|(_, pos)| *pos == position)
                        .unwrap()
                        .0;
                    print!("{}", direction);
                } else {
                    print!("{}", count);
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

struct OpenGraph(DiGraphMap<((i32, i32), u32), ()>);

impl OpenGraph {
    fn add_cycle_nodes(&mut self, blizzards: &Blizzards, walls: &Walls, cycle: u32) {
        let boundaries = walls.boundaries();
        for x in 0..=boundaries.0 {
            for y in 0..=boundaries.1 {
                let position = IVec2::new(x, y);
                if walls.0.contains(&position) {
                    continue;
                }
                if blizzards.0.iter().any(|(_, pos)| *pos == position) {
                    continue;
                }
                self.0.add_node((position.into(), cycle));
            }
        }
    }
    fn finalize_edges(&mut self) {
        let num_cycles = self.0.nodes().map(|(_, cycle)| cycle).max().unwrap();
        for cycle in 0..num_cycles {
            let nodes = self
                .0
                .nodes()
                .filter(|(_, c)| *c == cycle)
                .collect::<Vec<_>>();
            let next_nodes = self
                .0
                .nodes()
                .filter(|(_, c)| *c == cycle + 1)
                .collect::<Vec<_>>();
            for node in nodes {
                for next_node in &next_nodes {
                    let node_pos = node.0;
                    let next_node_pos = next_node.0;
                    if (node_pos.0 - next_node_pos.0).abs() + (node_pos.1 - next_node_pos.1).abs()
                        == 1
                    {
                        self.0.add_edge(node, *next_node, ());
                    }
                }
                // if this position in the next cycle is open, also add that
                // edge
                let same_node_next_cycle = (node.0, cycle + 1);
                if self.0.contains_node(same_node_next_cycle) {
                    self.0.add_edge(node, same_node_next_cycle, ());
                }
            }
        }
        // Repeat for the last cycle, connecting it to cycle 0
        let nodes = self
            .0
            .nodes()
            .filter(|(_, c)| *c == num_cycles)
            .collect::<Vec<_>>();
        let next_nodes = self.0.nodes().filter(|(_, c)| *c == 0).collect::<Vec<_>>();
        for node in nodes {
            for next_node in &next_nodes {
                let node_pos = node.0;
                let next_node_pos = next_node.0;
                if (node_pos.0 - next_node_pos.0).abs() + (node_pos.1 - next_node_pos.1).abs() == 1
                {
                    self.0.add_edge(node, *next_node, ());
                }
            }
            let same_node_next_cycle = (node.0, 0);
            if self.0.contains_node(same_node_next_cycle) {
                self.0.add_edge(node, same_node_next_cycle, ());
            }
        }
        // Remove the 'wait' command from the ending nodes, we wouldn't wait,
        // we're already at the destination
        for node in self.endings() {
            self.0.remove_edge(node, (node.0, node.1 + 1));
            if node.1 == num_cycles {
                self.0.remove_edge(node, (node.0, 0));
            }
        }
        for node in self.starts() {
            self.0.remove_edge(node, (node.0, node.1 + 1));
            if node.1 == num_cycles {
                self.0.remove_edge(node, (node.0, 0));
            }
        }
    }
    fn endings(&self) -> Vec<((i32, i32), u32)> {
        // find all nodes that end in the max y value (second position)
        let max_y = self
            .0
            .nodes()
            .map(|((_, y), _)| y)
            .max()
            .expect("No nodes in graph");
        self.0.nodes().filter(|((_, y), _)| *y == max_y).collect()
    }
    fn starts(&self) -> Vec<((i32, i32), u32)> {
        // find all nodes that end in the max y value (second position)
        let min_y = self
            .0
            .nodes()
            .map(|((_, y), _)| y)
            .min()
            .expect("No nodes in graph");
        self.0.nodes().filter(|((_, y), _)| *y == min_y).collect()
    }
    fn find_solution(&self) -> Option<(i32, Vec<((i32, i32), u32)>)> {
        let mut shortest_path = None;
        let start = self.0.nodes().find(|node| *node == ((1, 0), 0)).unwrap();
        let endings = self.endings();
        let max_cycles = self.0.nodes().map(|(_, cycle)| cycle).max().unwrap();
        if endings.is_empty() {
            return None;
        }
        // grab any ending
        let ending = endings[0];
        // find the manhattan distance between the start position and the ending position
        let min_dist = (start.0 .0 - ending.0 .0).abs() + (start.0 .1 - ending.0 .1).abs();
        let reasonable_endings = endings
            .iter()
            .filter(|ending| ending.1 > min_dist as u32)
            .collect::<Vec<_>>();
        println!("original endings: {:?}", &endings.len());
        println!("reasonable_endings: {:?}", &reasonable_endings.len());
        for (i, ending) in reasonable_endings.iter().enumerate() {
            println!("iter: {}", i);
            let path = astar(
                &self.0,
                start,
                |node| node == **ending,
                |_| 1,
                |node| (node.0 .0 - ending.0 .0).abs() + (node.0 .1 - ending.0 .1).abs(),
            );
            // if a path is found but it's distance is over the max cycles, continue.
            // this is because the path is not valid, it's a loop
            if let Some((distance, _)) = path {
                if distance > max_cycles as i32 {
                    continue;
                }
            }
            // if let Some((distance, path)) = path {
            //     if let Some((shortest_distance, _)) = shortest_path {
            //         if distance < shortest_distance {
            //             shortest_path = Some((distance, path));
            //             println!("shortest path distance: {:?}, iter: {}", distance, i);
            //         }
            //     } else {
            //         shortest_path = Some((distance, path));
            //     }
            // }
            if path.is_some() {
                return path;
            }
        }
        shortest_path
    }
    fn find_solution_part_two(
        &self,
    ) -> (
        Option<(i32, Vec<((i32, i32), u32)>)>,
        Option<(i32, Vec<((i32, i32), u32)>)>,
        Option<(i32, Vec<((i32, i32), u32)>)>,
    ) {
        let start = self.0.nodes().find(|node| *node == ((1, 0), 0)).unwrap();
        let endings = self.endings();
        let max_cycles = self.0.nodes().map(|(_, cycle)| cycle).max().unwrap();

        if endings.is_empty() {
            return (None, None, None);
        }
        // grab any ending
        let ending = endings[0];
        // find the manhattan distance between the start position and the ending position
        let min_dist = (start.0 .0 - ending.0 .0).abs() + (start.0 .1 - ending.0 .1).abs();
        let reasonable_endings = endings
            .iter()
            .filter(|ending| ending.1 > min_dist as u32)
            .collect::<Vec<_>>();
        println!("original endings: {:?}", &endings.len());
        println!("reasonable_endings: {:?}", &reasonable_endings.len());
        let mut chosen_ending = None;
        let mut chosen_path = None;
        for (i, ending) in reasonable_endings.iter().enumerate() {
            if i != 140 {
                continue;
            }
            println!("iter: {}, stage: 1", i);
            let path = astar(
                &self.0,
                start,
                |node| node == **ending,
                |_| 1,
                |node| (node.0 .0 - ending.0 .0).abs() + (node.0 .1 - ending.0 .1).abs(),
            );
            // if let Some((distance, _)) = path {
            //     if distance > max_cycles as i32 {
            //         continue;
            //     }
            // }
            if path.is_some() {
                chosen_ending = Some(**ending);
                chosen_path = path;
                println!(
                    "iter: {}, stage: 1, distance: {:?}, ending: {:?}",
                    i,
                    chosen_path.as_ref().unwrap().0,
                    ending
                );
                break;
            }
        }
        let stage_two_start = chosen_ending.unwrap();
        let stage_two_endings = self.starts();
        println!("start: {:?}", stage_two_start);
        let reasonable_stage_two_endings = stage_two_endings
            .iter()
            .filter(|ending| ending.1 > min_dist as u32)
            .collect::<Vec<_>>();
        let mut chosen_two_ending = None;
        let mut chosen_two_path = None;
        for (i, ending) in reasonable_stage_two_endings.iter().enumerate() {
            println!("iter: {}, stage: 2", i);
            println!("start: {:?}, ending: {:?}", stage_two_start, ending);
            let path = astar(
                &self.0,
                stage_two_start,
                |node| node == **ending,
                |_| 1,
                |node| (node.0 .0 - ending.0 .0).abs() + (node.0 .1 - ending.0 .1).abs(),
            );
            if let Some((distance, _)) = path {
                if distance > max_cycles as i32 {
                    continue;
                }
            }
            if path.is_some() {
                chosen_two_ending = Some(**ending);
                chosen_two_path = path;
                println!(
                    "iter: {}, stage: 2, distance: {:?}, ending: {:?}",
                    i,
                    chosen_two_path.as_ref().unwrap().0,
                    ending
                );
                println!("path: {:?}", chosen_two_path);
                break;
            }
        }
        let stage_three_start = chosen_two_ending.unwrap();

        let stage_three_endings = self.endings();
        let reasonable_stage_three_endings = stage_three_endings
            .iter()
            .filter(|ending| ending.1 > min_dist as u32)
            .collect::<Vec<_>>();
        let mut chosen_three_ending = None;
        let mut chosen_three_path = None;
        for (i, ending) in reasonable_stage_three_endings.iter().enumerate() {
            println!("iter: {}, stage: 3", i);
            let path = astar(
                &self.0,
                stage_three_start,
                |node| node == **ending,
                |_| 1,
                |node| (node.0 .0 - ending.0 .0).abs() + (node.0 .1 - ending.0 .1).abs(),
            );
            if let Some((distance, _)) = path {
                if distance > max_cycles as i32 {
                    continue;
                }
            }
            if path.is_some() {
                chosen_three_ending = Some(**ending);
                chosen_three_path = path;
                break;
            }
        }
        // return ending from each stage

        (chosen_path, chosen_two_path, chosen_three_path)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut blizzards, walls) = parse(input);
    let mut open_graph: OpenGraph = OpenGraph(DiGraphMap::new());
    let boundaries = walls.boundaries();
    // open_graph.add_cycle_nodes(&blizzards, &walls, cycle);
    let num_cycles = lcm(boundaries.0 - 1, boundaries.1 - 1);
    for cycle in 0..num_cycles {
        // println!("Cycle {}", cycle);
        // display_state(&blizzards, &walls);
        open_graph.add_cycle_nodes(&blizzards, &walls, cycle as u32);
        blizzards = blizzards.step(&walls);
    }
    open_graph.finalize_edges();
    // println!(
    //     "{:?}",
    //     Dot::with_config(&open_graph.0, &[Config::EdgeNoLabel])
    // );
    // std::fs::write(
    //     "graph.dot",
    //     format!(
    //         "{:?}",
    //         Dot::with_config(&open_graph.0, &[Config::EdgeNoLabel])
    //     ),
    // )
    // .unwrap();
    let solution = open_graph.find_solution();
    // println!("SOLUTION:\n{:?}", &solution);

    // return the path length from `solution`
    Some(solution.expect("No Solution").0 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut blizzards, walls) = parse(input);
    let mut open_graph: OpenGraph = OpenGraph(DiGraphMap::new());
    let boundaries = walls.boundaries();
    // open_graph.add_cycle_nodes(&blizzards, &walls, cycle);
    // let num_cycles = ((boundaries.0 - 1) * (boundaries.1 - 1)) as u32;
    let num_cycles_lcm = lcm((boundaries.0 - 1), (boundaries.1 - 1));

    for cycle in 0..num_cycles_lcm {
        // println!("Cycle {}", cycle);
        // display_state(&blizzards, &walls);
        open_graph.add_cycle_nodes(&blizzards, &walls, cycle as u32);
        blizzards = blizzards.step(&walls);
    }
    open_graph.finalize_edges();
    // println!(
    //     "{:?}",
    //     Dot::with_config(&open_graph.0, &[Config::EdgeNoLabel])
    // );
    // std::fs::write(
    //     "graph.dot",
    //     format!(
    //         "{:?}",
    //         Dot::with_config(&open_graph.0, &[Config::EdgeNoLabel])
    //     ),
    // )
    // .unwrap();
    let solution = open_graph.find_solution_part_two();
    // println!("SOLUTION:\n{:?}", &solution);

    // return the path length from `solution`
    let mut total_distance = 0;
    for path in vec![
        solution.0.unwrap(),
        solution.1.unwrap(),
        solution.2.unwrap(),
    ]
    .iter()
    {
        total_distance += path.0;
    }
    Some(total_distance as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    // advent_of_code::solve!(1, part_one, input);
    // shortest path distance: 286, iter: 140
    advent_of_code::solve!(2, part_two, input);
    // 986 too high
    // 900 too high
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
