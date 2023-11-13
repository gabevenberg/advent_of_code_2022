#![allow(unused)]
use crate::file_tree::*;
use once_cell::sync::Lazy;
use regex::Regex;

static IS_COMMAND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$").unwrap());
static PARSE_CD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$ cd (\S*)$").unwrap());
static PARSE_LS_ENTRY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([[:alnum:]]*) (\S*)$").unwrap());

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

//parses a single line
pub fn parse_to_commands(input: &str) -> Vec<Command> {
    let mut ret = Vec::new();
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line == "$ ls" {
            ret.push(Command::Ls(parse_ls(&mut lines)));
        } else {
            let captures = PARSE_CD_REGEX
                .captures(line)
                .unwrap_or_else(|| panic!("invalid line {}", line));

            ret.push(match &captures[1] {
                ".." => Command::CdUp,
                "/" => Command::CdRoot,
                s => Command::Cd(s.to_string()),
            })
        }
    }
    ret
}

fn parse_ls(lines: &mut std::iter::Peekable<std::str::Lines<'_>>) -> Vec<LsEntry> {
    let mut ret: Vec<LsEntry> = Vec::new();
    while lines.peek().is_some() {
        // if the next line is a command, then we are at the end of the ls listing.
        let line = lines.peek().expect("no next line");
        if IS_COMMAND_REGEX.is_match(line) {
            break;
        }
        let captures = PARSE_LS_ENTRY_REGEX
            .captures(line)
            .unwrap_or_else(|| panic!("invalid line {}", line));
        ret.push(match &captures[1] {
            "dir" => LsEntry::Dir(captures[2].to_string()),
            s => LsEntry::File(ParseFile {
                size: str::parse(&captures[1]).unwrap_or_else(|_| panic!("invalid line {}", line)),
                name: captures[2].to_string(),
            }),
        });
        lines.next();
    }
    ret
}

pub fn parse(input: &str) -> Node {
    unimplemented!()
}

pub fn commands_to_tree(input: Vec<Command>) -> Node {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_commands() {
        let input = concat!(
            "$ cd /\n",
            "$ ls\n",
            "dir a\n",
            "14848514 b.txt\n",
            "8504156 c.dat\n",
            "dir d\n",
            "$ cd a\n",
            "$ ls\n",
            "dir e\n",
            "29116 f\n",
            "2557 g\n",
            "62596 h.lst\n",
            "$ cd e\n",
            "$ ls\n",
            "584 i\n",
            "$ cd ..\n",
            "$ cd ..\n",
            "$ cd d\n",
            "$ ls\n",
            "4060174 j\n",
            "8033020 d.log\n",
            "5626152 d.ext\n",
            "7214296 k"
        );
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
                    size: 584,
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
