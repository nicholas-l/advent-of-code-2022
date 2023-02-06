use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::East => write!(f, ">"),
            Direction::West => write!(f, "<"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Position {
    Wall,
    Blizzard(Vec<Direction>),
    Empty,
}

impl Position {
    fn add_blizzard(&mut self, dir: Direction) {
        if let Self::Blizzard(dirs) = self {
            dirs.push(dir);
        } else {
            *self = Self::Blizzard(vec![dir]);
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Wall => write!(f, "#"),
            Position::Blizzard(d) => {
                if d.len() > 1 {
                    write!(f, "{}", d.len())
                } else {
                    write!(f, "{}", d[0])
                }
            }
            Position::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    current_position: Coord,
    end_position: Coord,
}

// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for State {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let sq_distance1 = {
//             let dy = self.current_position.0 - self.end_position.0;
//             let dx = self.current_position.1 - self.end_position.1;
//             dx * dx + dy * dy
//         };
//         let sq_distance2 = {
//             let dy = other.current_position.0 - other.end_position.0;
//             let dx = other.current_position.1 - other.end_position.1;
//             dx * dx + dy * dy
//         };
//         sq_distance1.cmp(&sq_distance2)
//     }
// }

impl State {
    fn step(&self, map: &Map) -> Vec<State> {
        [
            (0, 0),  //Wait
            (-1, 0), // Up
            (1, 0),  // Down,
            (0, -1), // Left,
            (0, 1),
        ]
        .iter()
        .filter_map(|delta| {
            let new_coord = (
                self.current_position.0 + delta.0,
                self.current_position.1 + delta.1,
            );
            // if self.current_position == (3, 5) {
            //     println!("{:?} - {:?}", delta, map.get(&new_coord));
            // }
            if map
                .get(&new_coord)
                .map(|p| matches!(p, &Position::Empty))
                .unwrap_or(false)
            {
                Some(State {
                    current_position: new_coord,
                    end_position: self.end_position,
                })
            } else {
                None
            }
        })
        .collect()
    }
}

struct Map {
    bounds: (isize, isize, isize, isize),
    positions: HashMap<Coord, Position>,
    // blizzards: HashMap<Coord, Vec<Position>>,
    // walls: HashMap<Coord, Position>,
    start: Coord,
    end: Coord,
}

impl Map {
    fn get(&self, coord: &Coord) -> Option<&Position> {
        if self.bounds.0 <= coord.0
            && coord.0 <= self.bounds.1
            && self.bounds.2 <= coord.1
            && coord.1 <= self.bounds.3
        {
            Some(self.positions.get(coord).unwrap_or(&Position::Empty))
        } else {
            None
        }
    }

    fn step(&mut self) {
        let mut new_state = HashMap::new();

        for (coord, entity) in &self.positions {
            match entity {
                Position::Wall => {
                    new_state.insert(*coord, Position::Wall);
                }
                Position::Blizzard(ref dirs) => {
                    for dir in dirs {
                        let delta = match dir {
                            Direction::North => (-1, 0),
                            Direction::South => (1, 0),
                            Direction::East => (0, 1),
                            Direction::West => (0, -1),
                        };
                        let new_coord = (coord.0 + delta.0, coord.1 + delta.1);
                        let new_coord =
                            if matches!(self.positions.get(&new_coord), Some(Position::Wall)) {
                                match dir {
                                    Direction::North => (self.bounds.1 - 1, coord.1),
                                    Direction::South => (self.bounds.0 + 1, coord.1),
                                    Direction::East => (coord.0, self.bounds.2 + 1),
                                    Direction::West => (coord.0, self.bounds.3 - 1),
                                }
                            } else {
                                new_coord
                            };
                        new_state
                            .entry(new_coord)
                            .or_insert(Position::Empty)
                            .add_blizzard(dir.clone());
                    }
                }
                Position::Empty => {
                    new_state.entry(*coord).or_insert(Position::Empty);
                }
            }
        }
        self.positions.clear();
        self.positions.extend(new_state);
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.bounds.0..=self.bounds.1 {
            for j in self.bounds.2..=self.bounds.3 {
                match self.positions.get(&(i, j)) {
                    Some(v) => write!(f, "{v}")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(buf: &str) -> (Map, State) {
    let positions: HashMap<_, _> = buf
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| match c {
                '#' => ((i as isize, j as isize), Position::Wall),
                '>' => (
                    (i as isize, j as isize),
                    Position::Blizzard(vec![Direction::East]),
                ),
                '^' => (
                    (i as isize, j as isize),
                    Position::Blizzard(vec![Direction::North]),
                ),
                '<' => (
                    (i as isize, j as isize),
                    Position::Blizzard(vec![Direction::West]),
                ),
                'v' => (
                    (i as isize, j as isize),
                    Position::Blizzard(vec![Direction::South]),
                ),
                '.' => ((i as isize, j as isize), Position::Empty),
                _ => panic!(),
            })
        })
        .collect();
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

    let start_position = (
        0,
        (bounds.2..=bounds.3)
            .find(|x| matches!(positions.get(&(0, *x)), Some(Position::Empty)))
            .unwrap(),
    );
    let end_position = (
        bounds.1,
        (bounds.2..=bounds.3)
            .find(|x| matches!(positions.get(&(bounds.1, *x)), Some(Position::Empty)))
            .unwrap(),
    );

    // let walls = HashMap::from_iter(
    //     positions
    //         .iter()
    //         .filter(|(k, p)| !matches!(p, &Position::Blizzard(_)))
    //         .map(|(k, v)| (*k, v.clone()))
    // );

    // let blizzards = HashMap::from_iter(
    //     positions
    //         .into_iter()
    //         .filter(|(_k, v)| matches!(v, Position::Blizzard(_)))
    //         .map(|(k, v)| (k, vec![v])),
    // );

    (
        Map {
            positions,
            bounds,
            start: start_position,
            end: end_position,
        },
        State {
            current_position: start_position,
            end_position,
        },
    )
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let (mut map, state) = parse_input(&buf);

    let mut states = HashSet::new();
    states.insert(state);

    println!("{states:?}");
    println!("{map}");
    println!("{:?}", map.bounds);
    println!("{:?}", map.end);

    for t in 0.. {
        if states.iter().any(|state| state.current_position == map.end) {
            // println!("{map}");
            return t.to_string();
        }
        // println!("Minute {}", t + 1);
        map.step();
        // println!("{}", map);

        states = states
            .into_iter()
            .flat_map(|state| state.step(&map))
            .collect();
        // println!("{:?}", states);
        // if t > 18 {
        //     break;
        // }
    }
    unreachable!()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let (mut map, state) = parse_input(&buf);

    let mut states = HashSet::new();
    states.insert(state);

    println!("{states:?}");
    println!("{map}");
    println!("{:?}", map.bounds);
    println!("{:?}", map.end);

    let mut completed = 0;

    for t in 0.. {
        if states
            .iter()
            .any(|state| state.current_position == state.end_position)
        {
            states.clear();
            if completed == 2 {
                return t.to_string();
            } else if completed == 0 {
                states.insert(State {
                    current_position: map.end,
                    end_position: map.start,
                });
            } else {
                states.insert(State {
                    current_position: map.start,
                    end_position: map.end,
                });
            }
            completed += 1
        }
        // println!("Minute {}", t + 1);
        map.step();
        // println!("{}", map);

        states = states
            .into_iter()
            .flat_map(|state| state.step(&map))
            .collect();
        // println!("{:?}", states);
        // if t > 18 {
        //     break;
        // }
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
                b"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            )),
            "18"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            )),
            "54"
        );
    }
}

// 264 - too low
