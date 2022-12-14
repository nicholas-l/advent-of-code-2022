use std::{collections::HashMap, fmt::Display, io::BufRead};

#[derive(Debug)]
enum Position {
    Rock,
    Sand,
}

fn parse_input(buf: String) -> Vec<Vec<(usize, usize)>> {
    buf.lines()
        .map(|line| {
            line.split("->")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').unwrap();
                    (
                        x.trim().parse::<usize>().unwrap(),
                        y.trim()
                            .parse::<usize>()
                            .unwrap_or_else(|_e| panic!("Could not parse: {}", y)),
                    )
                })
                .collect()
        })
        .collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut lists = parse_input(buf);

    let mut map = HashMap::new();

    let mut map_max_x = lists[0][0].0;
    let mut map_max_y = lists[0][0].1;
    let mut map_min_x = lists[0][0].0;
    let mut map_min_y = lists[0][0].1;

    while let Some(mut line) = lists.pop() {
        let mut previous = line.pop().unwrap();
        map_max_x = map_max_x.max(previous.0);
        map_max_y = map_max_y.max(previous.1);
        map_min_x = map_min_x.min(previous.0);
        map_min_y = map_min_y.min(previous.1);
        while let Some(pos) = line.pop() {
            if previous.0 == pos.0 {
                // Vertical Line
                for y in previous.1.min(pos.1)..=pos.1.max(previous.1) {
                    map.insert((previous.0, y), Position::Rock);
                }
            } else if previous.1 == pos.1 {
                // Horizontal line
                // Todo might point up?
                for x in previous.0.min(pos.0)..=pos.0.max(previous.0) {
                    map.insert((x, previous.1), Position::Rock);
                }
            } else {
                panic!("diagonal lines not supported");
            }
            map_max_x = map_max_x.max(pos.0);
            map_max_y = map_max_y.max(pos.1);
            map_min_x = map_min_x.min(pos.0);
            map_min_y = map_min_y.min(pos.1);

            previous = pos;
        }
    }
    println!("{}", map_min_x);
    println!("{}", map_max_x);
    println!("{}", map_min_y);
    println!("{}", map_max_y);

    for j in map_min_y..=map_max_y {
        for i in map_min_x..=map_max_x {
            let v = match map.get(&(i, j)) {
                Some(Position::Rock) => '#',
                Some(Position::Sand) => 'o',
                None => '.',
            };
            print!("{}", v);
        }
        println!();
    }

    let mut new_sand_position = (500, 0);
    let mut sand_units = 0;
    loop {
        match map.get(&(new_sand_position.0, new_sand_position.1 + 1)) {
            Some(_) => {
                if map
                    .get(&(new_sand_position.0 - 1, new_sand_position.1 + 1))
                    .is_none()
                {
                    new_sand_position = (new_sand_position.0 - 1, new_sand_position.1 + 1);
                } else if map
                    .get(&(new_sand_position.0 + 1, new_sand_position.1 + 1))
                    .is_none()
                {
                    new_sand_position = (new_sand_position.0 + 1, new_sand_position.1 + 1);
                } else {
                    sand_units += 1;
                    map.insert(new_sand_position, Position::Sand);
                    new_sand_position = (500, 0);
                }
            }
            None => {
                if new_sand_position.1 > map_max_y {
                    return sand_units.to_string();
                }
                new_sand_position = (new_sand_position.0, new_sand_position.1 + 1);
            }
        }
    }
}

struct Map {
    positions: HashMap<(usize, usize), Position>,
    max_y: usize,
    min_x: Option<usize>,
    max_x: Option<usize>,
}

impl Map {
    fn new() -> Map {
        Map {
            positions: HashMap::new(),
            max_y: 0,
            min_x: None,
            max_x: None,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..=self.max_y {
            for i in self.min_x.unwrap_or(0)..=self.max_x.unwrap_or(0) {
                let v = match self.positions.get(&(i, j)) {
                    Some(Position::Rock) => '#',
                    Some(Position::Sand) => 'o',
                    None => '.',
                };
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut lists = parse_input(buf);

    let mut map = HashMap::new();
    let mut _map2 = Map::new(); // TODO

    let mut map_max_x = lists[0][0].0;
    let mut map_max_y = lists[0][0].1;
    let mut map_min_x = lists[0][0].0;
    let mut map_min_y = lists[0][0].1;

    while let Some(mut line) = lists.pop() {
        let mut previous = line.pop().unwrap();
        map_max_x = map_max_x.max(previous.0);
        map_max_y = map_max_y.max(previous.1);
        map_min_x = map_min_x.min(previous.0);
        map_min_y = map_min_y.min(previous.1);
        while let Some(pos) = line.pop() {
            if previous.0 == pos.0 {
                // Vertical Line
                for y in previous.1.min(pos.1)..=pos.1.max(previous.1) {
                    map.insert((previous.0, y), Position::Rock);
                }
            } else if previous.1 == pos.1 {
                // Horizontal line
                for x in previous.0.min(pos.0)..=pos.0.max(previous.0) {
                    map.insert((x, previous.1), Position::Rock);
                }
            } else {
                panic!("diagonal lines not supported");
            }
            map_max_x = map_max_x.max(pos.0);
            map_max_y = map_max_y.max(pos.1);
            map_min_x = map_min_x.min(pos.0);
            map_min_y = map_min_y.min(pos.1);

            previous = pos;
        }
    }
    println!("{}", map_min_x);
    println!("{}", map_max_x);
    println!("{}", map_min_y);
    println!("{}", map_max_y);

    map_max_y += 2;

    // print_map(&map, map_max_y, map_min_x, map_max_x);

    let mut new_sand_position = (500, 0);
    let mut sand_units = 0;
    loop {
        match map.get(&(new_sand_position.0, new_sand_position.1 + 1)) {
            Some(_) => {
                if map
                    .get(&(new_sand_position.0 - 1, new_sand_position.1 + 1))
                    .is_none()
                {
                    new_sand_position = (new_sand_position.0 - 1, new_sand_position.1 + 1);
                } else if map
                    .get(&(new_sand_position.0 + 1, new_sand_position.1 + 1))
                    .is_none()
                {
                    new_sand_position = (new_sand_position.0 + 1, new_sand_position.1 + 1);
                } else {
                    sand_units += 1;
                    map.insert(new_sand_position, Position::Sand);
                    if new_sand_position == (500, 0) {
                        return sand_units.to_string();
                    }
                    new_sand_position = (500, 0);
                }
            }
            None => {
                if new_sand_position.1 == map_max_y - 1 {
                    sand_units += 1;
                    map.insert(new_sand_position, Position::Sand);
                    new_sand_position = (500, 0);
                } else {
                    new_sand_position = (new_sand_position.0, new_sand_position.1 + 1);
                }
            }
        }
        // print_map(&map, map_max_y, map_min_x, map_max_x);
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )),
            "24"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )),
            "93"
        );
    }
}

// 30132
