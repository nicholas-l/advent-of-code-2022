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
            true
        } else {
            false
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
            true
        } else {
            false
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

#[derive(Debug, PartialEq, Eq)]
struct Cavern2 {
    positions: HashMap<usize, [bool; 7]>,
}

impl Cavern2 {
    fn new() -> Cavern2 {
        Cavern2 {
            positions: HashMap::new(),
        }
    }

    fn command(&self, rock: &mut Rock, command: &Command) -> bool {
        let dir = match command {
            Command::Right => 1,
            Command::Left => -1,
        };

        if rock.template.iter().any(|p| {
            let new_pos = (rock.origin.0 + p.0 + dir, rock.origin.1 + p.1);
            new_pos.0 < 0
                || new_pos.0 >= 7
                || self
                    .positions
                    .get(&(new_pos.1 as usize))
                    .map(|x| x[new_pos.0 as usize])
                    .unwrap_or(false)
        }) {
            false
        } else {
            rock.origin.0 += dir;
            true
        }
    }

    fn fall(&mut self, rock: &mut Rock) -> bool {
        if rock.template.iter().any(|p| {
            self.positions
                .get(&((rock.origin.1 + p.1 - 1) as usize))
                .map(|c| c[(rock.origin.0 + p.0) as usize])
                .unwrap_or(false)
                || rock.origin.1 + p.1 - 1 < 0
        }) {
            false
        } else {
            rock.origin.1 -= 1;
            true
        }
    }

    fn heights(&self) -> [isize; 7] {
        let mut heights = [0; 7];
        for (y, row) in self.positions.iter() {
            // println!("{:?} {:?}", y, row);
            for (x, col) in row.iter().enumerate() {
                // println!("\t{:?} {:?}: {:?}", x, col, heights);
                if *col {
                    heights[x] = heights[x].max(*y as isize + 1);
                }
            }
        }
        heights
    }

    fn max_height(&self) -> isize {
        *self.heights().iter().max().unwrap_or(&0)
    }

    fn place_rock(&mut self, rock: Rock) {
        for (x, y) in rock.positions() {
            self.positions.entry(y as usize).or_insert([false; 7])[x as usize] = true;
        }
        if self.positions.keys().len() > 50 {
            let min = *self.positions.keys().min().unwrap();
            self.positions.remove(&min);
        }
    }

    fn relative_heights(&self) -> [isize; 7] {
        let mut heights = [0; 7];
        let min = self.min_height();
        let absolute_heights = self.heights();
        for (i, height) in heights.iter_mut().enumerate() {
            *height = absolute_heights[i] - min;
        }
        heights
    }

    fn min_height(&self) -> isize {
        *self.heights().iter().min().unwrap()
    }

    fn drop_rock(&mut self, mut rock: Rock, commands: &[Command], command_index: &mut usize) {
        let mut did_fall = true;
        rock.set_current_position((2, self.max_height() as i64 + 3));
        while did_fall {
            let command = &commands[*command_index];
            self.command(&mut rock, command);
            *command_index += 1;
            *command_index %= commands.len();
            did_fall = self.fall(&mut rock);
        }
        self.place_rock(rock);
    }

    fn add_height(&mut self, height: isize) {
        let new_positions = self
            .positions
            .iter()
            .map(|(y, row)| (y + height as usize, *row))
            .collect::<HashMap<_, _>>();
        self.positions = new_positions;
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
        let mut rock = rock_iter.next().unwrap().clone();
        let mut did_fall = true;
        rock.set_current_position((2, cavern.max_y + 4));

        while did_fall {
            if let Some(command) = commands.next() {
                rock.command(&cavern, command);
            }
            did_fall = rock.fall(&cavern);
        }
        cavern.positions.extend(rock.positions());
        cavern.max_y = cavern.max_y.max(rock.highest());

        rock_count += 1;
    }

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

    let mut command_index = 0;

    let mut rock_count = 0;

    let finish_rock_count = 1_000_000_000_000;
    // 1_199_999_999_961
    let mut seen = HashMap::new();

    while rock_count <= finish_rock_count {
        let rock_index = rock_count % rock_types.len();
        // println!("{}: {:?}", rock_count, cavern.heights());
        if let Some((seen_rock_count, seen_height)) =
            seen.get(&(cavern.relative_heights(), rock_index, command_index))
        {
            // println!(
            //     "We have seen this before! {} {} {} {}",
            //     rock_count,
            //     seen_rock_count,
            //     seen_height,
            //     cavern.min_height()
            // );
            // We have seen this before so we can skip ahead
            let diff = rock_count - seen_rock_count;
            let remaining = finish_rock_count - rock_count;
            let skip = remaining / diff;
            assert!(
                skip * seen_rock_count + rock_count <= finish_rock_count,
                "{}",
                skip * seen_rock_count + rock_count
            );
            // println!(
            //     "{}, {} {} {} {}",
            //     rock_count, diff, remaining, skip, finish_rock_count
            // );
            cavern.add_height((cavern.min_height() - seen_height) * skip as isize);
            rock_count += diff * skip;
            // rock_count += skip * diff;
            // println!("{}: {:?}", rock_count, cavern.heights());
            // command_index += 1;
            // command_index %= commands.len();
            // break
        } else {
            seen.insert(
                (cavern.relative_heights(), rock_index, command_index),
                (rock_count, cavern.min_height()),
            );
        }

        let rock = rock_types[rock_index].clone();

        cavern.drop_rock(rock, &commands, &mut command_index);

        rock_count += 1;
        // if rock_count >= 999999999950 || rock_count == 999999999995 {
        //     println!("{}: {:?}", rock_count, cavern.heights());
        //     break;
        // }
    }

    (cavern.max_height() - 1).to_string()
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
        assert!(res);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 3),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 2),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 1),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 0),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(!did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 0),
            }
        );

        cavern.place_rock(rock);

        assert_eq!(cavern.heights(), [4, 0, 0, 0, 0, 0, 0]);

        let mut rock = Rock {
            template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            template_origin: (0, 0),
            origin: (0, 6),
        };

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 5),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            }
        );

        let did_fall = cavern.fall(&mut rock);
        assert!(!did_fall);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (0, 4),
            },
            "rock should not have fallen, {:?}",
            cavern.positions
        );

        cavern.place_rock(rock);
        assert_eq!(cavern.heights(), [8, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_last_spot() {
        let mut cavern = Cavern2::new();

        let mut rock = Rock {
            template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            template_origin: (0, 0),
            origin: (6, 1),
        };
        let command = Command::Right;

        let res = cavern.command(&mut rock, &command);
        assert!(!res);

        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (6, 1),
            }
        );
        let res = cavern.fall(&mut rock);
        assert!(res);
        assert_eq!(
            rock,
            Rock {
                template: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                template_origin: (0, 0),
                origin: (6, 0),
            }
        );

        assert_eq!(
            rock.positions().collect::<Vec<_>>(),
            [(6, 0), (6, 1), (6, 2), (6, 3)]
        );

        cavern.place_rock(rock);

        // assert_eq!(cavern, Cavern2::new());

        assert_eq!(cavern.heights(), [0, 0, 0, 0, 0, 0, 4]);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")),
            "3068"
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")),
            "1514285714288"
        );
    }
}
