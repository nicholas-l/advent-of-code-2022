use std::{collections::HashMap, fmt::Display, io::BufRead, ops::RangeInclusive};

#[derive(Debug, PartialEq, Eq)]
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
                            .unwrap_or_else(|_e| panic!("Could not parse: {y}")),
                    )
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum SandPosition {
    Start,
    Overflow,
    InMap,
}

struct Map {
    positions: HashMap<(usize, usize), Position>,
    max_y: usize,
    min_x: Option<usize>,
    max_x: Option<usize>,
    bottom: Option<usize>,
}

impl Map {
    fn new() -> Map {
        Map {
            positions: HashMap::new(),
            max_y: 0,
            min_x: None,
            max_x: None,
            bottom: None,
        }
    }

    fn add_rocks(&mut self, mut lists: Vec<Vec<(usize, usize)>>) {
        while let Some(mut line) = lists.pop() {
            let mut previous = line.pop().unwrap();
            while let Some(pos) = line.pop() {
                if previous.0 == pos.0 {
                    // Vertical Line
                    self.add_vertical_rock(pos.0, previous.1.min(pos.1)..=pos.1.max(previous.1));
                } else if previous.1 == pos.1 {
                    self.add_horizontal_rock(pos.1, previous.0.min(pos.0)..=pos.0.max(previous.0));
                } else {
                    panic!("diagonal lines not supported");
                }
                previous = pos;
            }
        }
    }

    fn add_horizontal_rock(&mut self, y: usize, range: RangeInclusive<usize>) {
        let start = *range.start();
        let end = *range.end();
        for x in range {
            self.positions.insert((x, y), Position::Rock);
        }
        self.max_x = self.max_x.max(Some(end));
        self.max_y = self.max_y.max(y);
        self.min_x = self.min_x.min(Some(start));
    }

    fn add_vertical_rock(&mut self, x: usize, range: RangeInclusive<usize>) {
        let end = *range.end();
        for y in range {
            self.positions.insert((x, y), Position::Rock);
        }
        self.max_x = self.max_x.max(Some(x));
        self.max_y = self.max_y.max(end);
        self.min_x = self.min_x.min(Some(x));
    }

    fn add_bottom(&mut self, value: usize) {
        self.bottom = Some(self.max_y + value);
    }

    fn add_sand(&mut self) -> SandPosition {
        let mut sand_position = (500, 0);
        loop {
            match self.positions.get(&(sand_position.0, sand_position.1 + 1)) {
                Some(_) => {
                    if self
                        .positions
                        .get(&(sand_position.0 - 1, sand_position.1 + 1))
                        .is_none()
                    {
                        sand_position = (sand_position.0 - 1, sand_position.1 + 1);
                    } else if self
                        .positions
                        .get(&(sand_position.0 + 1, sand_position.1 + 1))
                        .is_none()
                    {
                        sand_position = (sand_position.0 + 1, sand_position.1 + 1);
                    } else {
                        // self.sand_units += 1;
                        self.positions.insert(sand_position, Position::Sand);
                        if sand_position == (500, 0) {
                            return SandPosition::Start;
                        } else {
                            return SandPosition::InMap;
                        }
                    }
                }
                None => {
                    if self
                        .bottom
                        .map(|v| v - 1 == sand_position.1)
                        .unwrap_or(false)
                    {
                        // sand_units += 1;
                        self.positions.insert(sand_position, Position::Sand);
                        return SandPosition::InMap;
                    } else if sand_position.1 > self.max_y {
                        return SandPosition::Overflow;
                    }
                    sand_position = (sand_position.0, sand_position.1 + 1);
                }
            }
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
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let lists = parse_input(buf);

    let mut map = Map::new(); // TODO

    map.add_rocks(lists);
    // println!("{}", map2);

    let mut sand_units = 0;
    loop {
        if SandPosition::Overflow == map.add_sand() {
            return sand_units.to_string();
        }
        sand_units += 1;
    }
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let lists = parse_input(buf);

    let mut map = Map::new(); // TODO

    map.add_rocks(lists);
    // println!("{}", map2);

    map.add_bottom(2);

    let mut sand_units = 0;
    loop {
        sand_units += 1;
        if SandPosition::Start == map.add_sand() {
            return sand_units.to_string();
        }
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
