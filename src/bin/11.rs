use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value},
    sequence::{preceded, delimited, terminated, tuple},
    multi::separated_list1,
    IResult,
};

use advent_of_code::helpers::parse_int;

#[derive(Clone)]
enum Arg {
    Const(i64), OldValue
}

#[derive(Clone)]
enum Operator {
    Add, Mul
}

struct Operation { 
    arg1: Arg,
    arg2: Arg,
    operator: Operator,
}

fn parse_arg(input: &str) -> IResult<&str, Arg> {
    alt((
        value(Arg::OldValue, tag("old")),
        map(parse_int, |x| Arg::Const(x)),
    ))(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Add, char('+')),
        value(Operator::Mul, char('*')),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((
            terminated(parse_arg, char(' ')),
            terminated(parse_operator, char(' ')),
            parse_arg,
        )),
        |(arg1, operator, arg2)| Operation {
            arg1: arg1,
            arg2: arg2,
            operator: operator,
        },
    )(input)
}

pub struct MonkeyDescription {
    operation: Operation,
    test: i64,
    throw_if_true: i64,
    throw_if_false: i64,
}

pub struct Monkey {
    items: Vec<i64>,
    description: MonkeyDescription,
}

fn parse_items(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(
        tag(", "),
        parse_int,
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    preceded(
        delimited(tag("Monkey "), parse_int::<i64>, tag(":\n")),
        map(
            tuple((
                delimited(tag("  Starting items: "), parse_items, char('\n')),
                delimited(tag("  Operation: new = "), parse_operation, char('\n')),
                delimited(tag("  Test: divisible by "), parse_int::<i64>, char('\n')),
                delimited(tag("    If true: throw to monkey "), parse_int::<i64>, char('\n')),
                delimited(tag("    If false: throw to monkey "), parse_int::<i64>, char('\n')),
            )),
            |(items, operation, test, throw_if_true, throw_if_false)| Monkey {
                items: items,
                description: MonkeyDescription {
                    operation: operation,
                    test: test,
                    throw_if_true: throw_if_true,
                    throw_if_false: throw_if_false,
                },
            },
        ),
    )(input)
}

type Input = Vec<Monkey>;

fn input_parser(input: &str) -> IResult<&str, Input> {
    separated_list1(
        char('\n'),
        parse_monkey,
    )(input)
}

impl Arg {
    fn get(&self, old: i64) -> i64 {
        match self {
            Arg::Const(x) => *x,
            Arg::OldValue => old,
        }
    }
}

impl Operator {
    fn run(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }
}

impl Operation {
    fn run(&self, old: i64) -> i64 {
        self.operator.run(self.arg1.get(old), self.arg2.get(old))
    }
}

pub fn part_one(input: &Input) -> Option<i64> {
    let mut items: Vec<Vec<i64>> = vec![vec!(); input.len()];
    for (i, monkey) in input.iter().enumerate() {
        for item in &monkey.items {
            items[i].push(*item);
        }
    }
    let mut op_count = vec![0; input.len()];
    for _ in 0..20 {
        for (i, monkey) in input.iter().enumerate() {
            let description = &monkey.description;
            let mut items_i = vec!();
            std::mem::swap(&mut items_i, &mut items[i]);
            for item in items_i {
                op_count[i] += 1;
                let new_item = description.operation.run(item) / 3;
                if new_item % description.test == 0 {
                    items[description.throw_if_true as usize].push(new_item);
                } else {
                    items[description.throw_if_false as usize].push(new_item);
                }
            }
            items[i].clear();
        }
    }
    op_count.sort_by(|a, b| b.cmp(a));
    Some(op_count[0] * op_count[1])
}

pub fn part_two(input: &Input) -> Option<i64> {
    let mut items: Vec<Vec<i64>> = vec![vec!(); input.len()];
    for (i, monkey) in input.iter().enumerate() {
        for item in &monkey.items {
            items[i].push(*item);
        }
    }
    let mut op_count = vec![0; input.len()];
    let not_lcm: i64 = input
        .iter()
        .map(|monkey| monkey.description.test)
        .product();
    for _ in 0..10000 {
        for (i, monkey) in input.iter().enumerate() {
            let description = &monkey.description;
            let mut items_i = vec!();
            std::mem::swap(&mut items_i, &mut items[i]);
            for item in items_i {
                op_count[i] += 1;
                let new_item = description.operation.run(item) % not_lcm;
                if new_item % description.test == 0 {
                    items[description.throw_if_true as usize].push(new_item);
                } else {
                    items[description.throw_if_false as usize].push(new_item);
                }
            }
            items[i].clear();
        }
    }
    op_count.sort_by(|a, b| b.cmp(a));
    Some(op_count[0] * op_count[1])
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 11, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 11, input_parser);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 11, input_parser);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
