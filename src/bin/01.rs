use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
};

use std::str::FromStr;

pub fn input_parser(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        char('\n'),
        separated_list0(
            char('\n'),
            map_res(digit1, FromStr::from_str),
        ),
    )(input)
}

pub fn part_one(input_str: &str) -> Option<u32> {
    let (_rest, input) = input_parser(input_str).ok()?;
    input.iter()
        .map(|elf| elf.iter().sum())
        .max()
}

pub fn part_two(input_str: &str) -> Option<u32> {
    let (_rest, input) = input_parser(input_str).ok()?;
    let mut sums: Vec<u32> = input.iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    Some(sums[0] + sums[1] + sums[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
