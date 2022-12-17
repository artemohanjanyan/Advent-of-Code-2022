/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use nom::{
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::many1,
    IResult,
};

use std::str::FromStr;

pub fn parse_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

pub fn parse_signed_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(
        many1(one_of("0123456789-")),
        |chars| FromStr::from_str(&chars.iter().collect::<String>()),
    )(input)
}
