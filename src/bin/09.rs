use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    sequence::{separated_pair, terminated},
    multi::many1,
    IResult,
};

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone)]
enum Direction { Up, Down, Left, Right }

pub struct Step {
    direction: Direction,
    count: u32,
}

pub type Input = Vec<Step>;

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        map(
            separated_pair(
                direction_parser,
                char(' '),
                map_res(digit1, FromStr::from_str),
            ),
            |(direction, count)| Step {
                direction: direction,
                count: count,
            },
        ),
        char('\n'),
    ))(input)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn go(&mut self, direction: &Direction) {
        match *direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow_y(&mut self, head: &Point) {
        if self.y > head.y {
            self.go(&Direction::Down);
        } else if self.y < head.y {
            self.go(&Direction::Up);
        }
    }

    fn follow(&mut self, head: &Point) {
        if self.x == head.x {
            self.follow_y(head)
        } else {
            if self.x > head.x {
                self.go(&Direction::Left);
            } else {
                self.go(&Direction::Right);
            };
            self.follow_y(head);
        }
    }
}

fn dist(a: &Point, b: &Point) -> i32 {
    std::cmp::max((a.x - b.x).abs(), (a.y - b.y).abs())
}

struct Rope {
    points: Vec<Point>
}

impl Rope {
    fn go(&mut self, direction: &Direction) {
        self.points[0].go(direction);
        for i in 1..self.points.len() {
            if dist(&self.points[i - 1], &self.points[i]) > 1 {
                let point_to_follow = self.points[i - 1];
                self.points[i].follow(&point_to_follow);
            }
        }
    }
}

pub fn part_one(input: &Input) -> Option<usize> {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited_points = HashSet::new();
    visited_points.insert(tail);
    for step in input {
        for _ in 0..step.count {
            head.go(&step.direction);
            if dist(&head, &tail) > 1 {
                tail.follow(&head);
            }
            visited_points.insert(tail);
        }
    }
    Some(visited_points.len())
}

pub fn part_two(input: &Input) -> Option<usize> {
    let mut rope = Rope { points: vec![Point { x: 0, y: 0 }; 10] };
    let mut visited_points = HashSet::new();
    visited_points.insert(rope.points.last().unwrap().clone());
    for step in input {
        for _ in 0..step.count {
            rope.go(&step.direction);
            visited_points.insert(rope.points.last().unwrap().clone());
        }
    }
    Some(visited_points.len())
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 9, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 9, input_parser);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 9, input_parser);
        assert_eq!(part_two(&input), Some(1));
    }
}
