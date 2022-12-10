use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, char},
    combinator::{map, map_res, value},
    sequence::{preceded, terminated},
    multi::many1,
    IResult,
};

use std::str::FromStr;

#[derive(Clone)]
pub enum Command {
    Noop, Addx(i32)
}

type Input = Vec<Command>;

fn command_parser(input: &str) -> IResult<&str, Command> {
    alt((
        value(Command::Noop, tag("noop")),
        map(
            preceded(
                tag("addx "),
                map_res(
                    many1(one_of("0123456789-")),
                    |chars| FromStr::from_str(&chars.iter().collect::<String>()),
                ),
            ),
            |x| Command::Addx(x),
        ),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        command_parser,
        char('\n'),
    ))(input)
}

fn run(commands: &Input) -> Vec<i32> {
    let mut value = 1;
    let mut values = vec!(value);
    for command in commands {
        match command {
            Command::Noop => values.push(value),
            Command::Addx(x) => {
                values.push(value);
                value += x;
                values.push(value);
            },
        }
    }
    values
}

pub fn part_one(input: &Input) -> Option<i32> {
    let values = run(input);
    Some(values.iter().enumerate()
        .skip(19).step_by(40)
        .map(|(i, x)| (i + 1) as i32 * x)
        .sum()
    )
}

pub fn part_two(input: &Input) -> Option<String> {
    let values = run(input);
    let mut result = vec!();
    for (i, x) in values.iter().enumerate() {
        if i >= 240 {
            break;
        }
        let pos = i as i32 % 40;
        if (pos - x).abs() <= 1 {
            result.push('#');
        } else {
            result.push('.');
        }
        if pos == 39 {
            result.push('\n');
        }
    }
    Some(result.iter().collect())
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 10, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 10, input_parser);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 10, input_parser);
        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(part_two(&input), Some(output.to_owned()));
    }
}
