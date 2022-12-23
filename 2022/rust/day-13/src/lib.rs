use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Ord for Packet {
    /// recursivly call self.cmp until you have 2 integers. Returns that ordering
    /// ([1], [1]) would return Ordering::Equal
    /// ([1, 2], [3]) would return Ordering::Less
    /// ([1], [0]) would return Ordering::Greater
    /// this also allows to nest lists: eg:
    /// [1, [2, 3]], [1, [4, 5]] would return Ordering::Less
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Integer(b)) => a.cmp(&vec![Self::Integer(*b)]),
            (Self::Integer(a), Self::List(b)) => vec![Self::Integer(*a)].cmp(b),
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn parse_integer(single: &str) -> IResult<&str, Packet> {
        map(u32, Packet::Integer)(single)
    }

    /// single == "[1, 3, 4, [1, 2, 3]]"
    fn parse_list(single: &str) -> IResult<&str, Packet> {
        let list_parser = delimited(
            tag("["),
            separated_list0(tag(","), Self::parse_packets),
            tag("]"),
        );
        map(list_parser, Packet::List)(single)
    }

    /// parses either "8" or "[8]" to Packet
    fn parse_packets(single: &str) -> IResult<&str, Packet> {
        alt((Self::parse_integer, Self::parse_list))(single)
    }

    fn from_str(pair: &str) -> Self {
        let (left, right) = pair.split_once('\n').unwrap();

        Self {
            left: Self::parse_packets(left).unwrap().1,
            right: Self::parse_packets(right).unwrap().1,
        }
    }
}

pub fn parse_packet_pairs(file: String) -> Vec<PacketPair> {
    file.split("\n\n").map(PacketPair::from_str).collect()
}

pub fn parse_packets(file: String) -> Vec<Packet> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| PacketPair::parse_packets(line).unwrap().1)
        .collect()
}

pub fn process_input1(file: String) -> usize {
    let pairs = parse_packet_pairs(file);

    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.left <= pair.right)
        .map(|(index, _)| index + 1)
        .sum()
}

pub fn process_input2(file: String) -> usize {
    let mut packets: Vec<_> = parse_packets(file);

    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet == divider_6 || **packet == divider_2)
        .map(|(index, _)| index + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("test.txt");
        assert_eq!(process_input1(file.to_string()), 13);
    }

    #[test]
    fn part2() {
        let file = include_str!("test.txt");
        assert_eq!(process_input2(file.to_string()), 140);
    }
}
