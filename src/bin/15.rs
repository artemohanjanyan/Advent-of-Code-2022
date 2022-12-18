use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{terminated, tuple},
    multi::many1,
    IResult,
};

use advent_of_code::helpers::signed_int_parser;

#[derive(PartialEq, Eq, Debug)]
pub struct Point(i32, i32);

impl Point {
    fn dist(&self, that: &Point) -> i32 {
        (self.0 - that.0).abs() + (self.1 - that.1).abs()
    }

    fn tuning_frequency(&self) -> i64 {
        self.0 as i64 * 4_000_000 + self.1 as i64
    }
}

type Input = Vec<(Point, Point)>;

fn sensor_parser(input: &str) -> IResult<&str, (Point, Point)> {
    map(
        tuple((
            tag("Sensor at x="),
            signed_int_parser,
            tag(", y="),
            signed_int_parser,
            tag(": closest beacon is at x="),
            signed_int_parser,
            tag(", y="),
            signed_int_parser,
        )),
        |(_, x1, _, y1, _, x2, _, y2)| (Point(x1, y1), Point(x2, y2)),
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        sensor_parser,
        char('\n'),
    ))(input)
}

pub fn part_one(input: &Input) -> Option<u32> {
    let y = if input.len() < 20 { 10 } else { 2_000_000 };
    let mut ans = 0;
    'outer: for x in -10_000_000..10_000_000 {
        let current = Point(x, y);
        for (_, beacon) in input {
            if current == *beacon {
                continue 'outer;
            }
        }
        for (sensor, beacon) in input {
            if sensor.dist(beacon) >= sensor.dist(&current) {
                ans += 1;
                break;
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &Input) -> Option<i64> {
    let max_coord = if input.len() < 20 { 20 } else { 4_000_000 };
    let sensors_with_dists: Vec<(&Point, i32)> = input.iter()
        .map(|(sensor, beacon)| (sensor, sensor.dist(beacon)))
        .collect();
    for y in 0..max_coord + 1 {
        //if y % 100 == 0 {
        //    println!("y = {}", y);
        //}
        let mut x = 0;
        'outer: while x < max_coord {
            let current = Point(x, y);
            for (sensor, dist) in &sensors_with_dists {
                if *dist >= sensor.dist(&current) {
                    x = sensor.0 + (dist - (sensor.1 - current.1).abs()) + 1;
                    //println!("{:?} {} {:?} {}", sensor, dist, current, y);
                    continue 'outer;
                }
            }
            return Some(current.tuning_frequency());
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 15, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 15, input_parser);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 15, input_parser);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
