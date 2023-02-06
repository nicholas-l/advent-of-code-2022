use std::{cmp::Reverse, io::BufRead};

use regex::Regex;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiply,
    Divide,
    Subtract,
}

#[derive(Debug)]
enum Amount {
    Old,
    Value(usize),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: (Operation, Amount),
    divisor: usize,
    true_throw_to: usize,
    false_throw_to: usize,
    inspected: usize,
}

fn parse_monkeys(buf: String) -> Vec<Monkey> {
    let operation_regex = Regex::new(r"new = (\w+) ([\*\+/-]) (\w+)$").unwrap();

    buf.split("\n\n")
        .map(|monkey_input| {
            let lines = monkey_input.lines().collect::<Vec<_>>();

            let items = {
                let (_, end) = lines[1].split_once(':').unwrap();
                end.split(',')
                    .map(|item| item.trim().parse::<usize>().unwrap())
                    .collect()
            };

            let operation = {
                let (_, end) = lines[2].split_once(':').unwrap();
                let capture = operation_regex
                    .captures(end)
                    .unwrap_or_else(|| panic!("Could not find operation in {end}"));
                assert_eq!(&capture[1], "old");
                let op = match &capture[2] {
                    "+" => Operation::Addition,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    "-" => Operation::Subtract,
                    x => panic!("Found unexpected operation: {x}"),
                };
                let amount = if let Ok(x) = capture[3].parse::<usize>() {
                    Amount::Value(x)
                } else {
                    Amount::Old
                };
                (op, amount)
            };

            let divisor = {
                let divisor_regex = Regex::new(r"divisible by (\d+)$").unwrap();
                let divisor = divisor_regex.captures(lines[3]).unwrap()[1]
                    .parse::<usize>()
                    .unwrap();
                divisor
            };

            let true_throw_to = {
                let divisor_regex = Regex::new(r"If true: throw to monkey (\d)").unwrap();
                let divisor = divisor_regex.captures(lines[4]).unwrap()[1]
                    .parse::<usize>()
                    .unwrap();
                divisor
            };

            let false_throw_to = {
                let divisor_regex = Regex::new(r"If false: throw to monkey (\d)").unwrap();
                let divisor = divisor_regex.captures(lines[5]).unwrap()[1]
                    .parse::<usize>()
                    .unwrap();
                divisor
            };

            Monkey {
                items,
                operation,
                divisor,
                true_throw_to,
                false_throw_to,
                inspected: 0,
            }
        })
        .collect()
}

fn simulate(monkeys: &mut Vec<Monkey>, reduce_worry: bool) {
    let max_div: usize = monkeys.iter().map(|monkey| monkey.divisor).product();
    for i in 0..monkeys.len() {
        let items_length = monkeys.get(i).unwrap().items.len();
        while let Some(item) = monkeys.get_mut(i).unwrap().items.pop() {
            let mut new_item = match monkeys[i].operation {
                (Operation::Addition, Amount::Old) => item + item,
                (Operation::Addition, Amount::Value(x)) => item + x,
                (Operation::Multiply, Amount::Old) => item * item,
                (Operation::Multiply, Amount::Value(x)) => item * x,
                (Operation::Divide, Amount::Old) => 1,
                (Operation::Divide, Amount::Value(x)) => item / x,
                (Operation::Subtract, Amount::Old) => 0,
                (Operation::Subtract, Amount::Value(x)) => item - x,
            };
            if reduce_worry {
                new_item /= 3;
            } else {
                new_item %= max_div;
            }
            if new_item % monkeys.get(i).unwrap().divisor == 0 {
                let true_throw_to = monkeys.get(i).unwrap().true_throw_to;
                monkeys.get_mut(true_throw_to).unwrap().items.push(new_item);
            } else {
                let false_throw_to = monkeys.get(i).unwrap().false_throw_to;
                monkeys
                    .get_mut(false_throw_to)
                    .unwrap_or_else(|| panic!("Could not find {}", false_throw_to - 1))
                    .items
                    .push(new_item);
            }
        }
        monkeys.get_mut(i).unwrap().inspected += items_length;
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let mut monkeys = parse_monkeys(buf);
    for _round in 0..20 {
        simulate(&mut monkeys, true)
    }
    monkeys.sort_by_key(|m| Reverse(m.inspected));
    (monkeys[0].inspected * monkeys[1].inspected).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let mut monkeys = parse_monkeys(buf);

    for _round in 0..10000 {
        simulate(&mut monkeys, false)
    }
    monkeys.sort_by_key(|m| Reverse(m.inspected));
    (monkeys[0].inspected * monkeys[1].inspected).to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1"
            )),
            "10605"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1"
            )),
            "2713310158"
        );
    }
}
