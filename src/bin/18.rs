// thought: enter everything as a graph structure
// two nodes are connected if two of the 3 values are the same and the
// remaining value differs by 1

use petgraph::{
    algo::tarjan_scc,
    dot::{Config, Dot},
    graphmap::UnGraphMap,
};

fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    // input is 3 comma separated numbers per line
    let mut points = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        points.push((x, y, z));
    }
    points
}

fn points_to_graph(points: &[(u32, u32, u32)]) -> UnGraphMap<(u32, u32, u32), ()> {
    let mut graph = UnGraphMap::new();
    // add all nodes
    for point in points {
        graph.add_node(*point);
    }
    for (i, point) in points.iter().enumerate() {
        for other_point in points[i + 1..].iter() {
            if point.0 == other_point.0 && point.1 == other_point.1 && point.2 == other_point.2 {
                continue;
            }
            if (point.0 == other_point.0
                && point.1 == other_point.1
                && (point.2 as i32 - other_point.2 as i32).abs() == 1)
                || (point.0 == other_point.0
                    && point.2 == other_point.2
                    && (point.1 as i32 - other_point.1 as i32).abs() == 1)
                || (point.1 == other_point.1
                    && point.2 == other_point.2
                    && (point.0 as i32 - other_point.0 as i32).abs() == 1)
            {
                graph.add_edge(*point, *other_point, ());
            }
        }
    }
    graph
}

fn find_inverted_graph(graph: &UnGraphMap<(u32, u32, u32), ()>) -> UnGraphMap<(u32, u32, u32), ()> {
    // find the maximum values for the graph 3-tuples
    let (max_x, max_y, max_z) = graph.nodes().map(|node| (node.0, node.1, node.2)).fold(
        (0, 0, 0),
        |(max_x, max_y, max_z), (x, y, z)| {
            (
                if x > max_x { x } else { max_x },
                if y > max_y { y } else { max_y },
                if z > max_z { z } else { max_z },
            )
        },
    );
    // iterate through ranges for all the max values. Check if a node in the original garph exists
    // for that tuple. If it does not, add it to the inverted graph
    let mut inverted_points = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                if !graph.contains_node((x, y, z)) {
                    inverted_points.push((x, y, z));
                }
            }
        }
    }
    points_to_graph(&inverted_points)
}
pub fn part_one(input: &str) -> Option<u32> {
    let points = parse(input);
    let graph = points_to_graph(&points);
    // write the graphviz dotfile to a file
    // std::fs::write(
    //     "graph.dot",
    //     format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])),
    // )
    // .unwrap();
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // iterate over all the nodes, count their edges, and subtract that number from 6,
    // then sum that up
    Some(
        graph
            .nodes()
            .map(|node| 6 - graph.edges(node).count() as u32)
            .sum(),
    )
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    let points = parse(input);
    let graph = points_to_graph(&points);
    let inverted_graph = find_inverted_graph(&graph);
    // println!(
    //     "{:?}",
    //     Dot::with_config(&inverted_graph, &[Config::EdgeNoLabel])
    // );
    // use tarjans algorithm to find the number of connected components
    // let ivg = inverted_graph.into_graph::<u32>();
    let scc = tarjan_scc(&inverted_graph);
    // get components after the first (largest) one
    // let non_outer_component =
    // println!("{:?}", scc);
    // for component in scc.iter() {
    //     println!("sc len: {:?}", component.len());
    // }
    let removals = scc
        .iter()
        .skip(1)
        .map(|component| {
            component
                .iter()
                .map(|node| 6 - inverted_graph.edges(*node).count() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();
    let all_surface = graph
        .nodes()
        .map(|node| 6 - graph.edges(node).count() as u32)
        .sum::<u32>();
    // dbg!(all_surface);
    // dbg!(removals);
    Some(all_surface - removals)
    // None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
