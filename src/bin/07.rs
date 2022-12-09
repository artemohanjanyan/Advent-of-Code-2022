use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1},
    combinator::{map, map_res, value, verify},
    sequence::{pair, preceded, separated_pair, terminated},
    multi::{many0, many1},
    IResult,
};

use std::str::FromStr;

#[derive(Clone)]
pub enum CdDir {
    Root, Up, Dir(String)
}

#[derive(Clone)]
pub enum Command {
    Cd(CdDir),
    Ls,
}

#[derive(Clone, Debug)]
pub struct File {
    _name: String,
    size: usize,
}

#[derive(Clone, Debug)]
pub enum DirectoryItem {
    Directory(String),
    File(File),
}

pub struct CommandWithOutput {
    command: Command,
    output: Vec<DirectoryItem>,
}

pub type Input = Vec<CommandWithOutput>;

fn cd_dir_parser(input: &str) -> IResult<&str, CdDir> {
    alt((
        value(CdDir::Root, tag("/")),
        value(CdDir::Up, tag("..")),
        map(alpha1, |dir: &str| CdDir::Dir(dir.to_owned())),
    ))(input)
}

fn command_parser(input: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((
            preceded(
                tag("cd "),
                map(cd_dir_parser, |cd_dir| Command::Cd(cd_dir)),
            ),
            value(Command::Ls, tag("ls")),
        ))
    )(input)
}

fn directory_item_parser(input: &str) -> IResult<&str, DirectoryItem> {
    alt((
        map(
            preceded(tag("dir "), alpha1),
            |dir: &str| DirectoryItem::Directory(dir.to_owned()),
        ),
        map(
            separated_pair(
                map_res(digit1, FromStr::from_str),
                char(' '),
                many1(verify(anychar, |&c| c.is_ascii_alphabetic() || c == '.')),
            ),
            |(size, name)| DirectoryItem::File(File {
                _name: name.into_iter().collect(),
                size: size,
            }),
        ),
    ))(input)
}

fn command_with_output_parser(input: &str) -> IResult<&str, CommandWithOutput> {
    map(
        pair(
            terminated(command_parser, char('\n')),
            many0(terminated(directory_item_parser, char('\n'))),
        ),
        |(command, output)| CommandWithOutput {
            command: command,
            output: output,
        },
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many1(command_with_output_parser)(input)
}

#[derive(Debug)]
struct Dir {
    directories: Vec<(String, Box<Dir>)>,
    files: Vec<File>,
}

impl Dir {
    fn add_directory_item(&mut self, path: &[&str], directory_item: DirectoryItem) {
        if path.len() == 0 {
            match directory_item {
                DirectoryItem::Directory(dir_name) =>
                    self.directories.push((
                        dir_name.clone(),
                        Box::new(Dir {
                            directories: vec!(),
                            files: vec!(),
                        }),
                    )),
                DirectoryItem::File(file) =>
                    self.files.push(file.clone()),
            }
        } else {
            for (next_dir_name, next_dir) in &mut self.directories {
                if next_dir_name == path[0] {
                    next_dir.add_directory_item(&path[1..], directory_item);
                    break;
                }
            }
        }
    }

    fn size(&self, size_sum: &mut usize, min_dir: &mut usize, min_dir_req: usize) -> usize {
        let mut dir_size = 0;
        for (_, dir) in &self.directories {
            dir_size += dir.size(size_sum, min_dir, min_dir_req);
        }
        for file in &self.files {
            dir_size += file.size;
        }
        if dir_size <= 100000 {
            *size_sum += dir_size;
        }
        if dir_size >= min_dir_req {
            *min_dir = std::cmp::min(dir_size, *min_dir);
        }
        dir_size
    }
}

fn init(input: &Input, root: &mut Dir) {
    let mut path: Vec<&str> = vec!();
    for command_with_output in input {
        match &command_with_output.command {
            Command::Ls =>
                for directory_item in &command_with_output.output {
                    root.add_directory_item(&path[..], directory_item.clone());
                },
            Command::Cd(dir) =>
                match dir {
                    CdDir::Root => path.clear(),
                    CdDir::Up => { path.pop(); () },
                    CdDir::Dir(dir) => path.push(dir),
                },
        }
    }
}

pub fn part_one(input: &Input) -> Option<usize> {
    let mut root = Dir {
        directories: vec!(),
        files: vec!(),
    };
    init(input, &mut root);
    let mut size_sum = 0;
    let mut min_dir = 0;
    let min_dir_req = 0;
    root.size(&mut size_sum, &mut min_dir, min_dir_req);
    Some(size_sum)
}

pub fn part_two(input: &Input) -> Option<usize> {
    let mut root = Dir {
        directories: vec!(),
        files: vec!(),
    };
    init(input, &mut root);
    let mut size_sum = 0;
    let mut min_dir = 0;
    let mut min_dir_req = 0;
    let all_size = root.size(&mut size_sum, &mut min_dir, min_dir_req);
    size_sum = 0;
    min_dir = 70000000;
    min_dir_req = 30000000 - (70000000 - all_size);
    root.size(&mut size_sum, &mut min_dir, min_dir_req);
    Some(min_dir)
}

fn main() {
    let input = &advent_of_code::read_file_nom("inputs", 7, input_parser);
    advent_of_code::solve_nom!(1, part_one, input);
    advent_of_code::solve_nom!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_nom("examples", 7, input_parser);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_nom("examples", 7, input_parser);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
