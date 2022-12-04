use nom::{
    branch::alt,
    character::complete::char,
    combinator::value,
    sequence::{terminated, tuple},
    multi::many1,
    IResult,
};

fn abc_parser(input: &str) -> IResult<&str, i32> {
    alt((
        value(1, char('A')),
        value(2, char('B')),
        value(3, char('C')),
    ))(input)
}

fn xyz_parser(input: &str) -> IResult<&str, i32> {
    alt((
        value(1, char('X')),
        value(2, char('Y')),
        value(3, char('Z')),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(terminated(
        tuple((
            terminated(abc_parser, char(' ')),
            xyz_parser,
        )),
        char('\n'),
    ))(input)
}

fn result(a: i32, b: i32) -> i32 {
    ((b - a + 3) % 3 + 1) % 3 * 3 + b
}

fn result2(a: i32, b: i32) -> i32 {
    result(a, (a + (b - 2) - 1 + 3) % 3 + 1)
}

pub fn part_one(input_str: &str) -> Option<i32> {
    let (_rest, input) = input_parser(input_str).ok()?;
    let mut sum = 0;
    for &(a, b) in input.iter() {
        sum += result(a, b)
    }
    Some(sum)
}

pub fn part_two(input_str: &str) -> Option<i32> {
    let (_rest, input) = input_parser(input_str).ok()?;
    let mut sum = 0;
    for &(a, b) in input.iter() {
        sum += result2(a, b)
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
