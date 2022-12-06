use nom::{
    character::complete::{char, alpha1},
    combinator::map,
    sequence::terminated,
    IResult,
};

use std::collections::HashSet;

type Input = Vec<char>;

fn input_parser(input: &str) -> IResult<&str, Input> {
    map(
        terminated(
            alpha1,
            char('\n'),
        ),
        |s: &str| s.chars().collect(),
    )(input)
}

pub fn part_one(input: &Input) -> Option<usize> {
    for i in 0..input.len() {
        if input[i] != input[i + 1] && input[i] != input[i + 2] && input[i] != input[i + 3] &&
                input[i + 1] != input[i + 2] && input[i + 1] != input[i + 3] &&
                input[i + 2] != input[i + 3] {
            return Some(i + 4)
        }
    }
    None
}

pub fn part_two(input: &Input) -> Option<usize> {
    for i in 0..input.len() {
        if HashSet::<&char>::from_iter(input[i..i + 14].iter()).len() == 14 {
            return Some(i + 14)
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 6, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 6, input_parser);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 6, input_parser);
        assert_eq!(part_two(&input), Some(19));
    }
}
