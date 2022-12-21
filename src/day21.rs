use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone)]
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
            x => panic!("Found {}", x),
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
}

#[derive(Debug, Clone)]
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
            Token::Symbol(s) => format!("({})", equations.get(s).unwrap().print(equations)),
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
                let v2 = e2.calculate(&equations);
                op.calculate(*v1, v2)
            }
            (Token::Symbol(_), None, None) => todo!(),
            (Token::Symbol(_), None, Some(_)) => todo!(),
            (Token::Symbol(_), Some(_), None) => todo!(),
            (Token::Symbol(s1), Some(op), Some(Token::Value(v2))) => {
                let e1 = equations.get(s1).unwrap();
                let v1 = e1.calculate(&equations);
                op.calculate(v1, *v2)
            }

            (Token::Symbol(s1), Some(op), Some(Token::Symbol(s2))) => {
                let e1 = equations.get(s1).unwrap();
                let v1 = e1.calculate(&equations);
                let e2 = equations.get(s2).unwrap();
                let v2 = e2.calculate(&equations);
                op.calculate(v1, v2)
            }
        }
    }

    fn is_constant(&self) -> bool {
        match (&self.0, &self.2) {
            (Token::Value(_), None) => true,
            (Token::Value(_), Some(Token::Value(_))) => true,
            _ => false,
        }
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
}

fn simplify(
    equations: HashMap<String, Equation>,
    ignore: HashSet<String>,
) -> HashMap<String, Equation> {
    let mut constants = HashMap::<String, isize>::new();
    let mut equations2 = equations.clone();

    let mut changed = true;
    while changed {
        changed = false;
        for (name, equation) in equations2.iter_mut() {
            if ignore.contains(name) {
                continue;
            }
            if !constants.contains_key(name) && equation.is_constant() {
                changed = true;
                let v = equation.value();
                constants.insert(name.clone(), v);
                equation.0 = Token::Value(v);
                equation.1 = None;
                equation.2 = None;
            } else if !constants.contains_key(name) {
                if let Token::Symbol(s) = &equation.0 {
                    if let Some(value) = constants.get(s) {
                        changed = true;

                        equation.0 = Token::Value(*value);
                    }
                }
                if let Some(Token::Symbol(s)) = &equation.2 {
                    if let Some(value) = constants.get(s) {
                        changed = true;

                        equation.2 = Some(Token::Value(*value));
                    }
                }
            }
        }
    }
    println!("{:?}", constants);
    equations2
}

fn parse_input(buf: &str) -> HashMap<String, Equation> {
    let equations = buf
        .lines()
        .map(|line| {
            let (name, equation_input) = line.split_once(": ").unwrap();
            let mut parts = equation_input.split(' ');
            let t1 = Token::parse(parts.next().unwrap());
            let op = parts.next().map(|p| Operation::parse(p));
            let t2 = parts.next().map(|p| Token::parse(p));

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

    let mut equations = simplify(equations, ignore);

    // println!("{:#?}", equations);
    println!("{}", equations.get("root").unwrap().print(&equations));
    // panic!();

    //print equation

    for i in 8317900000.. {
        if i % 100_000 == 0 {
            println!("{}", i);
        }
        let human = equations.get_mut("humn").unwrap();
        human.0 = Token::Value(i);
        if equations.get("root").unwrap().calculate(&equations) == 1 {
            return i.to_string();
        }
    }
    equations
        .get("root")
        .unwrap()
        .calculate(&equations)
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
