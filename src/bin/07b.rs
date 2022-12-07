// This is an old, convoluted solution I tried to work at initially for Day 7
// I'm keeping it here for posterity, but it's not used in the final solution

use std::str::FromStr;

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
    fn size(&self) -> usize {
        self.arena.len()
    }
    fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
        // are we here?  If so, Some(0)
        if target == &self.arena[idx].val {
            return Some(0);
        }

        // If not, try all children
        for p in &self.arena[idx].children {
            if let Some(x) = self.depth_to_target(*p, &target) {
                return Some(1 + x);
            }
        }
        // If it cant be found, return None
        None
    }
    fn distance_between(&mut self, from: T, target: T) -> usize {
        // If it's not in the tree, this will add a new unconnected node
        // the final function will still return None
        let start_node = self.node(from);
        let mut ret = 0;
        // Start traversal
        let mut trav = &self.arena[start_node];
        // Explore all children, then hop up one
        while let Some(inner) = trav.parent {
            if let Some(x) = self.depth_to_target(inner, &target) {
                ret += x;
                break;
            }
            trav = &self.arena[inner];
            ret += 1;
        }
        // don't go all the way to target, just orbit
        if ret > 0 {
            ret - 1
        } else {
            ret
        }
    }
}

enum Line {
    Command(Command),
    Object(Object),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_input = s.split_whitespace();
        match split_input.next() {
            Some("$") => Ok(Line::Command(s.parse().unwrap())),
            Some("dir") => Ok(Line::Object(s.parse().unwrap())),
            // Idiomatically and concicely check if the first character is a digit
            Some(s) if s.chars().next().unwrap().is_digit(10) => {
                Ok(Line::Object(s.parse().unwrap()))
            }
            _ => Err("Invalid line".to_string()),
        }
    }
}
enum Command {
    CD(String),
    LS,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[1..].split_whitespace();
        match parts.next() {
            Some("cd") => Ok(Command::CD(parts.next().unwrap().to_string())),
            Some("ls") => Ok(Command::LS),
            _ => Err("Invalid command".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Default)]

struct Dir {
    name: String,
    parent: Option<Box<Dir>>,
    children: Vec<Object>,
}

// directories appear as 'dir <name>'
// And we don't know their children at the time of parsing
impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("dir") => Ok(Dir {
                name: parts.next().unwrap().to_string(),
                parent: None,
                children: Vec::new(),
            }),
            _ => Err("Invalid dir command".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
struct File {
    name: String,
    size: u32,
}

// Files appear as '<size> <name>'
impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        dbg!(&parts);
        if parts.len() == 2 {
            let size = parts[0].parse::<u32>().unwrap();
            let name = parts[1].to_string();
            Ok(File { name, size })
        } else {
            Err("Invalid file command".to_string())
        }
    }
}
#[derive(Debug, PartialEq)]
enum Object {
    Dir(Dir),
    File(File),
}

impl Default for Object {
    fn default() -> Self {
        Object::Dir(Dir::default())
    }
}

impl FromStr for Object {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().next() {
            Some("dir") => Ok(Object::Dir(s.parse().unwrap())),
            Some(s) if s.chars().next().unwrap().is_digit(10) => {
                Ok(Object::File(s.parse().unwrap()))
            }
            _ => Err("Invalid object".to_string()),
        }
    }
}

/* Our input is a series of commands and outputs from a terminal. We need to
determine the file and folder structure of our filesystem by parsing the lines and
storing the correct objects. We need to do this by keeping track of the current directory,
which can change with a 'cd' command, and then document which files or directories are in the current
directory when we see an 'ls' command. We should store this in a tree-like data structure that is
returned from this function.

For example, here's some input:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
<EOF>

Return these objects in a tree-like data structure so that we can traverse it
and find the size of the files in each directory.
*/
fn parse_input(input: &str) {
    let root = Object::Dir(Dir {
        name: "/".to_string(),
        parent: None,
        children: Vec::new(),
    });
    let mut current_dir = &root;
    let mut tree: ArenaTree<Object> = ArenaTree::default();
    let root_node = tree.node(root);
    for line in input.lines().skip(1) {
        match line.parse::<Line>() {
            Ok(Line::Command(Command::CD(dir))) => {
                println!("cd {}", dir)
            }
            Ok(Line::Command(Command::LS)) => println!("ls"),
            Ok(Line::Object(object)) => println!("{:?}", object),
            Err(e) => panic!("Invalid line: {}", e),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse_input(input);
    dbg!(tree);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), ());
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
