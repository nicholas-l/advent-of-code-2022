use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

type Coord = (i64, i64);

#[derive(Debug)]
enum Command {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    template: Vec<Coord>,
    template_origin: Coord,
    origin: Coord,
}

impl Rock {
    fn new(v: Vec<Coord>) -> Rock {
        let origin_x = v.iter().min_by_key(|x| x.0).unwrap().0;
        let origin_y = v.iter().min_by_key(|x| x.1).unwrap().1;
        Rock {
            template: v,
            template_origin: (origin_x, origin_y),
            origin: (0, 0),
        }
    }

    fn set_current_position(&mut self, wanted_position: Coord) {
        self.origin = (
            wanted_position.0 + self.template_origin.0,
            wanted_position.1 + self.template_origin.1,
        )
    }

    fn command(&mut self, cavern: &Cavern, command: &Command) -> bool {
        let dir = match command {
            Command::Right => 1,
            Command::Left => -1,
        };

        // Could change to `any`
        if !self.template.iter().any(|p| {
            let new_pos = (self.origin.0 + p.0 + dir, self.origin.1 + p.1);
            cavern.positions.contains(&new_pos) || new_pos.0 < 0 || new_pos.0 >= 7
        }) {
            self.origin.0 += dir;
            return true;
        } else {
            return false;
        }
    }

    fn fall(&mut self, cavern: &Cavern) -> bool {
        if !self.template.iter().any(|p| {
            cavern
                .positions
                .contains(&(self.origin.0 + p.0, self.origin.1 + p.1 - 1))
            // || self.origin.1 + p.1 - 1 == 0
        }) {
            self.origin.1 -= 1;
            return true;
        } else {
            return false;
        }
    }

    fn positions(&self) -> impl Iterator<Item = Coord> + '_ {
        self.template
            .iter()
            .map(|p| (self.origin.0 + p.0, self.origin.1 + p.1))
    }

    fn highest(&self) -> i64 {
        self.positions().max_by_key(|x| x.1).unwrap().1
    }
}

struct Cavern {
    positions: HashSet<Coord>,
    max_y: i64,
}

