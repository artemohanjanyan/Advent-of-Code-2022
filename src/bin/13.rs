use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use advent_of_code::helpers::int_parser;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    Integer(i32), List(Vec<Packet>)
}

type Input = Vec<(Packet, Packet)>;

fn packet_parser(input: &str) -> IResult<&str, Packet> {
    alt((
        map(int_parser, Packet::Integer),
        map(
            delimited(
                char('['),
                separated_list0(
                    char(','),
                    packet_parser,
                ),
                char(']'),
            ),
            Packet::List,
        ),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    separated_list1(
        char('\n'),
        separated_pair(packet_parser, char('\n'), terminated(packet_parser, char('\n'))),
    )(input)
}

fn cmp_packets(packet1: &Packet, packet2: &Packet) -> Ordering {
    match (packet1, packet2) {
        (Packet::Integer(n1), Packet::Integer(n2)) => {
            n1.cmp(n2)
        }
        (Packet::List(packets1), Packet::List(packets2)) => {
            let mut i = 0;
            let mut res = Ordering::Equal;
            while i < packets1.len() && i < packets2.len() {
                let res_i = cmp_packets(&packets1[i], &packets2[i]);
                if res_i != Ordering::Equal {
                    res = res_i;
                    break;
                }
                i += 1;
            }
            if res != Ordering::Equal || packets1.len() == packets2.len() {
                res
            } else {
                packets1.len().cmp(&packets2.len())
            }
        }
        (Packet::Integer(_), _) => {
            cmp_packets(&Packet::List(vec!(packet1.clone())), packet2)
        }
        (_, Packet::Integer(_)) => {
            cmp_packets(packet1, &Packet::List(vec!(packet2.clone())))
        }
    }
}

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        input.iter().enumerate()
            .map(|(i, (p1, p2))|
                 (i as u32 + 1) * (cmp_packets(p1, p2) == Ordering::Less) as u32
             )
            .sum()
    )
}

pub fn part_two(input: &Input) -> Option<u32> {
    let mut packets: Vec<&Packet> = input.iter()
        .map(|(p1, p2)| [p1, p2])
        .flatten()
        .collect();
    let div2 = Packet::List(vec!(Packet::Integer(2)));
    let div6 = Packet::List(vec!(Packet::Integer(6)));
    packets.push(&div2);
    packets.push(&div6);
    packets.sort_by(|a, b| cmp_packets(a, b));
    let mut result = 1;
    for (i, &p) in packets.iter().enumerate() {
        if *p == div2 || *p == div6 {
            result *= i + 1;
        }
    }
    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 13, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 13, input_parser);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 13, input_parser);
        assert_eq!(part_two(&input), Some(140));
    }
}
