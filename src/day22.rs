use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

type Coord = (isize, isize);

enum Command {
    Left,
    Right,
    Move(isize),
}

type State = (isize, isize, i8);

impl Command {
    fn move_position(&self, map: &Map, state: State) -> State {
        match self {
            Command::Left => (state.0, state.1, (state.2 - 1).rem_euclid(4)),
            Command::Right => (state.0, state.1, (state.2 + 1).rem_euclid(4)),
            Command::Move(x) => {
                let mut curr_pos = (state.0, state.1);
                for _i in 0..*x {
                    let next_position = match state.2 {
                        // Right
                        0 => (curr_pos.0, curr_pos.1 + 1),
                        // Down
                        1 => (curr_pos.0 + 1, curr_pos.1),
                        // Left
                        2 => (curr_pos.0, curr_pos.1 - 1),
                        // Up
                        3 => (curr_pos.0 - 1, curr_pos.1),
                        x => panic!("Direction is not valid ({})", x),
                    };
                    curr_pos = match map.positions.get(&next_position) {
                        Some(true) => next_position, // Valid move
                        Some(false) => break,        // found wall
                        None => {
                            // Wrap around
                            let wrapped_pos = map.get_most(&next_position, state.2);
                            // Check for wall
                            match map.positions.get(&wrapped_pos) {
                                Some(true) => wrapped_pos, // Valid move
                                Some(false) => break,      // found wall
                                None => panic!("Left most returned a position off the map"),
                            }
                        }
                    };
                }
                (curr_pos.0, curr_pos.1, state.2)
            }
        }
    }
}

// enum Direction {
//     Up,
//     Right,
//     Down,
//     Left,
// }

struct Map {
    positions: HashMap<Coord, bool>,
    bounds: (isize, isize, isize, isize),
}

impl Map {
    fn get_left_most(&self, row: isize) -> isize {
        (self.bounds.2..=self.bounds.3)
            .find(|j| self.positions.get(&(row, *j)).is_some())
            .unwrap()
    }

    fn get_right_most(&self, row: isize) -> isize {
        (self.bounds.2..=self.bounds.3)
            .rev()
            .find(|j| self.positions.get(&(row, *j)).is_some())
            .unwrap()
    }
    fn get_top_most(&self, col: isize) -> isize {
        (self.bounds.0..=self.bounds.1)
            .find(|i| self.positions.get(&(*i, col)).is_some())
            .unwrap()
    }
    fn get_bottom_most(&self, col: isize) -> isize {
        (self.bounds.0..=self.bounds.1)
            .rev()
            .find(|i| self.positions.get(&(*i, col)).is_some())
            .unwrap()
    }

    fn get_most(&self, position: &Coord, dir: i8) -> (isize, isize) {
        match dir {
            0 => (position.0, self.get_left_most(position.0)),
            1 => (self.get_top_most(position.1), position.1),
            2 => (position.0, self.get_right_most(position.0)),
            3 => (self.get_bottom_most(position.1), position.1),
            x => panic!("Invalid direction: {}", x),
        }
    }
}

fn parse_input(buf: &str) -> (Map, Vec<Command>) {
    let (map_input, path_input) = buf.split_once("\n\n").unwrap();
    let path = {
        let mut path = vec![];
        let mut digits = vec![];

        for c in path_input.chars() {
            match c {
                x if x.is_ascii_digit() => {
                    digits.push(x);
                }
                'R' => {
                    if !digits.is_empty() {
                        path.push(Command::Move(
                            digits
                                .drain(..)
                                .collect::<String>()
                                .parse::<isize>()
                                .unwrap(),
                        ));
                    }
                    path.push(Command::Right);
                }
                'L' => {
                    if !digits.is_empty() {
                        path.push(Command::Move(
                            digits
                                .drain(..)
                                .collect::<String>()
                                .parse::<isize>()
                                .unwrap_or_else(|_e| panic!("{:?}", digits)),
                        ));
                    }
                    path.push(Command::Left);
                }
                _ => panic!(),
            }
        }

        if !digits.is_empty() {
            path.push(Command::Move(
                digits.iter().collect::<String>().parse::<isize>().unwrap(),
            ));
        }
        path
    };
    let map = {
        let positions = HashMap::from_iter(map_input.lines().enumerate().flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '.' => Some(((i as isize, j as isize), true)),
                '#' => Some(((i as isize, j as isize), false)),
                ' ' => None,
                _ => panic!("Bad map"),
            })
        }));
        let minmax_y = positions
            .keys()
            .map(|(i, _j)| i)
            .minmax()
            .into_option()
            .unwrap();
        let minmax_x = positions
            .keys()
            .map(|(_i, j)| j)
            .minmax()
            .into_option()
            .unwrap();
        let bounds = (*minmax_y.0, *minmax_y.1, *minmax_x.0, *minmax_x.1);
        Map { positions, bounds }
    };
    (map, path)
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let (map, commands) = parse_input(&buf);
    let mut position = (0, map.get_left_most(0), 0);
    // println!("{:?}", position);
    // println!("{}", map.positions.get(&(7,3)).unwrap());
    for command in commands {
        position = command.move_position(&map, position);
        // println!("{:?}", position);
    }
    // println!("{} {} {}", position.0, position.1, position.2);
    (1000 * (position.0 + 1) + 4 * (position.1 + 1) + position.2 as isize).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let _equations = parse_input(&buf);
    todo!()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            )),
            "6032"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            )),
            "5031"
        );
    }
}
