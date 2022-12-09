use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

#[derive(Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut state: VecDeque<(isize, isize)> = VecDeque::new();
    state.push_back((0, 0));
    state.push_back((0, 0));
    let mut visited = HashSet::new();
    for line in buf.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let direction = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            x => panic!("Unknown direction: {}", x),
        };
        let distance = amount.parse::<usize>().unwrap();
        for _i in 0..distance {
            let head = state.pop_back().unwrap();
            let new_head_position = match &direction {
                Dir::Up => (head.0 + 1, head.1),
                Dir::Down => (head.0 - 1, head.1),
                Dir::Left => (head.0, head.1 - 1),
                Dir::Right => (head.0, head.1 + 1),
            };
            let mut new_knots = VecDeque::new();
            new_knots.push_back(new_head_position);
            while let Some(knot) = state.pop_back() {
                let new_knot_position = if (knot.0 - new_head_position.0).abs() > 1
                    || (knot.1 - new_head_position.1).abs() > 1
                {
                    match direction {
                        Dir::Up => (knot.0 + 1, new_head_position.1),
                        Dir::Down => (knot.0 - 1, new_head_position.1),
                        Dir::Left => (new_head_position.0, knot.1 - 1),
                        Dir::Right => (new_head_position.0, knot.1 + 1),
                    }
                } else {
                    knot
                };
                new_knots.push_front(new_knot_position);
            }
            visited.insert(*new_knots.front().unwrap());
            state = new_knots;

            println!("{:?}", state);
        }
    }
    visited.len().to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut state: VecDeque<(isize, isize)> = VecDeque::new();
    for _i in 0..10 {
        state.push_back((0, 0));
    }
    let mut visited = HashSet::new();
    for line in buf.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let direction = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            x => panic!("Unknown direction: {}", x),
        };
        let distance = amount.parse::<usize>().unwrap();
        for _i in 0..distance {
            let head = state.pop_front().unwrap();
            let new_head_position = match &direction {
                Dir::Up => (head.0 + 1, head.1),
                Dir::Down => (head.0 - 1, head.1),
                Dir::Left => (head.0, head.1 - 1),
                Dir::Right => (head.0, head.1 + 1),
            };
            let mut new_knots = VecDeque::new();
            new_knots.push_back(new_head_position);
            while let Some(knot) = state.pop_front() {
                let previous_knot = new_knots.back().unwrap();

                let new_knot_position = if (knot.0 - previous_knot.0)
                    .abs()
                    .max((knot.1 - previous_knot.1).abs())
                    > 1
                {
                    let dy = match previous_knot.0.cmp(&knot.0) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };

                    let dx = match previous_knot.1.cmp(&knot.1) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };
                    (knot.0 + dy, knot.1 + dx)
                } else {
                    knot
                };
                new_knots.push_back(new_knot_position);
            }
            visited.insert(*new_knots.back().unwrap());
            state = new_knots;

            println!("{:?}: {:?}", direction, state);
        }
    }
    println!("{:?}", visited.contains(&(259, -178)));
    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )),
            "13"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )),
            "1"
        );

        assert_eq!(
            star_two(Cursor::new(
                b"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            "36"
        );
    }
}

// 2678
// 2722
