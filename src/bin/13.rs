use std::cmp::Ordering;

use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Collection(Vec<Packet>),
    Value(u32),
}

impl TryFrom<Value> for Packet {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(arr) => {
                let mut packets = Vec::new();
                for item in arr {
                    packets.push(Packet::try_from(item)?);
                }
                Ok(Packet::Collection(packets))
            }
            Value::Number(num) => Ok(Packet::Value(num.as_u64().unwrap() as u32)),
            _ => Err("Invalid packet".to_string()),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
            (Packet::Collection(a), Packet::Collection(b)) => a.cmp(b),
            (Packet::Value(a), Packet::Collection(b)) => {
                Packet::Collection(vec![Packet::Value(*a)]).cmp(&Packet::Collection(b.clone()))
            }
            (Packet::Collection(a), Packet::Value(b)) => a.cmp(&vec![Packet::Value(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ordered_pair_indexes: Vec<u32> = Vec::new();
    for (i, pair) in input.split("\n\n").enumerate() {
        let packet_strs: Vec<&str> = pair.lines().collect();
        let p1_json: Value = serde_json::from_str(packet_strs[0]).unwrap();
        let p2_json: Value = serde_json::from_str(packet_strs[1]).unwrap();
        let p1 = Packet::try_from(p1_json).unwrap();
        let p2 = Packet::try_from(p2_json).unwrap();
        if p1.cmp(&p2) == Ordering::Less {
            ordered_pair_indexes.push((i + 1) as u32);
        }
    }
    Some(ordered_pair_indexes.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all_packets: Vec<Packet> = Vec::new();
    for (i, input_str) in input.split("\n").filter(|l| !l.is_empty()).enumerate() {
        let p1_json: Value = serde_json::from_str(input_str).unwrap();
        let p1 = Packet::try_from(p1_json).unwrap();
        all_packets.push(p1);
    }
    let divider_packets = vec!["[[2]]", "[[6]]"]
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .map(|v: Value| Packet::try_from(v).unwrap())
        .collect::<Vec<Packet>>();
    for packet in divider_packets.iter() {
        all_packets.push(packet.clone());
    }
    all_packets.sort();
    let divider_packet_one_index = all_packets
        .iter()
        .enumerate()
        .find(|(_, p)| p == &&divider_packets[0])
        .unwrap()
        .0
        + 1;
    let divider_packet_two_index = all_packets
        .iter()
        .enumerate()
        .find(|(_, p)| p == &&divider_packets[1])
        .unwrap()
        .0
        + 1;
    Some((divider_packet_one_index * divider_packet_two_index) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