impl Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for height in (0..=self.max_y).rev() {
            writeln!(
                f,
                "{}",
                (0..7)
                    .map(|x| {
                        if self.positions.contains(&(x, height)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cavern2 {
    heights: [isize; 7],
}

impl Cavern2 {
    fn new() -> Cavern2 {
        Cavern2 { heights: [-1; 7] }
    }

    fn command(&self, rock: &mut Rock, command: &Command) -> bool {
        let dir = match command {
            Command::Right => 1,
            Command::Left => -1,
        };

        // Could change to `any`
        if rock.template.iter().any(|p| {
            let new_pos = (rock.origin.0 + p.0 + dir, rock.origin.1 + p.1);
            new_pos.0 < 0
                || new_pos.0 >= 7
                || self
                    .heights
                    .iter()
                    .enumerate()
                    .any(|p| (p.0 as i64, *p.1 as i64) == new_pos)
        }) {
            return false;
        } else {
            rock.origin.0 += dir;
            return true;
        }
    }

    fn fall(&mut self, rock: &mut Rock) -> bool {
        if rock.template.iter().any(|p| {
            self.heights[(rock.origin.0 + p.0) as usize] as i64 >= (rock.origin.1 + p.1 - 1)
        }) {
            return false;
        } else {
            rock.origin.1 -= 1;
            return true;
        }
    }

    fn max_height(&self) -> isize {
        *self.heights.iter().max().unwrap() + 1
    }

    fn place_rock(&mut self, rock: Rock) {
        for (x, y) in rock.positions() {
            self.heights[x as usize] = self.heights[x as usize].max(y as isize);
        }
    }

    fn relative_heights(&self) -> [isize; 7] {
        let mut heights = [0; 7];
        let min = self.min_height();
        for i in 0..7 {
            heights[i] = self.heights[i] - min;
        }
        heights
    }

    fn min_height(&self) -> isize {
        self.heights.iter().min().unwrap() + 1
    }
}

fn parse_input(buf: String) -> Vec<Command> {
    buf.lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => Command::Right,
            '<' => Command::Left,
            x => panic!("{}", x),
        })
        .collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let commands = parse_input(buf);

    let rock_types = [
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];

    let mut cavern = Cavern {
        positions: HashSet::new(),
        max_y: 0,
    };

    cavern.positions.insert((0, 0));
    cavern.positions.insert((1, 0));
    cavern.positions.insert((2, 0));
    cavern.positions.insert((3, 0));
    cavern.positions.insert((4, 0));
    cavern.positions.insert((5, 0));
    cavern.positions.insert((6, 0));

    let mut rock_iter = rock_types.iter().cycle();
    let mut commands = commands.iter().cycle();
    let mut rock_count = 0;

    while rock_count < 2022 {
        // println!("{}", rock_count);
        let mut rock = rock_iter.next().unwrap().clone();
        let mut did_fall = true;
        rock.set_current_position((2, cavern.max_y + 4));

        // println!("S: {:?}", rock);

        while did_fall {
            if let Some(command) = commands.next() {
                rock.command(&cavern, &command);
                // println!("1: {:?}: {} {:?}", rock, did_fall, command);
            }
            did_fall = rock.fall(&cavern);
            // println!("2: {:?}: {}", rock, did_fall);
        }
        cavern.positions.extend(rock.positions());
        cavern.max_y = cavern.max_y.max(rock.highest());
        // println!("{:?}", rock);
        // println!("{:?}", rock.positions().collect::<Vec<_>>());
        // println!("{}", cavern);
        rock_count += 1;
    }
    // println!("{}", cavern);

    cavern.max_y.to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let commands = parse_input(buf);

    let rock_types = [
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];

    let mut cavern = Cavern2::new();

    let mut rock_index = 0;
    let mut command_index = 0;

    let mut rock_count: i64 = 0;

    let finish_rock_count = 1_000_000_000_000;
    // 1_000_000_000_000;
    let mut seen = HashMap::new();

    while rock_count <= finish_rock_count {
        if let Some((seen_rock_count, seen_height)) =
            seen.get(&(cavern.relative_heights(), rock_index, command_index))
        {
            let delta_rock_count = dbg!(rock_count - seen_rock_count);
            let delta_height = dbg!(cavern.min_height() - seen_height);

            let d = dbg!((finish_rock_count - rock_count) / delta_rock_count);
            while rock_count + delta_rock_count < finish_rock_count {
                for height in cavern.heights.iter_mut() {
                    *height += d as isize * (delta_height + 1);
                }
                rock_count += d * delta_rock_count;
                rock_count += 1;
            }
            seen.clear();
        } else {
            seen.insert(
                (cavern.relative_heights(), rock_index, command_index),
                (rock_count, cavern.min_height()),
            );
        }
        let mut rock = rock_types[rock_index].clone();
        let mut did_fall = true;
        rock.set_current_position((2, cavern.max_height() as i64 + 3));

        while did_fall {
            cavern.command(&mut rock, &commands[command_index]);
            command_index += 1;
            command_index %= commands.len();

            did_fall = cavern.fall(&mut rock);
        }
        cavern.place_rock(rock);

        rock_count += 1;
        rock_index += 1;
        rock_index %= rock_types.len();
    }

    (cavern.max_height() + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, Cavern2, Command, Rock};
    use std::io::Cursor;

    #[test]
    fn test_cavern2() {
        let mut cavern = Cavern2::new();
        let mut rock = Rock {
            template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            template_origin: (0, 0),
            origin: (1, 4),
        };
        let res = cavern.command(&mut rock, &Command::Left);
        assert_eq!(res, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 3),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 2),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 1),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 0),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, false);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 0),
            }
        );

        cavern.place_rock(rock);

        assert_eq!(cavern.heights, [3, -1, -1, -1, -1, -1, -1]);

        let mut rock = Rock {
            template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            template_origin: (0, 0),
            origin: (0, 6),
        };

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 5),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, true);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert_eq!(did_fall, false);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            }
        );

        cavern.place_rock(rock);
        assert_eq!(cavern.heights, [7, -1, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")),
            "3068"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")),
            "1514285714288"
        );
    }
}

// 1299667774096 low
////1299003322270
// 1_300_332_225_922 low
// 690_842_490_843
// 1563085399432 ?
// 1411042944771
// 1563085399432
