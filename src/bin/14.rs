use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{separated_pair, terminated},
    multi::{many1, separated_list1},
    IResult,
};

use advent_of_code::helpers::int_parser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point(i32, i32);

type Path = Vec<Point>;

type Input = Vec<Path>;

fn point_parser(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            int_parser,
            char(','),
            int_parser,
        ),
        |(p1, p2)| Point(p1, p2),
    )(input)
}

fn path_parser(input: &str) -> IResult<&str, Path> {
    separated_list1(
        tag(" -> "),
        point_parser,
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        path_parser,
        char('\n'),
    ))(input)
}

#[derive(Clone, PartialEq, Eq)]
enum Pixel {
    Air, Stone, Sand
}

pub fn part_one(input: &Input) -> Option<u32> {
    let min_x = input.iter()
        .map(|path| path.iter()
             .map(|point| point.0)
             .min()
             .unwrap()
         )
        .min()
        .unwrap();
    let min_y = 0;
    let max_x = input.iter()
        .map(|path| path.iter()
             .map(|point| point.0)
             .max()
             .unwrap()
         )
        .max()
        .unwrap();
    let max_y = input.iter()
        .map(|path| path.iter()
             .map(|point| point.1)
             .max()
             .unwrap()
         )
        .max()
        .unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut field = vec![vec![Pixel::Air; height as usize]; width as usize];
    for path in input {
        for i in 1..path.len() {
            let point0 = &path[i - 1];
            let point1 = &path[i];
            let dx = point1.0.cmp(&point0.0) as i32;
            let dy = point1.1.cmp(&point0.1) as i32;
            let mut point = point0.clone();
            loop {
                field[(point.0 - min_x) as usize][(point.1 - min_y) as usize] = Pixel::Stone;
                if point == *point1 {
                    break;
                }
                point.0 += dx;
                point.1 += dy;
            }
        }
    }
    let sand_start = Point(500 - min_x, 0);
    let mut sand_count = 0;
    'outer: loop {
        let mut sand = sand_start.clone();
        while sand.0 >= 0 && sand.0 < width && sand.1 < height {
            if field[sand.0 as usize][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
            } else if sand.0 == 0 ||
                    field[sand.0 as usize - 1][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
                sand.0 -= 1;
            } else if sand.0 == width - 1 ||
                    field[sand.0 as usize + 1][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                field[sand.0 as usize][sand.1 as usize] = Pixel::Sand;
                sand_count += 1;
                continue 'outer;
            }
        }
        break;
    }
    Some(sand_count)
}

pub fn part_two(input: &Input) -> Option<u32> {
    let min_y = 0;
    let max_y = input.iter()
        .map(|path| path.iter()
             .map(|point| point.1)
             .max()
             .unwrap()
         )
        .max()
        .unwrap() + 2;
    let height = max_y - min_y + 1;
    let min_x = input.iter()
        .map(|path| path.iter()
             .map(|point| point.0)
             .min()
             .unwrap()
         )
        .min()
        .unwrap()
        .min(500 - height);
    let max_x = input.iter()
        .map(|path| path.iter()
             .map(|point| point.0)
             .max()
             .unwrap()
         )
        .max()
        .unwrap()
        .max(500 + height);
    let width = max_x - min_x + 1;
    let mut field = vec![vec![Pixel::Air; height as usize]; width as usize];
    for path in input {
        for i in 1..path.len() {
            let point0 = &path[i - 1];
            let point1 = &path[i];
            let dx = point1.0.cmp(&point0.0) as i32;
            let dy = point1.1.cmp(&point0.1) as i32;
            let mut point = point0.clone();
            loop {
                field[(point.0 - min_x) as usize][(point.1 - min_y) as usize] = Pixel::Stone;
                if point == *point1 {
                    break;
                }
                point.0 += dx;
                point.1 += dy;
            }
        }
    }
    for x in 0..field.len() {
        field[x][height as usize - 1] = Pixel::Stone;
    }
    let sand_start = Point(500 - min_x, 0);
    let mut sand_count = 0;
    'outer: loop {
        let mut sand = sand_start.clone();
        while sand.0 >= 0 && sand.0 < width && sand.1 < height {
            if field[sand.0 as usize][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
            } else if sand.0 == 0 ||
                    field[sand.0 as usize - 1][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
                sand.0 -= 1;
            } else if sand.0 == width - 1 ||
                    field[sand.0 as usize + 1][sand.1 as usize + 1] == Pixel::Air {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                field[sand.0 as usize][sand.1 as usize] = Pixel::Sand;
                sand_count += 1;
                if sand == sand_start {
                    break 'outer;
                } else {
                    continue 'outer;
                }
            }
        }
        break;
    }
    Some(sand_count)
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 14, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 14, input_parser);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 14, input_parser);
        assert_eq!(part_two(&input), Some(93));
    }
}
