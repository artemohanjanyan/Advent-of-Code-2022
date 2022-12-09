use nom::{
    character::complete::{anychar, char},
    combinator::{map, verify},
    sequence::terminated,
    multi::many1,
    IResult,
};

type Input = Vec<Vec<i8>>;

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        many1(
            map(
                verify(anychar, |&c| c.is_ascii_digit()),
                |c| c as i8 - '0' as i8,
            ),
        ),
        char('\n'),
    ))(input)
}

pub fn part_one(input: &Input) -> Option<usize> {
    let mut visibility_map = vec![vec![false; input[0].len()]; input.len()];
    for i in 0..visibility_map.len() {
        let mut max_height = -1i8;
        for j in 0..visibility_map[0].len() {
            if input[i][j] > max_height {
                max_height = input[i][j];
                visibility_map[i][j] = true;
            }
        }
    }
    for i in 0..visibility_map.len() {
        let mut max_height = -1i8;
        for j in 0..visibility_map[0].len() {
            if input[j][i] > max_height {
                max_height = input[j][i];
                visibility_map[j][i] = true;
            }
        }
    }
    for i in (0..visibility_map.len()).rev() {
        let mut max_height = -1i8;
        for j in (0..visibility_map[0].len()).rev() {
            if input[i][j] > max_height {
                max_height = input[i][j];
                visibility_map[i][j] = true;
            }
        }
    }
    for i in (0..visibility_map.len()).rev() {
        let mut max_height = -1i8;
        for j in (0..visibility_map[0].len()).rev() {
            if input[j][i] > max_height {
                max_height = input[j][i];
                visibility_map[j][i] = true;
            }
        }
    }
    Some(
        visibility_map.iter()
            .map(|row| row.iter().filter(|f| **f).count())
            .sum()
    )
}

fn scenic_score(input: &Input, row: usize, column: usize) -> usize {
    let mut v1 = 0usize;
    for i in (0..row).rev() {
        v1 += 1;
        if input[i][column] >= input[row][column] {
            break;
        }
    }
    let mut v2 = 0usize;
    for j in (0..column).rev() {
        v2 += 1;
        if input[row][j] >= input[row][column] {
            break;
        }
    }
    let mut v3 = 0usize;
    for i in row + 1..input.len() {
        v3 += 1;
        if input[i][column] >= input[row][column] {
            break;
        }
    }
    let mut v4 = 0usize;
    for j in column + 1..input[0].len() {
        v4 += 1;
        if input[row][j] >= input[row][column] {
            break;
        }
    }
    v1 * v2 * v3 * v4
}

pub fn part_two(input: &Input) -> Option<usize> {
    let mut max_score = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let score = scenic_score(input, i, j);
            max_score = std::cmp::max(max_score, score);
        }
    }
    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 8, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 8, input_parser);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 8, input_parser);
        assert_eq!(part_two(&input), Some(8));
    }
}
