use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use regex::Regex;

// TODO Add check about if possible to get maximum with remaining valves and time.

struct State<'a> {
    room: &'a str,
    time_left: usize,
    pressure_released: usize,
    opened: HashSet<&'a str>,
}

struct State2<'a> {
    room: &'a str,
    elephant: &'a str,
    time_left: usize,
    pressure_released: usize,
    opened: HashSet<&'a str>,
}

fn parse_input(buf: String) -> HashMap<String, (usize, Vec<String>)> {
    let input_re = Regex::new(r"^Valve (\w\w) has flow rate=(\d+)$").unwrap();
    let output_re = Regex::new(r"leads? to valves? (.+)").unwrap();

    let lines = buf.lines().map(|line| {
        let (input, output) = line.split_once(';').unwrap();
        let (name, flow_rate) = {
            let captures = input_re
                .captures(input)
                .unwrap_or_else(|| panic!("Could not parse {}", input));
            (
                captures[1].trim().to_string(),
                captures[2].parse::<usize>().unwrap(),
            )
        };
        let output_valves = {
            let captures = output_re
                .captures(output)
                .unwrap_or_else(|| panic!("Could not parse {}", input));
            captures[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };
        (name, (flow_rate, output_valves))
    });
    lines.collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_input(buf);

    let mut stack = vec![State {
        room: "AA",
        time_left: 30,
        pressure_released: 0,
        opened: HashSet::new(),
    }]; // TODO Change to binary heap

    let mut max_pressure = 0;
    let mut i = 0;
    let mut seen = HashSet::new();

    while let Some(state) = stack.pop() {
        if i % 10000 == 0 {
            println!("{} - {}", stack.len(), state.time_left);
        }
        i += 1;
        if seen.contains(&(state.room, state.time_left, state.pressure_released)) {
            continue;
        }
        seen.insert((state.room, state.time_left, state.pressure_released));
        let (flow_rate, connected) = map
            .get(state.room)
            .unwrap_or_else(|| panic!("Failed to get '{}' from {:?}", state.room, map));
        if state.time_left == 0 {
            if max_pressure < state.pressure_released {
                max_pressure = state.pressure_released;
            }
        } else {
            if flow_rate > &0 && !state.opened.contains(&state.room) {
                let mut new_state = State {
                    room: state.room,
                    time_left: state.time_left - 1,
                    pressure_released: state.pressure_released + flow_rate * (state.time_left - 1),
                    opened: state.opened.clone(),
                };
                new_state.opened.insert(new_state.room);
                stack.push(new_state);
            }
            for valve in connected {
                stack.push(State {
                    room: valve.as_str(),
                    time_left: state.time_left - 1,
                    pressure_released: state.pressure_released,
                    opened: state.opened.clone(),
                })
            }
        }
    }
    max_pressure.to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_input(buf);

    let mut stack = vec![State2 {
        room: "AA",
        elephant: "AA",
        time_left: 26,
        pressure_released: 0,
        opened: HashSet::new(),
    }];

    let mut max_pressure = 0;
    // let mut i = 0;
    let mut seen = HashSet::new();

    while let Some(state) = stack.pop() {
        // if i % 100_000 == 0 {
        //     println!("{} - {} - {}", stack.len(), state.time_left, max_pressure);
        // }
        // i += 1;
        if seen.contains(&(
            state.room,
            state.elephant,
            state.time_left,
            state.pressure_released,
        )) {
            continue;
        }
        seen.insert((
            state.room,
            state.elephant,
            state.time_left,
            state.pressure_released,
        ));

        if state.time_left == 0 {
            if max_pressure < state.pressure_released {
                max_pressure = state.pressure_released;
            }
        } else {
            let (flow_rate, connected) = map
                .get(state.room)
                .unwrap_or_else(|| panic!("Failed to get '{}' from {:?}", state.room, map));
            let (elephant_flow_rate, elephant_connected) = map
                .get(state.elephant)
                .unwrap_or_else(|| panic!("Failed to get '{}' from {:?}", state.elephant, map));

            // Both Turn valves on
            if state.room != state.elephant
                && flow_rate > &0
                && elephant_flow_rate > &0
                && !state.opened.contains(&state.room)
                && !state.opened.contains(&state.elephant)
            {
                let mut new_state = State2 {
                    room: state.room,
                    elephant: state.elephant,
                    time_left: state.time_left - 1,
                    pressure_released: state.pressure_released
                        + flow_rate * (state.time_left - 1)
                        + elephant_flow_rate * (state.time_left - 1),
                    opened: state.opened.clone(),
                };
                new_state.opened.insert(new_state.room);
                new_state.opened.insert(new_state.elephant);
                stack.push(new_state);
            }

            // I turn valve, elephent move
            if flow_rate > &0 && !state.opened.contains(&state.room) {
                for elephant in elephant_connected.iter() {
                    let mut new_state = State2 {
                        room: state.room,
                        elephant,
                        time_left: state.time_left - 1,
                        pressure_released: state.pressure_released
                            + flow_rate * (state.time_left - 1),
                        opened: state.opened.clone(),
                    };
                    new_state.opened.insert(new_state.room);
                    stack.push(new_state);
                }
            }

            // I move, elephent turn
            if elephant_flow_rate > &0 && !state.opened.contains(&state.elephant) {
                for room in connected.iter() {
                    let mut new_state = State2 {
                        room,
                        elephant: state.elephant,
                        time_left: state.time_left - 1,
                        pressure_released: state.pressure_released
                            + elephant_flow_rate * (state.time_left - 1),
                        opened: state.opened.clone(),
                    };
                    new_state.opened.insert(new_state.elephant);
                    stack.push(new_state);
                }
            }

            // Both move
            for room in connected {
                for elephant in elephant_connected {
                    stack.push(State2 {
                        room,
                        elephant,
                        time_left: state.time_left - 1,
                        pressure_released: state.pressure_released,
                        opened: state.opened.clone(),
                    });
                }
            }
        }
    }
    max_pressure.to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            )),
            "1651"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            )),
            "1707"
        );
    }
}

//3255950337485 - low
