fn parse_input(input: &str) -> Vec<Vec<u32>> {
    // Each row is a sequence of digits
    // so just parse each digit into a Vec
    // Then collect all of those Vecs
    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    forest
}

fn determine_visible(forest: &Vec<Vec<u32>>) -> u32 {
    let mut visible = 0;
    for (x, row) in forest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            // Iterate in each direction from this tree
            let visible_left = row.iter().rev().skip(row.len() - y).all(|t| tree > t);
            let visible_right = row.iter().skip(y + 1).all(|t| tree > t);
            let visible_down = forest.iter().map(|r| &r[y]).skip(x + 1).all(|t| tree > t);
            let visible_up = forest
                .iter()
                .rev()
                .skip(forest.len() - x)
                .map(|r| &r[y])
                .all(|t| tree > t);
            // dbg!(&visible_left, &visible_right, &visible_down, &visible_up);
            if visible_left || visible_right || visible_down || visible_up {
                visible += 1;
                // let visible = dbg!((x, y, tree));
            } else {
                // let invisible = dbg!((
                //     x,
                //     y,
                //     tree,
                //     visible_left,
                //     visible_right,
                //     forest
                //         .iter()
                //         .map(|r| &r[y])
                //         .skip(x + 1)
                //         .collect::<Vec<&u32>>(),
                //     visible_up
                // ));
            }
        }
    }
    visible
}

fn determine_scores(forest: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut scores = Vec::new();
    for (x, row) in forest.iter().enumerate() {
        let mut row_scores = Vec::new();
        for (y, tree) in row.iter().enumerate() {
            // Iterate in each direction from this tree
            let visible_left: Vec<&u32> = row.iter().rev().skip(row.len() - y).collect();
            // TODO: Implement a flag to tell if we've stopped iterating because we've hit an edge
            // i.e. the iterator is empty
            // OR if we hit a tree that is taller than the current tree
            let n_visible_left = match visible_left.len() {
                0 => 0,
                size => {
                    let it_size = visible_left
                        .iter()
                        .take_while(|t| tree > t)
                        .collect::<Vec<_>>()
                        .len();
                    match it_size == size {
                        true => it_size,
                        false => it_size.overflowing_add(1).0,
                    }
                }
            };
            let visible_right = row.iter().skip(y + 1).collect::<Vec<&u32>>();
            let n_visible_right = match visible_right.len() {
                0 => 0,
                size => {
                    let it_size = visible_right
                        .iter()
                        .take_while(|t| tree > t)
                        .collect::<Vec<_>>()
                        .len();
                    match it_size == size {
                        true => it_size,
                        false => it_size.overflowing_add(1).0,
                    }
                }
            };
            let visible_down = forest
                .iter()
                .map(|r| &r[y])
                .skip(x + 1)
                .collect::<Vec<&u32>>();
            let n_visible_down = match visible_down.len() {
                0 => 0,
                size => {
                    let it_size = visible_down
                        .iter()
                        .take_while(|t| tree > t)
                        .collect::<Vec<_>>()
                        .len();
                    match it_size == size {
                        true => it_size,
                        false => it_size.overflowing_add(1).0,
                    }
                }
            };
            let visible_up = forest
                .iter()
                .rev()
                .skip(forest.len() - x)
                .map(|r| &r[y])
                .collect::<Vec<&u32>>();
            let n_visible_up = match visible_up.len() {
                0 => 0,
                size => {
                    let it_size = visible_up
                        .iter()
                        .take_while(|t| tree > t)
                        .collect::<Vec<_>>()
                        .len();
                    match it_size == size {
                        true => it_size,
                        false => it_size.overflowing_add(1).0,
                    }
                }
            };

            // if x == 3 && y == 2 {
            //     let debug_location = dbg!((x, y, tree));
            //     let debug_score = dbg!(
            //         n_visible_left,
            //         n_visible_right,
            //         n_visible_down,
            //         n_visible_up,
            //         n_visible_left * n_visible_right * n_visible_down * n_visible_up
            //     );
            //     let debug_list = dbg!(visible_left, visible_right, visible_down, visible_up,);
            // }

            let score = (n_visible_left * n_visible_right * n_visible_down * n_visible_up);
            row_scores.push(score as u32);
        }
        scores.push(row_scores);
    }
    scores
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = parse_input(input);
    let visible = determine_visible(&forest);
    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse_input(input);
    let scores = determine_scores(&forest);
    // dbg!(&scores);
    let max_score = scores
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap();
    Some(*max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
