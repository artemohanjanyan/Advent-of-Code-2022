use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{terminated, tuple},
    multi::many1,
    IResult,
};

use std::str::FromStr;

type Assignment = (u32, u32);

type Input = Vec<(Assignment, Assignment)>;

fn num_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn assignment_parser(input: &str) -> IResult<&str, Assignment> {
    tuple((
        terminated(num_parser, char('-')),
        num_parser,
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        tuple((
            terminated(assignment_parser, char(',')),
            assignment_parser,
        )),
        char('\n'),
    ))(input)
}

pub fn part_one(input: &Input) -> Option<u32> {
    let mut ans = 0;
    for &((a, b), (c, d)) in input.iter() {
        if a <= c && b >= d || c <= a && d >= b {
            ans += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &Input) -> Option<u32> {
    let mut ans = 0;
    for &((a, b), (c, d)) in input.iter() {
        if a <= c && c <= b || a <= d && d <= b || a < c && d < b || c < a && b < d {
            ans += 1;
        }
    }
    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 4, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 4, input_parser);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 4, input_parser);
        assert_eq!(part_two(&input), Some(4));
    }
}
