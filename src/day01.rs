use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    buf.split("\n\n")
        .map(|elve| {
            elve.lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let mut elves = buf
        .split("\n\n")
        .map(|elve| elve.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .collect::<Vec<_>>();
    elves.sort();
    elves.iter().rev().take(3).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            "24000"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            "45000"
        );
    }
}
