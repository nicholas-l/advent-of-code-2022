use std::{collections::HashMap, fmt::Display, io::BufRead, ops::Add};

fn parse_input(buf: String) -> ((isize, isize), Vec<(Coord, Coord)>) {
    let mut lines = buf.lines();
    let meta = lines
        .next()
        .map(|l| {
            let (row, max_search) = l.split_once(' ').unwrap();
            (
                row.parse::<isize>().unwrap(),
                max_search.parse::<isize>().unwrap(),
            )
        })
        .unwrap();
    let lists = lines
        .map(|line| {
            let (sensor_input, beacon_input) = line.split_once(':').unwrap();
            let sensor = {
                let (x_input, y_input) = sensor_input.split_once(',').unwrap();
                (
                    x_input
                        .replace("Sensor at x=", "")
                        .parse::<isize>()
                        .unwrap(),
                    y_input.replace(" y=", "").parse::<isize>().unwrap(),
                )
            };
            let beacon = {
                let (x_input, y_input) = beacon_input.split_once(',').unwrap();
                (
                    x_input
                        .replace(" closest beacon is at x=", "")
                        .parse::<isize>()
                        .unwrap(),
                    y_input.replace(" y=", "").parse::<isize>().unwrap(),
                )
            };
            (sensor, beacon)
        })
        .collect();
    (meta, lists)
}

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Sensor,
    Beacon,
    NearestSensor,
}

type Coord = (isize, isize);

#[derive(Debug)]
struct Span {
    start: isize,
    end: isize,
}

impl Span {
    fn overlaps(&self, other: &Span) -> bool {
        (self.start - 1 <= other.start && other.start <= self.end + 1)
            || (self.start - 1 <= other.end && other.end <= self.end + 1)
            || (other.start - 1 <= self.start && self.start <= other.end + 1)
            || (other.start - 1 <= self.end && self.end <= other.end + 1)
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Span {
            start: self.start.min(rhs.start),
            end: self.end.max(rhs.end),
        }
    }
}

#[derive(Debug)]
struct Map {
    positions: HashMap<Coord, Position>,
    min_y: Option<isize>,
    max_y: Option<isize>,
    min_x: Option<isize>,
    max_x: Option<isize>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..=self.max_y.unwrap() {
            for i in self.min_x.unwrap_or(0)..=self.max_x.unwrap_or(0) {
                let v = match self.positions.get(&(i, j)) {
                    Some(Position::Sensor) => 'S',
                    Some(Position::Beacon) => 'B',
                    Some(Position::NearestSensor) => '#',
                    None => '.',
                };
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new() -> Map {
        Map {
            positions: HashMap::new(),
            min_y: None,
            max_y: None,
            min_x: None,
            max_x: None,
        }
    }

    fn not_beacon_row(&self, row: isize) -> usize {
        (self.min_x.unwrap()..=self.max_x.unwrap())
            .filter_map(|i| self.positions.get(&(i, row)).map(|x| ((i, row), x)))
            .filter(|x| matches!(x.1, Position::NearestSensor))
            .count()
    }

    fn add_sensor_beacon(&mut self, sensor: Coord, beacon: Coord, row: isize) {
        self.positions.insert(sensor, Position::Sensor);
        self.positions.insert(beacon, Position::Beacon);

        let max_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        for i in sensor.0 - max_distance..=(sensor.0 + max_distance) {
            let j = row;
            if (sensor.0 - i).abs() + (sensor.1 - j).abs() <= max_distance
                && !self.positions.contains_key(&(i, j))
            {
                self.positions.insert((i, j), Position::NearestSensor);
            }
        }
        self.max_x = self.max_x.max(Some(sensor.0 + max_distance));
        self.max_y = self.max_y.max(Some(sensor.1 + max_distance));
        self.min_x = self
            .min_x
            .map(|x| x.min(sensor.0 - max_distance))
            .or(Some(sensor.0 - max_distance));
        self.min_y = self
            .min_x
            .map(|x| x.min(sensor.1 - max_distance))
            .or(Some(sensor.1 - max_distance));
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let ((row, _), lists) = parse_input(buf);

    let mut map = Map::new();

    for pair in lists {
        map.add_sensor_beacon(pair.0, pair.1, row);
    }

    map.not_beacon_row(row).to_string()
}

struct Map2 {
    min: Option<Coord>,
    max: Option<Coord>,
    spans: Vec<Vec<Span>>,
}

impl Map2 {
    fn new_with_size(min: Coord, max: Coord) -> Map2 {
        Map2 {
            min: Some(min),
            max: Some(max),
            spans: (min.0..=max.0).map(|_| Vec::new()).collect(),
        }
    }

    fn collapse_spans(&mut self) {
        for v in self.spans.iter_mut() {
            v.sort_by_key(|span| span.start);
            let tmp = v.remove(0);
            let new_v = v.drain(..).fold(vec![tmp], |mut current, next| {
                let s = current.pop().unwrap();
                if s.overlaps(&next) {
                    current.push(s + next);
                } else {
                    current.push(s);
                    current.push(next);
                }
                current
            });
            *v = new_v;
        }
    }

    fn not_beacon(&mut self) -> Coord {
        self.collapse_spans();

        let max = self.max.unwrap();
        let min = self.min.unwrap();

        for i in min.0..=max.0 {
            let mut j = min.1;
            for span in &self.spans[i as usize] {
                if span.start <= j && j <= span.end {
                    j = span.end + 1;
                } else if j < max.1 {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }

    fn add_sensor_beacon(&mut self, sensor: Coord, beacon: Coord) {
        let max_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let max = self.max.unwrap_or((isize::MAX, isize::MAX));
        let min = self.min.unwrap_or((isize::MIN, isize::MIN));

        for distance in 0..max_distance {
            self.spans[((sensor.0 + distance).min(max.0)) as usize].push(Span {
                start: (sensor.1 - max_distance + distance).max(min.1),
                end: (sensor.1 + max_distance - distance).min(max.1),
            });

            self.spans[((sensor.0 - distance).max(min.0)) as usize].push(Span {
                start: (sensor.1 - max_distance + distance).max(min.1),
                end: (sensor.1 + max_distance - distance).min(max.1),
            });
        }
    }
}

impl Display for Map2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.min.unwrap().0..=self.max.unwrap().0 {
            for j in self.min.unwrap().0..=self.max.unwrap().0 {
                write!(
                    f,
                    "{}",
                    self.spans[i as usize]
                        .iter()
                        .find(|span| span.start <= j && j <= span.end)
                        .map(|_x| '#')
                        .unwrap_or('.')
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let ((_row, max_search), lists) = parse_input(buf);

    let mut map = Map2::new_with_size((0, 0), (max_search, max_search));

    for pair in lists {
        map.add_sensor_beacon(pair.0, pair.1);
    }

    let beacon = map.not_beacon();

    (beacon.0 * 4000000 + beacon.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"10 20
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            )),
            "26"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"10 20
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            )),
            "56000011"
        );
    }
}

//3255950337485 - low
