use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> String {
    let data = input.lines().map(|line| line.unwrap()).map(|line| {
        let (a, b) = line.split_once(' ').unwrap();
        match a {
            "A" => match b {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3,
                x => panic!("B didnt match: {}", x),
            },
            "B" => match b {
                "X" => 1,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                x => panic!("B didnt match: {}", x),
            },
            "C" => match b {
                "X" => 1 + 6,
                "Y" => 2,
                "Z" => 3 + 3,
                x => panic!("B didnt match: {}", x),
            },
            x => panic!("{}", x),
        }
    });
    data.sum::<usize>().to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            match a {
                "A" => match b {
                    "X" => ("A", "C"),
                    "Y" => ("A", "A"),
                    "Z" => ("A", "B"),
                    x => panic!("B didnt match: {}", x),
                },
                "B" => match b {
                    "X" => ("B", "A"),
                    "Y" => ("B", "B"),
                    "Z" => ("B", "C"),
                    x => panic!("B didnt match: {}", x),
                },
                "C" => match b {
                    "X" => ("C", "B"),
                    "Y" => ("C", "C"),
                    "Z" => ("C", "A"),
                    x => panic!("B didnt match: {}", x),
                },
                x => panic!("{}", x),
            }
        })
        .map(|(a, b)| {
            println!("{} {}", a, b);
            match a {
                "A" => match b {
                    "A" => 1 + 3,
                    "B" => 2 + 6,
                    "C" => 3,
                    x => panic!("B didnt match: {}", x),
                },
                "B" => match b {
                    "A" => 1,
                    "B" => 2 + 3,
                    "C" => 3 + 6,
                    x => panic!("B didnt match: {}", x),
                },
                "C" => match b {
                    "A" => 1 + 6,
                    "B" => 2,
                    "C" => 3 + 3,
                    x => panic!("B didnt match: {}", x),
                },
                x => panic!("{}", x),
            }
        });
    data.sum::<usize>().to_string()
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
            "15"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"A Y
B X
C Z"
            )),
            "12"
        );
    }
}
