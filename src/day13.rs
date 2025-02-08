use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, u32},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult, Parser,
};
use std::{cmp::Ordering, io::BufRead, iter};

#[derive(Debug, PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
    Value(usize),
}

fn parse_list(input: &str) -> IResult<&str, Node> {
    map(separated_list0(char(','), parse_node), Node::List).parse(input)
}

fn parse_value(input: &str) -> IResult<&str, Node> {
    map(u32, |x| Node::Value(x as usize)).parse(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((delimited(char('['), parse_list, char(']')), parse_value)).parse(input)
}

fn parse_input(buf: String) -> Vec<(Node, Node)> {
    buf.split("\n\n")
        .map(|pair_input| {
            let (l1, l2) = pair_input.split_once('\n').unwrap();
            (parse_node(l1).unwrap().1, parse_node(l2).unwrap().1)
        })
        .collect()
}

fn is_right_order(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::List(l1), Node::List(l2)) => {
            for p in l1.iter().zip_longest(l2.iter()) {
                match p {
                    itertools::EitherOrBoth::Both(l, r) => {
                        let res = is_right_order(l, r);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    itertools::EitherOrBoth::Left(_l) => return Ordering::Greater,
                    itertools::EitherOrBoth::Right(_r) => return Ordering::Less,
                };
            }
            Ordering::Equal
        }
        (Node::List(l1), Node::Value(v2)) => {
            for p in l1.iter().zip_longest(iter::once(&Node::Value(*v2))) {
                match p {
                    itertools::EitherOrBoth::Both(l, r) => {
                        let res = is_right_order(l, r);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    itertools::EitherOrBoth::Left(_l) => return Ordering::Greater,
                    itertools::EitherOrBoth::Right(_r) => return Ordering::Less,
                };
            }
            Ordering::Equal
        }
        (Node::Value(v1), Node::List(l2)) => {
            for p in iter::once(&Node::Value(*v1)).zip_longest(l2.iter()) {
                match p {
                    itertools::EitherOrBoth::Both(l, r) => {
                        let res = is_right_order(l, r);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    itertools::EitherOrBoth::Left(_l) => return Ordering::Greater,
                    itertools::EitherOrBoth::Right(_r) => return Ordering::Less,
                };
            }
            Ordering::Equal
        }
        (Node::Value(v1), Node::Value(v2)) => v1.cmp(v2),
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let lists = parse_input(buf);

    lists
        .iter()
        .enumerate()
        .filter(|(_, pair)| is_right_order(&pair.0, &pair.1) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let lists = parse_input(buf);

    let mut all_packets: Vec<Node> = lists
        .into_iter()
        .flat_map(|pair| vec![pair.0, pair.1])
        .collect();

    all_packets.push(Node::List(vec![Node::List(vec![Node::Value(2)])]));
    all_packets.push(Node::List(vec![Node::List(vec![Node::Value(6)])]));

    all_packets.sort_by(is_right_order);

    let pos_2 = all_packets
        .iter()
        .position(|n| n == &Node::List(vec![Node::List(vec![Node::Value(2)])]))
        .unwrap()
        + 1;
    let pos_6 = all_packets
        .iter()
        .position(|n| n == &Node::List(vec![Node::List(vec![Node::Value(6)])]))
        .unwrap()
        + 1;
    (pos_2 * pos_6).to_string()
}

#[cfg(test)]
mod tests {
    use super::{parse_node, star_one, star_two, Node};
    use std::io::Cursor;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_node("1"), Ok(("", Node::Value(1))));

        assert_eq!(
            parse_node("[1,2,3]"),
            Ok((
                "",
                Node::List(vec![Node::Value(1), Node::Value(2), Node::Value(3)])
            ))
        );
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"
            )),
            "13"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            )),
            "140"
        );
    }
}
