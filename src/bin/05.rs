use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{map, map_res, value, verify},
    sequence::{delimited, separated_pair, terminated, tuple},
    multi::{many1, separated_list1},
    IResult,
};

use std::str::FromStr;

type Crate = char;

#[derive(Clone, Debug)]
pub struct Command {
    amount: u32,
    from: u32,
    to: u32,
}

#[derive(Clone, Debug)]
pub struct Input {
    stacks: Vec<Vec<Crate>>,
    commands: Vec<Command>,
}

fn crate_parser(input: &str) -> IResult<&str, Option<Crate>> {
    alt((
        map(delimited(char('['), verify(anychar, |&c| c.is_ascii_uppercase()), char(']')), Some),
        value(None, tag("   ")),
    ))(input)
}

fn transpose_filter<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().rev().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn stacks_parser(input: &str) -> IResult<&str, Vec<Vec<Crate>>> {
    map(
        terminated(
            many1(terminated(
                separated_list1(char(' '), crate_parser),
                char('\n'),
            )),
            terminated(
                separated_list1(
                    char(' '),
                    delimited(char(' '), verify(anychar, |&c| c.is_ascii_digit()), char(' ')),
                ),
                char('\n'),
            ),
        ),
        transpose_filter,
    )(input)
}

fn u32_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn command_parser(input: &str) -> IResult<&str, Command> {
    map(
        tuple((
            tag("move "),
            u32_parser,
            tag(" from "),
            u32_parser,
            tag(" to "),
            u32_parser,
        )),
        |(_move, a, _from, b, _to, c)| Command { amount: a, from: b, to: c },
    )(input)
}

fn commands_parser(input: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(
        command_parser,
        char('\n'),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    map(
        separated_pair(
            stacks_parser,
            char('\n'),
            commands_parser,
        ),
        |(stacks, commands)| Input { stacks: stacks, commands: commands },
    )(input)
}

fn run_command(stacks: &mut Vec<Vec<Crate>>, command: &Command) {
    for _ in 0..command.amount {
        let current_crate: char = *stacks[(command.from - 1) as usize].last().unwrap();
        stacks[(command.to - 1) as usize].push(current_crate);
        stacks[(command.from - 1) as usize].pop();
    }
}

fn run_command_2(stacks: &mut Vec<Vec<Crate>>, command: &Command) {
    let from_stack_len = stacks[(command.from - 1) as usize].len();
    for current_crate_i in from_stack_len - command.amount as usize.. from_stack_len {
        let current_crate = stacks[(command.from - 1) as usize][current_crate_i];
        stacks[(command.to - 1) as usize].push(current_crate);
    }
    for _ in 0..command.amount {
        stacks[(command.from - 1) as usize].pop();
    }
}

impl Input {
    fn run_commands(&mut self) {
        for command in &self.commands {
            run_command(&mut self.stacks, command);
        }
    }

    fn run_commands_2(&mut self) {
        for command in &self.commands {
            run_command_2(&mut self.stacks, command);
        }
    }

    fn collect_tops(&self) -> String {
        self.stacks.iter()
            .map(|s| s.last().unwrap())
            .collect()
    }
}

pub fn part_one(input_ref: &Input) -> Option<String> {
    let input = &mut input_ref.clone();
    input.run_commands();
    Some(input.collect_tops())
}

pub fn part_two(input_ref: &Input) -> Option<String> {
    let input = &mut input_ref.clone();
    input.run_commands_2();
    Some(input.collect_tops())
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 5, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 5, input_parser);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 5, input_parser);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
