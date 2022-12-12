use std::{
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Position {
    Start,
    End,
    Value(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    path: Vec<(usize, usize)>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.path.len().partial_cmp(&self.path.len())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.path.len().cmp(&self.path.len())
    }
}

fn is_climbable(pos1: &Position, pos2: &Position) -> bool {
    match (pos1, pos2) {
        (Position::Start, Position::Start) => panic!("Impossible"),
        (Position::Start, Position::End) => true,
        (Position::Start, Position::Value(_)) => true,
        (Position::End, Position::Start) => false,
        (Position::End, Position::End) => panic!("Impossible end end"),
        (Position::End, Position::Value(_)) => false,
        (Position::Value(_), Position::Start) => false,
        (Position::Value(c), Position::End) => 'z' as usize <= *c as usize + 1,
        (Position::Value(c1), Position::Value(c2)) => *c2 as usize <= *c1 as usize + 1,
    }
}

fn parse_map(buf: String) -> Vec<Vec<Position>> {
    buf.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Position::Start,
                    'E' => Position::End,
                    x => Position::Value(x),
                })
                .collect()
        })
        .collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_map(buf);

    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|x| x == &Position::Start)
                .map(|j| (i, j))
        })
        .unwrap();

    let mut stack = BinaryHeap::new();
    stack.push(State { path: vec![start] });
    let mut visited = HashSet::new();

    while let Some(state) = stack.pop() {
        let next = state.path.last().cloned().unwrap();
        if !visited.contains(&next) {
            visited.insert(next);
            if matches!(map[next.0][next.1], Position::End) {
                return (state.path.len() - 1).to_string();
            }

            if next.0 >= 1 && is_climbable(&map[next.0][next.1], &map[next.0 - 1][next.1]) {
                let mut new_state = state.clone();
                new_state.path.push((next.0 - 1, next.1));
                stack.push(new_state);
            }

            if next.0 < map.len() - 1
                && is_climbable(&map[next.0][next.1], &map[next.0 + 1][next.1])
            {
                let mut new_state = state.clone();
                new_state.path.push((next.0 + 1, next.1));
                stack.push(new_state);
            }

            if next.1 >= 1 && is_climbable(&map[next.0][next.1], &map[next.0][next.1 - 1]) {
                let mut new_state = state.clone();
                new_state.path.push((next.0, next.1 - 1));
                stack.push(new_state);
            }

            if next.1 < map[next.0].len() - 1
                && is_climbable(&map[next.0][next.1], &map[next.0][next.1 + 1])
            {
                let mut new_state = state.clone();
                new_state.path.push((next.0, next.1 + 1));
                stack.push(new_state);
            }
        }
    }

    unreachable!()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_map(buf);

    let starts = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_j, x)| x == &Position::Start || x == &Position::Value('a'))
                .map(move |(j, _x)| (i, j))
        })
        .map(|start| State { path: vec![start] });

    let mut stack = BinaryHeap::new();
    stack.extend(starts);
    let mut visited = HashSet::new();

    while let Some(state) = stack.pop() {
        let next = state.path.last().cloned().unwrap();
        if !visited.contains(&next) {
            visited.insert(next);
            if matches!(map[next.0][next.1], Position::End) {
                return (state.path.len() - 1).to_string();
            }

            if next.0 >= 1 && is_climbable(&map[next.0][next.1], &map[next.0 - 1][next.1]) {
                let mut new_state = state.clone();
                new_state.path.push((next.0 - 1, next.1));
                stack.push(new_state);
            }

            if next.0 < map.len() - 1
                && is_climbable(&map[next.0][next.1], &map[next.0 + 1][next.1])
            {
                let mut new_state = state.clone();
                new_state.path.push((next.0 + 1, next.1));
                stack.push(new_state);
            }

            if next.1 >= 1 && is_climbable(&map[next.0][next.1], &map[next.0][next.1 - 1]) {
                let mut new_state = state.clone();
                new_state.path.push((next.0, next.1 - 1));
                stack.push(new_state);
            }

            if next.1 < map[next.0].len() - 1
                && is_climbable(&map[next.0][next.1], &map[next.0][next.1 + 1])
            {
                let mut new_state = state.clone();
                new_state.path.push((next.0, next.1 + 1));
                stack.push(new_state);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )),
            "31"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )),
            "29"
        );
    }
}
