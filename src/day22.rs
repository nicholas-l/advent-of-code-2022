use std::{collections::HashMap, io::BufRead, ops::Add};

use itertools::Itertools;

type Coord = (isize, isize);

#[derive(Debug)]
enum Command {
    Left,
    Right,
    Move(isize),
}

type State = (isize, isize, i8);

impl Command {
    fn move_position(&self, map: &Map, state: State) -> Result<State, String> {
        match self {
            Command::Left => Ok((state.0, state.1, (state.2 - 1).rem_euclid(4))),
            Command::Right => Ok((state.0, state.1, (state.2 + 1).rem_euclid(4))),
            Command::Move(x) => {
                let mut curr_pos = (state.0, state.1);
                let mut curr_dir = state.2;
                for _i in 0..*x {
                    let next_position = match curr_dir {
                        // Right
                        0 => (curr_pos.0, curr_pos.1 + 1),
                        // Down
                        1 => (curr_pos.0 + 1, curr_pos.1),
                        // Left
                        2 => (curr_pos.0, curr_pos.1 - 1),
                        // Up
                        3 => (curr_pos.0 - 1, curr_pos.1),
                        x => panic!("Direction is not valid ({x})"),
                    };
                    (curr_pos, curr_dir) = match map.positions.get(&next_position) {
                        Some(true) => {
                            (next_position, curr_dir) // Valid move
                        }
                        Some(false) => break, // found wall
                        None => {
                            // Wrap around
                            println!("Wrapping around: {:?}", next_position);
                            let (wrapped_pos, wrapped_dir) = map.get_most(&curr_pos, curr_dir)?;
                            // Check for wall
                            match map.positions.get(&wrapped_pos) {
                                Some(true) => (wrapped_pos, wrapped_dir.into()), // Valid move
                                Some(false) => break,                            // found wall
                                None => panic!(
                                    "Get most returned a position off the map: {:?}",
                                    wrapped_pos
                                ),
                            }
                        }
                    };
                }
                Ok((curr_pos.0, curr_pos.1, curr_dir))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Add<Coord> for Direction {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        let delta = match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        (rhs.0 + delta.0, rhs.1 + delta.1)
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        let delta = match rhs {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        (self.0 + delta.0, self.1 + delta.1)
    }
}

impl From<Direction> for i8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CornerDirection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq, Eq)]
enum CornerType {
    Internal(CornerDirection),
    Outside(CornerDirection),
    Flat,
}

struct Map {
    positions: HashMap<Coord, bool>,
    bounds: (isize, isize, isize, isize),
    dimensions: usize,
    edge_mapping: HashMap<(Coord, Direction), (Coord, Direction)>,
}

impl Map {
    fn get_left_most(&self, coord: &Coord) -> Result<(Coord, Direction), String> {
        if self.dimensions == 3 {
            self.edge_mapping
                .get(&(*coord, Direction::Right))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "No left most for {:?}\nR:{:?}\nD:{:?}\nL:{:?}\nU:{:?}\n",
                        coord,
                        self.edge_mapping.get(&(*coord, Direction::Right)),
                        self.edge_mapping.get(&(*coord, Direction::Down)),
                        self.edge_mapping.get(&(*coord, Direction::Left)),
                        self.edge_mapping.get(&(*coord, Direction::Up))
                    )
                })
        } else {
            let x = (self.bounds.2..=self.bounds.3)
                .find(|j| self.positions.get(&(coord.0, *j)).is_some())
                .unwrap();
            Ok(((coord.0, x), Direction::Right))
        }
    }

    fn get_right_most(&self, coord: &Coord) -> Result<(Coord, Direction), String> {
        if self.dimensions == 3 {
            self.edge_mapping
                .get(&(*coord, Direction::Left))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "No right most for {:?}\nR:{:?}\nD:{:?}\nL:{:?}\nU:{:?}\n",
                        coord,
                        self.edge_mapping.get(&(*coord, Direction::Right)),
                        self.edge_mapping.get(&(*coord, Direction::Down)),
                        self.edge_mapping.get(&(*coord, Direction::Left)),
                        self.edge_mapping.get(&(*coord, Direction::Up))
                    )
                })
        } else {
            let x = (self.bounds.2..=self.bounds.3)
                .rev()
                .find(|j| self.positions.get(&(coord.0, *j)).is_some())
                .unwrap();
            // We don't care about the direction here, so just return the direction
            Ok(((coord.0, x), Direction::Left))
        }
    }

    fn get_top_most(&self, coord: &Coord) -> Result<(Coord, Direction), String> {
        if self.dimensions == 3 {
            self.edge_mapping
                .get(&(*coord, Direction::Down))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "No top most for {:?}\nR:{:?}\nD:{:?}\nL:{:?}\nU:{:?}\n",
                        coord,
                        self.edge_mapping.get(&(*coord, Direction::Right)),
                        self.edge_mapping.get(&(*coord, Direction::Down)),
                        self.edge_mapping.get(&(*coord, Direction::Left)),
                        self.edge_mapping.get(&(*coord, Direction::Up))
                    )
                })
        } else {
            let y = (self.bounds.0..=self.bounds.1)
                .find(|i| self.positions.get(&(*i, coord.1)).is_some())
                .unwrap();
            Ok(((y, coord.1), Direction::Down))
        }
    }

    fn get_bottom_most(&self, coord: &Coord) -> Result<(Coord, Direction), String> {
        if self.dimensions == 3 {
            // self.get_corresponding_point(col)
            self.edge_mapping
                .get(&(*coord, Direction::Up))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "No bottom most for {:?}\nR:{:?}\nD:{:?}\nL:{:?}\nU:{:?}\n",
                        coord,
                        self.edge_mapping.get(&(*coord, Direction::Right)),
                        self.edge_mapping.get(&(*coord, Direction::Down)),
                        self.edge_mapping.get(&(*coord, Direction::Left)),
                        self.edge_mapping.get(&(*coord, Direction::Up))
                    )
                })
        } else {
            let y = (self.bounds.0..=self.bounds.1)
                .rev()
                .find(|i| self.positions.get(&(*i, coord.1)).is_some())
                .unwrap();
            Ok(((y, coord.1), Direction::Up))
        }
    }

    fn categorise2(&self, coord: &Coord) -> Option<CornerType> {
        let top = self
            .positions
            .contains_key(&((coord.0, coord.1) + Direction::Up));
        let bottom = self.positions.contains_key(&(coord.0 + 1, coord.1));
        let left = self.positions.contains_key(&(coord.0, coord.1 - 1));
        let right = self.positions.contains_key(&(coord.0, coord.1 + 1));

        let top_left = self.positions.contains_key(&(coord.0 - 1, coord.1 - 1));
        let top_right = self.positions.contains_key(&(coord.0 - 1, coord.1 + 1));
        let bottom_left = self.positions.contains_key(&(coord.0 + 1, coord.1 - 1));
        let bottom_right = self.positions.contains_key(&(coord.0 + 1, coord.1 + 1));
        if !self.positions.contains_key(coord) {
            return None;
        }

        let adjacent_spots =
            usize::from(top) + usize::from(bottom) + usize::from(left) + usize::from(right);

        match (
            adjacent_spots,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ) {
            (4, true, true, true, false) => {
                Some(CornerType::Internal(CornerDirection::BottomRight))
            }
            (4, true, true, false, true) => Some(CornerType::Internal(CornerDirection::BottomLeft)),
            (4, true, false, true, true) => Some(CornerType::Internal(CornerDirection::TopRight)),
            (4, false, true, true, true) => Some(CornerType::Internal(CornerDirection::TopLeft)),
            (2, true, false, false, false) => Some(CornerType::Outside(CornerDirection::TopLeft)),
            (2, false, true, false, false) => Some(CornerType::Outside(CornerDirection::TopRight)),
            (2, false, false, true, false) => {
                Some(CornerType::Outside(CornerDirection::BottomLeft))
            }
            (2, false, false, false, true) => {
                Some(CornerType::Outside(CornerDirection::BottomRight))
            }
            (_, true, true, true, true) => None,
            (_, true, false, false, true) => panic!("Could not happen"),
            (_, false, true, true, false) => panic!("Could not happen"),
            _ => Some(CornerType::Flat),
        }
    }

    fn get_next_point(
        &self,
        coord: &Coord,
        dir: Direction,
    ) -> ((Coord, Direction), Option<(Coord, Direction)>) {
        let next = dir + *coord;
        let position_type = self.categorise2(&next);
        match position_type {
            Some(CornerType::Internal(CornerDirection::TopLeft)) => {
                if dir == Direction::Down {
                    ((next + Direction::Left, Direction::Left), None)
                } else {
                    assert_eq!(dir, Direction::Right);
                    ((next + Direction::Up, Direction::Up), None)
                }
            }
            Some(CornerType::Internal(CornerDirection::TopRight)) => {
                if dir == Direction::Down {
                    ((next + Direction::Right, Direction::Right), None)
                } else {
                    assert_eq!(dir, Direction::Left);
                    ((next + Direction::Up, Direction::Up), None)
                }
            }
            Some(CornerType::Internal(CornerDirection::BottomLeft)) => {
                if dir == Direction::Up {
                    ((next + Direction::Left, Direction::Left), None)
                } else {
                    assert_eq!(dir, Direction::Right);
                    ((next + Direction::Down, Direction::Down), None)
                }
            }
            Some(CornerType::Internal(CornerDirection::BottomRight)) => {
                if dir == Direction::Up {
                    ((next + Direction::Right, Direction::Right), None)
                } else {
                    assert_eq!(dir, Direction::Left);
                    ((next + Direction::Down, Direction::Down), None)
                }
            }
            Some(CornerType::Outside(CornerDirection::TopLeft)) => {
                let next_dir = if dir == Direction::Down {
                    Direction::Left
                } else {
                    assert_eq!(dir, Direction::Right);
                    Direction::Up
                };
                ((next, dir), Some((next, next_dir)))
            }
            Some(CornerType::Outside(CornerDirection::TopRight)) => {
                let next_dir = if dir == Direction::Down {
                    Direction::Right
                } else {
                    assert_eq!(dir, Direction::Left);
                    Direction::Up
                };
                ((next, dir), Some((next, next_dir)))
            }
            Some(CornerType::Outside(CornerDirection::BottomLeft)) => {
                let next_dir = if dir == Direction::Up {
                    Direction::Left
                } else {
                    assert_eq!(dir, Direction::Right);
                    Direction::Down
                };
                ((next, dir), Some((next, next_dir)))
            }
            Some(CornerType::Outside(CornerDirection::BottomRight)) => {
                let next_dir = if dir == Direction::Up {
                    Direction::Right
                } else {
                    assert_eq!(dir, Direction::Left);
                    Direction::Down
                };
                ((next, dir), Some((next, next_dir)))
            }
            Some(CornerType::Flat) => ((next, dir), None),
            x => {
                panic!("Not, {:?} for {:?}", x, next);
            }
        }
    }

    fn get_direction_of_edge(&self, coord: &Coord, direction: &Direction) -> Direction {
        match direction {
            Direction::Up | Direction::Down => {
                if self.positions.contains_key(&(coord.0, coord.1 - 1)) {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Right | Direction::Left => {
                if self.positions.contains_key(&(coord.0 - 1, coord.1)) {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
        }
    }

    fn init_edge_mapping(&mut self) {
        let mut stack = Vec::new();

        println!("{:?}", self.bounds);

        for y in self.bounds.0..=self.bounds.1 {
            for x in self.bounds.2..=self.bounds.3 {
                match self.categorise2(&(y, x)) {
                    Some(CornerType::Internal(CornerDirection::TopLeft)) => {
                        println!("Top left found at ({}, {})", y, x);
                        stack.push(((y - 1, x), Direction::Up, (y, x - 1), Direction::Left))
                    }
                    Some(CornerType::Internal(CornerDirection::TopRight)) => {
                        println!("Top right found at ({}, {})", y, x);
                        stack.push(((y - 1, x), Direction::Up, (y, x + 1), Direction::Right))
                    }
                    Some(CornerType::Internal(CornerDirection::BottomLeft)) => {
                        println!("Bottom left found at ({}, {})", y, x);
                        stack.push(((y + 1, x), Direction::Down, (y, x - 1), Direction::Left))
                    }
                    Some(CornerType::Internal(CornerDirection::BottomRight)) => {
                        println!("Bottom right found at ({}, {})", y, x);
                        stack.push(((y + 1, x), Direction::Down, (y, x + 1), Direction::Right))
                    }
                    _ => {}
                }
            }
        }

        while let Some((side1, dir1, side2, dir2)) = stack.pop() {
            println!("{:?}", stack);
            println!(
                "Processing: {:?}, {:?} - {:?}, {:?}",
                side1, dir1, side2, dir2
            );

            let corner_type1 = self.categorise2(&side1);
            let corner_type2 = self.categorise2(&side2);

            println!("{:?} {:?}", corner_type1, corner_type2);

            assert_ne!(side1, side2);

            if matches!(corner_type1, Some(CornerType::Outside(_)))
                && matches!(corner_type2, Some(CornerType::Outside(_)))
            {
                println!("Found two outside corners");
                continue;
            }

            if matches!(corner_type1, Some(CornerType::Internal(_)))
                || matches!(corner_type2, Some(CornerType::Internal(_)))
            {
                continue;
            }

            if self
                .edge_mapping
                .contains_key(&(side1, self.get_direction_of_edge(&side1, &dir1)))
            {
                // We have already seen this edge
                continue;
            }

            self.edge_mapping.insert(
                (side1, self.get_direction_of_edge(&side1, &dir1)),
                (side2, self.get_direction_of_edge(&side2, &dir2).inverse()),
            );
            self.edge_mapping.insert(
                (side2, self.get_direction_of_edge(&side2, &dir2)),
                (side1, self.get_direction_of_edge(&side1, &dir1).inverse()),
            );

            // Find next two mappings
            let (next_side1, maybe_next_side1) = self.get_next_point(&side1, dir1);
            let (next_side2, maybe_next_side2) = self.get_next_point(&side2, dir2);

            if maybe_next_side1.is_some() || maybe_next_side2.is_some() {
                println!("452: {:?} {:?}", maybe_next_side1, maybe_next_side2);

                println!("Adding to mapping: {:?} {:?}", next_side1, next_side2);
                self.edge_mapping.insert(
                    (
                        next_side1.0,
                        self.get_direction_of_edge(&next_side1.0, &next_side1.1),
                    ),
                    (
                        next_side2.0,
                        self.get_direction_of_edge(&next_side2.0, &next_side2.1)
                            .inverse(),
                    ),
                );
                self.edge_mapping.insert(
                    (
                        next_side2.0,
                        self.get_direction_of_edge(&next_side2.0, &next_side2.1),
                    ),
                    (
                        next_side1.0,
                        self.get_direction_of_edge(&next_side1.0, &next_side1.1)
                            .inverse(),
                    ),
                );

                // We should add to mapping here as well as queuing up the next point.
                if maybe_next_side1.is_some() && maybe_next_side2.is_some() {
                    //ending
                    // dbg!(next_side1);
                    // dbg!(next_side2);

                    // dbg!(maybe_next_side1);
                    // dbg!(maybe_next_side2);
                    // assert!(false);
                } else if let Some(next_side1) = maybe_next_side1 {
                    // Todo deal with outside corners needing to be in there twice.
                    let (next_side2, _) = self.get_next_point(&next_side2.0, next_side2.1);
                    stack.push((next_side1.0, next_side1.1, next_side2.0, next_side2.1));
                } else if let Some(next_side2) = maybe_next_side2 {
                    let (next_side1, _) = self.get_next_point(&next_side1.0, next_side1.1);
                    stack.push((next_side1.0, next_side1.1, next_side2.0, next_side2.1));
                }
            } else {
                stack.push((next_side1.0, next_side1.1, next_side2.0, next_side2.1));
            }
        }
    }

    fn get_most(&self, position: &Coord, dir: i8) -> Result<(Coord, Direction), String> {
        match dir {
            0 => self.get_left_most(position),
            1 => self.get_top_most(position),
            2 => self.get_right_most(position),
            3 => self.get_bottom_most(position),
            x => panic!("Invalid direction: {}", x),
        }
    }
}

fn parse_input(buf: &str, dimensions: usize) -> (Map, Vec<Command>) {
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
                                .unwrap_or_else(|_e| panic!("{digits:?}")),
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
        Map {
            positions,
            bounds,
            dimensions,
            edge_mapping: HashMap::new(),
        }
    };
    (map, path)
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let (map, commands) = parse_input(&buf, 2);

    let start_x = (0..=map.bounds.3)
        .find(|j| map.positions.get(&(0, *j)).is_some())
        .unwrap();

    let mut position = (0, map.get_left_most(&(0, start_x)).unwrap().0 .1, 0);

    for command in commands {
        println!("{:?}", position);
        position = command
            .move_position(&map, position)
            .unwrap_or_else(|e| panic!("Error: {:?} at {:?}", e, position));
    }
    (1000 * (position.0 + 1) + 4 * (position.1 + 1) + position.2 as isize).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let (mut map, commands) = parse_input(&buf, 3);
    let start_x = (0..=map.bounds.3)
        .find(|j| map.positions.get(&(0, *j)).is_some())
        .unwrap();
    let mut position = (0, start_x, 0);
    map.init_edge_mapping();

    for command in commands {
        position = command
            .move_position(&map, position)
            .unwrap_or_else(|e| panic!("Error: {:?} at {:?}", e, position));
    }
    (1000 * (position.0 + 1) + 4 * (position.1 + 1) + position.2 as isize).to_string()
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
