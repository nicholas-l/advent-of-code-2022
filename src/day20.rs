use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Coordinate {
    Moved(isize),
    UnMoved(isize),
}

impl Coordinate {
    fn value(&self) -> isize {
        match self {
            Coordinate::Moved(x) => *x,
            Coordinate::UnMoved(x) => *x,
        }
    }
}

fn parse_input(buf: &str) -> Vec<Coordinate> {
    buf.lines()
        .map(|l| Coordinate::UnMoved(l.parse::<isize>().unwrap()))
        .collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let coordinates = parse_input(&buf);

    // println!("{:?}", coordinates);
    let mut list = VecDeque::from_iter(coordinates.iter().cloned().enumerate());
    for i in 0..coordinates.len() {
        let j = list
            .iter()
            .enumerate()
            .find(|(_k, (j, _x))| j == &i)
            .unwrap()
            .0;
        // println!("Moving {:?}", x);
        // assert_eq!(x.value(), coordinates[i].value());
        list.rotate_left(j);
        let (j2, x) = list.pop_front().unwrap();
        // assert_eq!(&x, d);
        if x.value() < 0 {
            list.rotate_right(x.value().unsigned_abs() % list.len() as usize);
        } else {
            list.rotate_left(x.value().unsigned_abs() % list.len() as usize);
        }
        list.push_front((j2, Coordinate::Moved(x.value())));
        // println!("{:?}", list);
    }

    // find 0
    let pos_0 = list
        .iter()
        .position(|(_i, c)| c == &Coordinate::Moved(0))
        .unwrap();

    let pos_1 = (pos_0 + 1000) % list.len();
    let pos_2 = (pos_0 + 2000) % list.len();
    let pos_3 = (pos_0 + 3000) % list.len();

    // println!(
    //     "{:?} - {} {} {} {}",
    //     v.iter().map(|c| c.value()).collect::<Vec<_>>(),
    //     pos_0,
    //     v[pos_1].value(),
    //     v[pos_2].value(),
    //     v[pos_3].value()
    // );

    (list[pos_1].1.value() + list[pos_2].1.value() + list[pos_3].1.value()).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let coordinates = parse_input(&buf);

    let key = 811589153;

    // println!("{:?}", coordinates);
    let mut list = VecDeque::from_iter(
        coordinates
            .iter()
            .cloned()
            .map(|x| Coordinate::UnMoved(x.value() * key))
            .enumerate(),
    );
    for _mix in 0..10 {

        for i in 0..coordinates.len() {
            let j = list
                .iter()
                .enumerate()
                .find(|(_k, (j, _x))| j == &i)
                .unwrap()
                .0;
            // println!("Moving {:?}", x);
            // assert_eq!(x.value(), coordinates[i].value());
            list.rotate_left(j);
            let (j2, x) = list.pop_front().unwrap();
            // assert_eq!(&x, d);
            if x.value() < 0 {
                list.rotate_right(x.value().unsigned_abs() % list.len() as usize);
            } else {
                list.rotate_left(x.value().unsigned_abs() % list.len() as usize);
            }
            list.push_front((j2, Coordinate::Moved(x.value())));
            // println!("{:?}", list);
        }
    
    }
    // find 0
    let pos_0 = list
        .iter()
        .position(|(_i, c)| c == &Coordinate::Moved(0))
        .unwrap();

    let pos_1 = (pos_0 + 1000) % list.len();
    let pos_2 = (pos_0 + 2000) % list.len();
    let pos_3 = (pos_0 + 3000) % list.len();

    (list[pos_1].1.value() + list[pos_2].1.value() + list[pos_3].1.value()).to_string()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"1
2
-3
3
-2
0
4
"
            )),
            "3"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"1
        2
        -3
        3
        -2
        0
        4"
            )),
            "1623178306"
        );
    }
}
