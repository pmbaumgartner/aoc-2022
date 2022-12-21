use std::cmp::Ordering;

use num::zero;

fn wrap_vec(vec: Vec<(u32, i64)>, index: usize) -> Vec<(u32, i64)> {
    let (current_index, (_, value)) = vec
        .iter()
        .enumerate()
        .find(|(_, (i, _))| *i as usize == index)
        .unwrap();
    let mut new_vec = vec.clone();
    let item = new_vec.remove(current_index);
    let mut new_index = (current_index as i64 + value).rem_euclid(vec.len() as i64 - 1);
    if new_index == 0 {
        new_index = vec.len() as i64 - 1;
    }
    // let new_index_position = match (new_index.cmp(&(current_index as i64)), new_index.cmp(&0)) {
    //     (Ordering::Less, Ordering::Less) => vec.len() as i64 + new_index,
    //     (Ordering::Less, Ordering::Greater) => new_index,
    //     (Ordering::Greater, Ordering::Less) => vec.len() as i64 + new_index - 1,
    //     (Ordering::Greater, Ordering::Greater) => new_index - 1,
    //     (_, _) => new_index,
    // };

    new_vec.insert(new_index as usize, item);
    new_vec
}

fn parse_input(input: &str) -> Vec<(u32, i64)> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .map(|(index, value)| (index as u32, value))
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let coordinates = parse_input(input);
    let mut vec = coordinates.clone();
    for index in 0..vec.len() {
        vec = wrap_vec(vec, index);
        // dbg!(&coordinates[index], &vec);
    }
    let zero_index = vec
        .iter()
        .enumerate()
        .find(|(_, (_, v))| *v == 0)
        .unwrap()
        .0;
    let coordinate_sum: Vec<i64> = vec![1000, 2000, 3000]
        .iter()
        .map(|x: &u32| (x + zero_index as u32).rem_euclid(coordinates.len() as u32))
        .map(|x| vec.get(x as usize).unwrap())
        .map(|(_, x)| *x)
        .collect::<Vec<_>>();
    // dbg!(&coordinate_sum, &zero_index);
    Some(coordinate_sum.iter().sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let decryption_key = 811589153;
    let n_mixes = 10;
    let mut coordinates = parse_input(input);
    let coordinates = coordinates
        .iter()
        .map(|(i, x)| (*i, *x * decryption_key))
        .collect::<Vec<_>>();
    let mut vec = coordinates.clone();
    for n in 0..n_mixes {
        for index in 0..vec.len() {
            vec = wrap_vec(vec, index);
            // dbg!(&coordinates[index], &vec);
        }
    }
    let zero_index = vec
        .iter()
        .enumerate()
        .find(|(_, (_, v))| *v == 0)
        .unwrap()
        .0;
    let coordinate_sum: Vec<i64> = vec![1000, 2000, 3000]
        .iter()
        .map(|x: &u32| (x + zero_index as u32).rem_euclid(coordinates.len() as u32))
        .map(|x| vec.get(x as usize).unwrap())
        .map(|(_, x)| *x)
        .collect::<Vec<_>>();
    // dbg!(&coordinate_sum, &zero_index);
    Some(coordinate_sum.iter().sum::<i64>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
