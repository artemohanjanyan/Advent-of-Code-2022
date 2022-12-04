use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
};

use std::str::FromStr;

type Input = Vec<Vec<u32>>;

pub fn input_parser(input: &str) -> IResult<&str, Input> {
    separated_list1(
        char('\n'),
        separated_list0(
            char('\n'),
            map_res(digit1, FromStr::from_str),
        ),
    )(input)
}

pub fn input_parser_q(input: String) -> Input {
    input_parser(&input).expect("could not parse input file").1
}

pub fn part_one(input: &Input) -> Option<u32> {
    input.iter()
        .map(|elf| elf.iter().sum())
        .max()
}

pub fn part_two(input: &Input) -> Option<u32> {
    let mut sums: Vec<u32> = input.iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    Some(sums[0] + sums[1] + sums[2])
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 1, input_parser_q);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 1, input_parser_q);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 1, input_parser_q);
        assert_eq!(part_two(&input), Some(45000));
    }
}
