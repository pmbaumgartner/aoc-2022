use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(u32, String),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => Ok(Line::Cd(parts[2].to_string())),
                "ls" => Ok(Line::Ls),
                _ => Err("Invalid command".to_string()),
            },
            "dir" => Ok(Line::Dir(parts[1].to_string())),
            // Match a string that's all digits
            digits if digits.chars().all(|c| c.is_digit(10)) => {
                let size = digits.parse::<u32>().unwrap();
                Ok(Line::File(size, parts[1].to_string()))
            }
            _ => Err("Invalid command".to_string()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().skip(1).map(|l| l.parse::<Line>().unwrap());
    let mut sizes: HashMap<PathBuf, u32> = HashMap::new();
    let root = PathBuf::from_str("/").unwrap();
    let mut current_dir: PathBuf = root;
    sizes.entry(current_dir.clone()).or_insert(0);
    for line in lines {
        match line {
            Line::Cd(dir) => match dir.as_str() {
                ".." => {
                    let new_dir: PathBuf = current_dir.parent().unwrap().to_path_buf();
                    current_dir = new_dir;
                }
                _ => {
                    let new_dir = current_dir.join(dir);
                    current_dir = new_dir;
                }
            },
            Line::Ls => {
                // We don't need to do anything on the line we run the `ls` command
            }
            Line::Dir(dir) => {
                // assume we always `cd` into a dir eventually, so we don't need to do anything when we see this command
            }
            Line::File(size, file) => {
                *sizes.entry(current_dir.clone()).or_insert(0) += size;
            }
        }
    }
    let mut recursive_sizes: HashMap<PathBuf, u32> = HashMap::new();
    for (path, size) in sizes.iter() {
        // We want to add the size of the current dir to the size of all of its parents
        for path in path.ancestors() {
            *recursive_sizes.entry(path.to_path_buf()).or_insert(0) += size;
        }
    }

    // Find entries that have a size of less than 100000, and sum their sizes
    let mut total = 0;
    for (path, size) in recursive_sizes.iter() {
        if *size < 100000 {
            total += size;
        }
    }

    // dbg!(recursive_sizes);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    const TOTAL_SPACE: u32 = 70000000;
    const NEEDED_UNUSED: u32 = 30000000;
    let lines = input.lines().skip(1).map(|l| l.parse::<Line>().unwrap());
    let mut sizes: HashMap<PathBuf, u32> = HashMap::new();
    let root = PathBuf::from_str("/").unwrap();
    let mut current_dir: PathBuf = root.clone();
    sizes.entry(current_dir.clone()).or_insert(0);
    for line in lines {
        match line {
            Line::Cd(dir) => match dir.as_str() {
                ".." => {
                    let new_dir: PathBuf = current_dir.parent().unwrap().to_path_buf();
                    current_dir = new_dir;
                }
                _ => {
                    let new_dir = current_dir.join(dir);
                    current_dir = new_dir;
                }
            },
            Line::Ls => {
                // We don't need to do anything on the line we run the `ls` command
            }
            Line::Dir(dir) => {
                // assume we always `cd` into a dir eventually, so we don't need to do anything when we see this command
            }
            Line::File(size, file) => {
                *sizes.entry(current_dir.clone()).or_insert(0) += size;
            }
        }
    }
    let mut recursive_sizes: HashMap<PathBuf, u32> = HashMap::new();
    for (path, size) in sizes.iter() {
        // We want to add the size of the current dir to the size of all of its parents
        for path in path.ancestors() {
            *recursive_sizes.entry(path.to_path_buf()).or_insert(0) += size;
        }
    }

    let need_to_free = NEEDED_UNUSED - (TOTAL_SPACE - recursive_sizes.get(&root).unwrap());
    // Collect the paths and sort them by size
    let mut paths_by_size = recursive_sizes
        .iter()
        .map(|(path, size)| (path, size))
        .collect::<Vec<(&PathBuf, &u32)>>();
    paths_by_size.sort_by(|(_, size1), (_, size2)| size1.cmp(size2));
    dbg!(&paths_by_size);
    let (_delete_path, delete_size) = paths_by_size
        .iter()
        .find(|(_, size)| size > &&need_to_free)
        .unwrap();

    // Return the size of the path we need to delete
    Some(**delete_size)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
