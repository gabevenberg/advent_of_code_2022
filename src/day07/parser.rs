#![allow(unused)]
use crate::file_tree::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CdRoot,
    CdUp,
    Cd(String),
    Ls(Vec<LsEntry>),
}
#[derive(Debug, PartialEq, Eq)]
pub enum LsEntry {
    Dir(String),
    File(ParseFile),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFile {
    size: usize,
    name: String,
}
pub fn parse(input: &str) -> Dir {
    unimplemented!()
}

pub fn parse_to_tree(input: Vec<Command>) -> Dir {
    unimplemented!()
}

//parses a single line
pub fn parse_to_commands(input: &str) -> Vec<Command> {
    static PARSE_COMMAND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^$").unwrap());
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /* #[test]
    fn test_parse() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(parse(input), 0);
    } */

    #[test]
    fn test_parse_to_commands() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(
            parse_to_commands(input),
            vec![
                Command::CdRoot,
                Command::Ls(vec![
                    LsEntry::Dir(String::from("a")),
                    LsEntry::File(ParseFile {
                        size: 14848514,
                        name: String::from("b.txt")
                    }),
                    LsEntry::File(ParseFile {
                        size: 8504156,
                        name: String::from("c.dat")
                    }),
                    LsEntry::Dir(String::from("d"))
                ]),
                Command::Cd(String::from("a")),
                Command::Ls(vec![
                    LsEntry::Dir(String::from("e")),
                    LsEntry::File(ParseFile {
                        size: 29116,
                        name: String::from("f")
                    }),
                    LsEntry::File(ParseFile {
                        size: 2557,
                        name: String::from("g")
                    }),
                    LsEntry::File(ParseFile {
                        size: 62596,
                        name: String::from("h.lst")
                    }),
                ]),
                Command::Cd(String::from("e")),
                Command::Ls(vec![LsEntry::File(ParseFile {
                    size: 484,
                    name: String::from("i")
                }),]),
                Command::CdUp,
                Command::CdUp,
                Command::Cd(String::from("d")),
                Command::Ls(vec![
                    LsEntry::File(ParseFile {
                        size: 4060174,
                        name: String::from("j")
                    }),
                    LsEntry::File(ParseFile {
                        size: 8033020,
                        name: String::from("d.log")
                    }),
                    LsEntry::File(ParseFile {
                        size: 5626152,
                        name: String::from("d.ext")
                    }),
                    LsEntry::File(ParseFile {
                        size: 7214296,
                        name: String::from("k")
                    }),
                ]),
            ]
        )
    }
}
