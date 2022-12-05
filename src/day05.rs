use regex::Regex;
use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let (stack_input, moves) = buf.split_once("\n\n").unwrap();

    let mut stack_lines = stack_input.lines().rev();

    let mut stacks = stack_lines
        .next()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|_| Vec::<char>::new())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);
    for line in stack_lines {
        let line_chars: Vec<char> = line.chars().collect();
        for i in 0..stacks.len() {
            if !line_chars[4 * i + 1].is_ascii_whitespace() {
                stacks[i].push(line_chars[4 * i + 1])
            }
        }
    }

    let re = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();

    for line in moves.lines() {
        //move 1 from 2 to 1
        let cap = re
            .captures(line)
            .unwrap_or_else(|| panic!("Did not match '{}'", line));
        println!("{:?}", cap);
        let count = cap[1].parse::<usize>().unwrap();
        let from_stack = cap[2].parse::<usize>().unwrap();
        let to_stack = cap[3].parse::<usize>().unwrap();
        for _i in 0..count {
            let container = stacks[from_stack - 1].pop().unwrap();
            stacks[to_stack - 1].push(container);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let (stack_input, moves) = buf.split_once("\n\n").unwrap();

    let mut stack_lines = stack_input.lines().rev();

    let mut stacks = stack_lines
        .next()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|_| Vec::<char>::new())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);
    for line in stack_lines {
        let line_chars: Vec<char> = line.chars().collect();
        for i in 0..stacks.len() {
            if !line_chars[4 * i + 1].is_ascii_whitespace() {
                stacks[i].push(line_chars[4 * i + 1])
            }
        }
    }

    let re = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();

    for line in moves.lines() {
        //move 1 from 2 to 1
        let cap = re
            .captures(line)
            .unwrap_or_else(|| panic!("Did not match '{}'", line));
        println!("{:?}", cap);
        let count = cap[1].parse::<usize>().unwrap();
        let from_stack = cap[2].parse::<usize>().unwrap();
        let to_stack = cap[3].parse::<usize>().unwrap();
        let mut tmp = Vec::new();
        for _i in 0..count {
            let container = stacks[from_stack - 1].pop().unwrap();
            tmp.push(container);
        }
        for container in tmp.into_iter().rev() {
            stacks[to_stack - 1].push(container);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "CMZ"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "MCD"
        );
    }
}
