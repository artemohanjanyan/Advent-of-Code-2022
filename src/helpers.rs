/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use nom::{
    error::ParseError,
    IResult,
};

pub fn nom_parser_to_fn<'a, F: 'a, O, E: ParseError<&'a str> + std::fmt::Debug>(parser: F) -> impl Fn(String) -> O
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    move |arg| parser(&arg).expect("parse error").1
}

/*
error[E0597]: `arg` does not live long enough
  --> src/helpers.rs:15:23
   |
11 | pub fn nom_parser_to_fn<'a, F: 'a, O, E: ParseError<&'a str> + std::fmt::Debug>(parser: F) -> impl Fn(String) -> O
   |                         -- lifetime `'a` defined here
...
15 |     move |arg| parser(&arg).expect("parse error").1
   |                -------^^^^-                       - `arg` dropped here while still borrowed
   |                |      |
   |                |      borrowed value does not live long enough
   |                argument requires that `arg` is borrowed for `'a`

*/
