use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

fn parse_input(buf: &str) -> impl Iterator<Item = (isize, isize, isize, isize, isize, isize)> + '_ {
    buf.lines().map(|line| {
        let (_first, second) = line.split_once(':').unwrap();

        // Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 15 obsidian.
        let captures = RE.captures(second).unwrap();
        (
            captures[1].parse::<isize>().unwrap(),
            captures[2].parse::<isize>().unwrap(),
            captures[3].parse::<isize>().unwrap(),
            captures[4].parse::<isize>().unwrap(),
            captures[5].parse::<isize>().unwrap(),
            captures[6].parse::<isize>().unwrap(),
        )
    })
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct State {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geode: isize,
    ore_robot: isize,
    clay_robot: isize,
    obsidian_robot: isize,
    geode_robot: isize,
    time: isize,
    max_time: isize,
}

impl State {
    fn step(&mut self) {
        self.ore += self.ore_robot as isize;
        self.clay += self.clay_robot as isize;
        self.obsidian += self.obsidian_robot as isize;
        self.geode += self.geode_robot as isize;

        self.time += 1;
        self.max_time = 24;
    }

    fn is_valid(&self) -> bool {
        self.ore >= 0 && self.clay >= 0 && self.obsidian >= 0
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.geode.cmp(&other.geode)
        let left = (
            self.geode + self.geode_robot * (self.max_time - self.time),
            self.obsidian + self.obsidian_robot * (self.max_time - self.time),
            self.clay + self.clay_robot * (self.max_time - self.time),
            self.ore + self.ore_robot * (self.max_time - self.time),
            -self.time,
        );
        let right = (
            other.geode + other.geode_robot * (other.max_time - other.time),
            other.obsidian + other.obsidian_robot * (other.max_time - other.time),
            other.clay + other.clay_robot * (other.max_time - other.time),
            other.ore + other.ore_robot * (other.max_time - other.time),
            -other.time,
        );
        left.cmp(&right)
        // match (self.geode + self.geode_robot).cmp(&(other.geode + other.geode_robot)) {
        //     std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        //     std::cmp::Ordering::Equal => (self.obsidian + self.obsidian_robot).cmp(&(other.obsidian + other.obsidian_robot)),
        //     std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        // }
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let receipes: Vec<_> = parse_input(&buf).collect();

    receipes
        .into_par_iter()
        .enumerate()
        .map(|(i, receipe)| {
            let mut seen = HashSet::new();
            let mut stack = BinaryHeap::new();
            stack.push(State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                geode_robot: 0,
                time: 1,
                max_time: 24,
            });

            println!("{}: {:?}", i, receipe);
            let mut j = 0;
            // Sort stack by geode and dont continue if impossible to create more than max
            // geode_robots * (24 - time) + (24- time) * (24 - time) > max_geode
            let mut max_geode = 0;
            while let Some(curr) = stack.pop() {
                if j % 100_000 == 0 {
                    print!(
                        "{}: {:?}-{}-{}-{}-----:\r",
                        i,
                        stack.len(),
                        curr.time,
                        j,
                        max_geode
                    );
                }
                j += 1;
                if curr.time == curr.max_time {
                    max_geode = max_geode.max(curr.geode);
                } else if !seen.contains(&curr)
                    && curr.geode + curr.geode_robot * (24 - curr.time) as isize
                        + (24 - curr.time) as isize * (24 - curr.time) as isize
                        > max_geode
                {
                    seen.insert(curr.clone());

                    let possible_values = vec![
                        (0..=(curr.ore / receipe.0) as usize).collect(),
                        (0..=(curr.ore / receipe.1) as usize).collect(),
                        (0..=(curr.ore / receipe.2).min(curr.clay / receipe.3).max(0) as usize)
                            .collect(),
                        (0..=(curr.ore / receipe.4).min(curr.obsidian / receipe.5).max(0) as usize)
                            .collect(),
                    ]
                    .into_iter()
                    .map(|v: Vec<usize>| v.into_iter())
                    .multi_cartesian_product();

                    for v in possible_values {
                        let ore_robots = v[0] as isize;
                        let clay_robots = v[1] as isize;
                        let obsidian_robots = v[2] as isize;
                        let geode_robots = v[3] as isize;

                        let mut state = State {
                            ore: curr.ore
                                - (ore_robots * receipe.0)
                                - (clay_robots * receipe.1)
                                - (obsidian_robots * receipe.2)
                                - (geode_robots * receipe.4),
                            clay: curr.clay - (obsidian_robots * receipe.3),
                            obsidian: curr.obsidian - (geode_robots * receipe.5),
                            ..curr
                        };
                        if state.is_valid() {
                            state.step();
                            state.ore_robot += ore_robots;
                            state.clay_robot += clay_robots;
                            state.obsidian_robot += obsidian_robots;
                            state.geode_robot += geode_robots;
                            stack.push(state);
                        }
                    }
                }
            }
            println!("{} * {}", i, max_geode);
            (i + 1, max_geode)
        })
        .map(|(i, x)| i * x as usize)
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let _map = parse_input(&buf);

    todo!()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{star_one, star_two, State};
    use std::{collections::BinaryHeap, io::Cursor};

    #[test]
    fn test_state() {
        let mut stack = BinaryHeap::new();
        stack.push(State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 35,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            time: 0,
            max_time: 24,
        });
        stack.push(State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 30,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            time: 23,
            max_time: 24,
        });

        assert_eq!(stack.pop().unwrap().geode, 35);

        {
            let mut stack = BinaryHeap::new();
            stack.push(State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 31,
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                geode_robot: 0,
                time: 23,
                max_time: 24,
            });
            stack.push(State {
                ore: 0,
                clay: 0,
                obsidian: 5,
                geode: 31,
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                geode_robot: 0,
                time: 23,
                max_time: 24,
            });

            let res = stack.pop().unwrap();
            assert_eq!(res.geode, 31);
            assert_eq!(res.obsidian, 5);
        }

        let res: Vec<_> = vec![
            (0..=(2 / 2) as usize).collect(),
            (0..=(4 / 2) as usize).collect(),
            (0..=(4 / 1).min(2 / 1).min(0) as usize).collect(),
            (0..=(4 / 2).min(2 / 2).max(0) as usize).collect(),
        ]
        .into_iter()
        .map(|v: Vec<usize>| v.into_iter())
        .multi_cartesian_product()
        .collect();

        let expected = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 1],
            vec![0, 2, 0, 0],
            vec![0, 2, 0, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 1],
            vec![1, 1, 0, 0],
            vec![1, 1, 0, 1],
            vec![1, 2, 0, 0],
            vec![1, 2, 0, 1],
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
            )),
            "33"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            )),
            "58"
        );
    }
}
