use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
    Equals,
}

impl Operation {
    fn parse(input: &str) -> Operation {
        match input {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            "-" => Operation::Subtract,
            "/" => Operation::Divide,
            x => panic!("Found {x}"),
        }
    }

    fn calculate(&self, v1: isize, v2: isize) -> isize {
        match self {
            Operation::Add => v1 + v2,
            Operation::Multiply => v1 * v2,
            Operation::Subtract => v1 - v2,
            Operation::Divide => v1 / v2,
            Operation::Equals => isize::from(v1 == v2),
        }
    }

    fn print(&self) -> String {
        match self {
            Operation::Add => "+".to_string(),
            Operation::Multiply => "*".to_string(),
            Operation::Subtract => "-".to_string(),
            Operation::Divide => "/".to_string(),
            Operation::Equals => "=".to_string(),
        }
    }

    fn inverse(&self) -> Operation {
        match self {
            Operation::Add => Operation::Subtract,
            Operation::Multiply => Operation::Divide,
            Operation::Subtract => Operation::Add,
            Operation::Divide => Operation::Multiply,
            Operation::Equals => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    Value(isize),
    Symbol(String),
}

impl Token {
    fn parse(input: &str) -> Token {
        if let Ok(v) = input.parse::<isize>() {
            Token::Value(v)
        } else {
            Token::Symbol(input.to_string())
        }
    }

    fn print(&self, equations: &HashMap<String, Equation>) -> String {
        match self {
            Token::Value(x) => x.to_string(),
            Token::Symbol(s) => format!("({s}: {})", equations.get(s).unwrap().print(equations)),
        }
    }
}

#[derive(Debug, Clone)]
struct Equation(Token, Option<Operation>, Option<Token>);

impl Equation {
    fn calculate(&self, equations: &HashMap<String, Equation>) -> isize {
        match (&self.0, &self.1, &self.2) {
            (Token::Value(v), None, None) => *v,
            (Token::Value(_), None, Some(_)) => panic!(),
            (Token::Value(_), Some(_), None) => panic!(),
            (Token::Value(v1), Some(op), Some(Token::Value(v2))) => op.calculate(*v1, *v2),
            (Token::Value(v1), Some(op), Some(Token::Symbol(s2))) => {
                let e2 = equations.get(s2).unwrap();
                let v2 = e2.calculate(equations);
                op.calculate(*v1, v2)
            }
            (Token::Symbol(_), None, None) => todo!(),
            (Token::Symbol(_), None, Some(_)) => todo!(),
            (Token::Symbol(_), Some(_), None) => todo!(),
            (Token::Symbol(s1), Some(op), Some(Token::Value(v2))) => {
                let e1 = equations.get(s1).unwrap();
                let v1 = e1.calculate(equations);
                op.calculate(v1, *v2)
            }

            (Token::Symbol(s1), Some(op), Some(Token::Symbol(s2))) => {
                let e1 = equations.get(s1).unwrap();
                let v1 = e1.calculate(equations);
                let e2 = equations.get(s2).unwrap();
                let v2 = e2.calculate(equations);
                op.calculate(v1, v2)
            }
        }
    }

    fn is_constant(&self) -> bool {
        matches!(
            (&self.0, &self.2),
            (Token::Value(_), None) | (Token::Value(_), Some(Token::Value(_)))
        )
    }

    fn value(&self) -> isize {
        match (&self.0, &self.1, &self.2) {
            (Token::Value(v), None, None) => *v,
            (Token::Value(_), None, Some(_)) => panic!(),
            (Token::Value(_), Some(_), None) => panic!(),
            (Token::Value(v1), Some(op), Some(Token::Value(v2))) => op.calculate(*v1, *v2),
            (Token::Value(_v1), Some(_op), Some(Token::Symbol(_s2))) => panic!(),
            (Token::Symbol(_), None, None) => todo!(),
            (Token::Symbol(_), None, Some(_)) => todo!(),
            (Token::Symbol(_), Some(_), None) => todo!(),
            (Token::Symbol(_s1), Some(_op), Some(Token::Value(_v2))) => panic!(),

            (Token::Symbol(_s1), Some(_op), Some(Token::Symbol(_s2))) => panic!(),
        }
    }

    #[allow(dead_code)]
    fn print(&self, equations: &HashMap<String, Equation>) -> String {
        if let Some(op) = &self.1 {
            let right = self.2.as_ref().unwrap();
            format!(
                "{} {} {}",
                self.0.print(equations),
                op.print(),
                right.print(equations)
            )
        } else {
            self.0.print(equations)
        }
    }

    fn try_solve(&self, symbol: &str) -> Option<isize> {
        if self.0 == Token::Symbol(symbol.to_string())
            && self
                .1
                .as_ref()
                .map(|op| op == &Operation::Equals)
                .unwrap_or_default()
            && self
                .2
                .as_ref()
                .map(|t| matches!(t, Token::Value(_)))
                .unwrap_or_default()
        {
            if let Some(Token::Value(v)) = self.2.as_ref() {
                Some(*v)
            } else {
                panic!()
            }
        } else if self.2 == Some(Token::Symbol(symbol.to_string()))
            && self.1 == Some(Operation::Equals)
            && matches!(self.0, Token::Value(_))
        {
            if let Token::Value(v) = self.0.clone() {
                Some(v)
            } else {
                panic!()
            }
        } else {
            None
        }
    }
}

fn simplify(equations: &mut HashMap<String, Equation>, ignore: &HashSet<String>) -> bool {
    let mut constants = HashMap::<String, isize>::new();
    let mut changed_all = false;

    let mut changed = true;
    while changed {
        changed = false;
        let keys: Vec<_> = { equations.keys().cloned().collect() };
        for name in keys {
            if ignore.contains(&name) {
                continue;
            }
            if !constants.contains_key(&name) {
                if equations.get(&name).unwrap().is_constant() {
                    changed = true;
                    changed_all = true;
                    let v = equations.get(&name).unwrap().value();
                    constants.insert(name.clone(), v);
                    let equation = equations.get_mut(&name).unwrap();
                    equation.0 = Token::Value(v);
                    equation.1 = None;
                    equation.2 = None;
                } else if let Token::Symbol(s) = &equations.get(&name).unwrap().0 {
                    if equations.get(&name).unwrap().2.is_none() {
                        let child_equation = equations.get(s).unwrap().clone();
                        let equation = equations.get_mut(&name).unwrap();
                        equation.0 = child_equation.0.clone();
                        equation.1.clone_from(&child_equation.1);
                        equation.2.clone_from(&child_equation.2);
                        changed = true;
                        changed_all = true;
                    } else if let Some(value) = constants.get(s) {
                        changed = true;
                        changed_all = true;
                        equations.get_mut(&name).unwrap().0 = Token::Value(*value);
                    }
                }
                if let Some(Token::Symbol(s)) = &equations.get(&name).unwrap().2 {
                    if let Some(value) = constants.get(s) {
                        changed = true;
                        changed_all = true;
                        equations.get_mut(&name).unwrap().2 = Some(Token::Value(*value));
                    }
                }
            }
        }
    }
    changed_all
}

fn take_constant_leaf(
    equations: &mut HashMap<String, Equation>,
    key: &str,
) -> Option<(Operation, Token, Token)> {
    let root_equation = &equations.get(key).unwrap();

    if let Token::Symbol(s) = &root_equation.0 {
        // Assume other side is value
        if let Some(Token::Value(v)) = root_equation.2.clone() {
            let operation = root_equation.1.as_ref().unwrap();

            return Some((
                operation.inverse(),
                Token::Value(v),
                Token::Symbol(s.clone()),
            ));
        } else {
            panic!("Symbols on both sides of equation");
        }
    }
    if let Some(Token::Symbol(s)) = &root_equation.2 {
        // Assume other side is value
        if let Token::Value(v) = root_equation.0.clone() {
            let operation = root_equation.1.as_ref().unwrap();
            match operation {
                Operation::Add => Some((
                    operation.inverse(),
                    Token::Value(v),
                    Token::Symbol(s.clone()),
                )),
                Operation::Multiply => Some((
                    operation.inverse(),
                    Token::Value(v),
                    Token::Symbol(s.clone()),
                )),
                Operation::Subtract => Some((
                    operation.inverse(),
                    Token::Symbol(s.clone()),
                    Token::Value(v),
                )),
                Operation::Divide => todo!(),
                Operation::Equals => todo!(),
            }
        } else {
            panic!("Symbols on both sides of equation");
        }
    } else {
        None
    }
}

fn rotate(equations: &mut HashMap<String, Equation>) -> bool {
    if let Token::Value(v) = equations.get("root").unwrap().0.clone() {
        // Left hand side is constant so lets move values from right to left
        if let Token::Symbol(ref s) = equations.get("root").unwrap().2.clone().unwrap() {
            if let Some((operation, value, token)) = take_constant_leaf(equations, s) {
                // println!("Rotating {} {} {:?}", v, operation.print(), token);
                let new_key = format!("root-{v}");
                equations.get_mut("root").unwrap().0 = Token::Symbol(new_key.clone());
                equations.get_mut("root").unwrap().2 = Some(token);
                equations.insert(
                    new_key,
                    Equation(Token::Value(v), Some(operation), Some(value)),
                );
                return true;
            }
        }
        false
    } else if let Some(Token::Value(_v)) = equations.get("root").unwrap().2.clone() {
        let root = equations.get_mut("root").unwrap();
        root.0 = root.2.replace(root.0.clone()).unwrap();
        true
    } else {
        panic!();
    }
}

fn parse_input(buf: &str) -> HashMap<String, Equation> {
    let equations = buf
        .lines()
        .map(|line| {
            let (name, equation_input) = line.split_once(": ").unwrap();
            let mut parts = equation_input.split(' ');
            let t1 = Token::parse(parts.next().unwrap());
            let op = parts.next().map(Operation::parse);
            let t2 = parts.next().map(Token::parse);

            (name.to_string(), Equation(t1, op, t2))
        })
        .collect();
    equations
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let equations = parse_input(&buf);
    equations
        .get("root")
        .unwrap()
        .calculate(&equations)
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    // TODO Simplify the equations down
    let mut equations = parse_input(&buf);

    let root = equations.get_mut("root").unwrap();
    root.1 = Some(Operation::Equals);

    let mut ignore = HashSet::new();
    ignore.insert("humn".to_string());

    let human = equations.get_mut("humn").unwrap();
    human.0 = Token::Value(-1);

    let mut changed = true;
    while changed {
        let changed1 = simplify(&mut equations, &ignore);
        // println!(
        //     "A: {}: {}",
        //     equations.get("root").unwrap().print(&equations),
        //     changed1
        // );
        let changed2 = rotate(&mut equations);
        changed = changed1 || changed2;
        // println!(
        //     "B: {}: {}",
        //     equations.get("root").unwrap().print(&equations),
        //     changed2
        // );
        // println!("{:?}", equations.get("root").unwrap().try_solve("humn"));
        if equations.get("root").unwrap().try_solve("humn").is_some() {
            break;
        }
    }

    equations
        .get("root")
        .unwrap()
        .try_solve("humn")
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            )),
            "152"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            )),
            "301"
        );
    }
}
