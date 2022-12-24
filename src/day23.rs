use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

use itertools::Itertools;

type Coord = (isize, isize);

enum Direction {
    North,
    South,
    West,
    East,
}

struct Map {
    elves: HashSet<Coord>,
}

impl Map {
    fn bounds(&self) -> (isize, isize, isize, isize) {
        let minmax_y = self
            .elves
            .iter()
            .map(|(i, _j)| i)
            .minmax()
            .into_option()
            .unwrap();
        let minmax_x = self
            .elves
            .iter()
            .map(|(_i, j)| j)
            .minmax()
            .into_option()
            .unwrap();
        (*minmax_y.0, *minmax_y.1, *minmax_x.0, *minmax_x.1)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        for i in bounds.0..=bounds.1 {
            for j in bounds.2..=bounds.3 {
                let c = if self.elves.contains(&(i, j)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn step(map: &mut Map, directions: &[Direction], start_idx: usize) -> bool {
    let mut new_positions = HashMap::new();
    let mut new_elves = HashSet::new();

    for elf in &map.elves {
        if [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .all(|(i, j)| !map.elves.contains(&(elf.0 + i, elf.1 + j)))
        {
            assert!(new_elves.insert(*elf));
        } else {
            let mut possible_move = false;
            for i in 0..directions.len() {
                let direction = &directions[(start_idx + i) % directions.len()];
                match direction {
                    Direction::North => {
                        if !map.elves.contains(&(elf.0 - 1, elf.1 - 1))
                            && !map.elves.contains(&(elf.0 - 1, elf.1))
                            && !map.elves.contains(&(elf.0 - 1, elf.1 + 1))
                        {
                            new_positions
                                .entry((elf.0 - 1, elf.1))
                                .or_insert_with(Vec::new)
                                .push(elf);
                            possible_move = true;
                            break;
                        }
                    }
                    Direction::South => {
                        if !map.elves.contains(&(elf.0 + 1, elf.1 - 1))
                            && !map.elves.contains(&(elf.0 + 1, elf.1))
                            && !map.elves.contains(&(elf.0 + 1, elf.1 + 1))
                        {
                            new_positions
                                .entry((elf.0 + 1, elf.1))
                                .or_insert_with(Vec::new)
                                .push(elf);
                            possible_move = true;

                            break;
                        }
                    }
                    Direction::West => {
                        if !map.elves.contains(&(elf.0 - 1, elf.1 - 1))
                            && !map.elves.contains(&(elf.0, elf.1 - 1))
                            && !map.elves.contains(&(elf.0 + 1, elf.1 - 1))
                        {
                            new_positions
                                .entry((elf.0, elf.1 - 1))
                                .or_insert_with(Vec::new)
                                .push(elf);
                            possible_move = true;

                            break;
                        }
                    }
                    Direction::East => {
                        if !map.elves.contains(&(elf.0 - 1, elf.1 + 1))
                            && !map.elves.contains(&(elf.0, elf.1 + 1))
                            && !map.elves.contains(&(elf.0 + 1, elf.1 + 1))
                        {
                            new_positions
                                .entry((elf.0, elf.1 + 1))
                                .or_insert_with(Vec::new)
                                .push(elf);
                            possible_move = true;

                            break;
                        }
                    }
                }
            }
            if !possible_move {
                assert!(new_elves.insert(*elf));
            }
        }
    }

    if new_positions.is_empty() {
        return false;
    }

    for (new_pos, old_postions) in new_positions {
        if old_postions.len() == 1 {
            assert!(new_elves.insert(new_pos));
        } else {
            for pos in old_postions {
                assert!(new_elves.insert(*pos));
            }
        }
    }

    *map = Map { elves: new_elves };

    true
}

fn parse_input(buf: &str) -> Map {
    let elves = HashSet::from_iter(buf.lines().enumerate().flat_map(|(i, line)| {
        line.chars().enumerate().filter_map(move |(j, c)| {
            if c == '#' {
                Some((i as isize, j as isize))
            } else {
                None
            }
        })
    }));
    Map { elves }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let mut map = parse_input(&buf);

    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for i in 0..10 {
        step(&mut map, &directions, i);
    }

    let bounds = map.bounds();

    (bounds.0..=bounds.1)
        .map(|i| {
            (bounds.2..=bounds.3)
                .filter(|j| !map.elves.contains(&(i, *j)))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut map = parse_input(&buf);

    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut i = 0;

    while step(&mut map, &directions, i) {
        i += 1
    }

    (i + 1).to_string()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            )),
            "110"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            )),
            "20"
        );
    }
}
