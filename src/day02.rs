use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> usize {
    let data = input.lines().map(|line| line.unwrap()).map(|line| {
        let (a, b) = line.split_once(' ').unwrap();
        match a {
            "A" => match b {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                x => panic!("B didnt match: {}", b),
            },
            "B" => match b {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                x => panic!("B didnt match: {}", b),
            },
            "C" => match b {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                x => panic!("B didnt match: {}", b),
            },
            x => panic!("{}", x),
        }
    });
    data.sum()
    // let mut buf = String::new();
    // let _res = input.read_to_string(&mut buf);
    // buf.split("\n\n")
    //     .map(|elve| elve.lines().map(|x| x.parse::<usize>().unwrap()).sum())
    //     .max()
    //     .unwrap()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    todo!()
    // input.lines().split()
    // let mut buf = String::new();
    // let _res = input.read_to_string(&mut buf);
    // let mut elves = buf
    //     .split("\n\n")
    //     .map(|elve| elve.lines().map(|x| x.parse::<usize>().unwrap()).sum())
    //     .collect::<Vec<_>>();
    // elves.sort();
    // elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"A Y
B X
C Z"
            )),
            15
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(b"")), 45000);
    }
}
