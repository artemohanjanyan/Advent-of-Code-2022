use nom::{
    character::complete::{char, alpha1},
    combinator::map,
    sequence::terminated,
    multi::many1,
    IResult,
};

type Input = Vec<String>;

pub fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(
        map(alpha1, |backpack: &str| backpack.to_owned()),
        char('\n'),
    ))(input)
}

fn type_to_priority(c: char) -> u32 {
    c as u32 - if c.is_ascii_lowercase() {
        'a' as u32 - 1
    } else {
        'A' as u32 - 27
    }
}

fn find_common_char(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1
            }
        }
    }
    '0'
}

fn find_common_char_3(s1: &str, s2: &str, s3: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                for c3 in s3.chars() {
                    if c1 == c3 {
                        return c1;
                    }
                }
            }
        }
    }
    '0'
}

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        input.iter()
            .map(|s| type_to_priority(find_common_char(&s[..s.len() / 2], &s[s.len() / 2..])))
            .sum()
    )
}

pub fn part_two(input: &Input) -> Option<u32> {
    let mut sum = 0;
    for i in (0..input.len()).step_by(3) {
        sum += type_to_priority(find_common_char_3(&input[i], &input[i + 1], &input[i + 2]));
    }
    Some(sum)
}

fn input_panicking_parser(input: String) -> Input {
    let (rest, input) = input_parser(&input).expect("could not parse input file");
    if !rest.is_empty() {
        panic!("Input wasn't fully parsed:\n{}", rest);
    }
    input
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 3, input_panicking_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 3, input_panicking_parser);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 3, input_panicking_parser);
        assert_eq!(part_two(&input), Some(70));
    }
}
