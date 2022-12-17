use nom::{
    character::complete::{char, satisfy},
    combinator::map,
    sequence::terminated,
    multi::many1,
    IResult,
};

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point(i32, i32);

pub struct Input {
    field: Vec<Vec<u8>>,
    start: Point,
    end: Point,
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    map(
        many1(terminated(
            many1(satisfy(|c| c.is_ascii_alphabetic())),
            char('\n'),
        )),
        |field| {
            let mut start = Point(0, 0);
            let mut end = start.clone();
            for (x, row) in field.iter().enumerate() {
                for (y, c) in row.iter().enumerate() {
                    if *c == 'S' {
                        start = Point(x as i32, y as i32);
                    }
                    if *c == 'E' {
                        end = Point(x as i32, y as i32);
                    }
                }
            }
            let field = field.iter()
                .map(|row| row.iter()
                    .map(|c|
                        if *c == 'S' {
                            0
                        } else if *c == 'E' {
                            25
                        } else {
                            *c as u8 - 'a' as u8
                        }
                    )
                    .collect()
                )
                .collect();
            Input {
                field: field,
                start: start,
                end: end,
            }
        },
    )(input)
}

pub fn part_one(input: &Input) -> Option<u32> {
    let mut visited = HashMap::new();
    visited.insert(input.start, 0);
    let mut queue = VecDeque::new();
    queue.push_back(input.start);
    while !visited.contains_key(&input.end) && !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_dist = *visited.get(&current).unwrap();
        let current_height = input.field[current.0 as usize][current.1 as usize];
        let dps = [Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)];
        for dp in dps {
            let next = Point(current.0 + dp.0, current.1 + dp.1);
            if next.0 < 0 || next.0 >= input.field.len() as i32 ||
                    next.1 < 0 || next.1 >= input.field[0].len() as i32 ||
                    visited.contains_key(&next) {
                continue;
            }
            let next_height = input.field[next.0 as usize][next.1 as usize];
            if current_height as i32 + 1 < next_height as i32 {
                continue;
            }
            queue.push_back(next);
            visited.insert(next, current_dist + 1);
        }
    }
    Some(*visited.get(&input.end).unwrap())
}

pub fn part_two(input: &Input) -> Option<u32> {
    let mut visited = HashMap::new();
    visited.insert(input.end, 0);
    let mut queue = VecDeque::new();
    queue.push_back(input.end);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_dist = *visited.get(&current).unwrap();
        let current_height = input.field[current.0 as usize][current.1 as usize];
        let dps = [Point(-1, 0), Point(0, -1), Point(1, 0), Point(0, 1)];
        for dp in dps {
            let next = Point(current.0 + dp.0, current.1 + dp.1);
            if next.0 < 0 || next.0 >= input.field.len() as i32 ||
                    next.1 < 0 || next.1 >= input.field[0].len() as i32 ||
                    visited.contains_key(&next) {
                continue;
            }
            let next_height = input.field[next.0 as usize][next.1 as usize];
            if next_height as i32 + 1 < current_height as i32 {
                continue;
            }
            let next_dist = current_dist + 1;
            if next_height == 0 {
                return Some(next_dist);
            }
            queue.push_back(next);
            visited.insert(next, next_dist);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 12, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 12, input_parser);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 12, input_parser);
        assert_eq!(part_two(&input), Some(29));
    }
}
