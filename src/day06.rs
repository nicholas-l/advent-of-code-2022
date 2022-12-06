use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    buf.chars()
        .tuple_windows()
        .position(|(a, b, c, d)| a != b && a != c && a != d && b != c && b != d && c != d)
        .map(|p| p + 4)
        .unwrap()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let chars: Vec<char> = buf.chars().collect();
    chars
        .windows(14)
        .position(|v| {
            let hs: HashSet<char> = HashSet::from_iter(v.iter().cloned());
            hs.len() == v.len()
        })
        .map(|p| p + 14)
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
            star_one(Cursor::new(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            "7"
        );
        assert_eq!(star_one(Cursor::new(b"bvwbjplbgvbhsrlpgdmjqwftvncz")), "5");
        assert_eq!(star_one(Cursor::new(b"nppdvjthqldpwncqszvftbrmjlhg")), "6");
        assert_eq!(
            star_one(Cursor::new(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "10"
        );

        assert_eq!(
            star_one(Cursor::new(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "11"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            "19"
        );
        assert_eq!(star_two(Cursor::new(b"bvwbjplbgvbhsrlpgdmjqwftvncz")), "23");
        assert_eq!(star_two(Cursor::new(b"nppdvjthqldpwncqszvftbrmjlhg")), "23");
        assert_eq!(
            star_two(Cursor::new(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "29"
        );
        assert_eq!(
            star_two(Cursor::new(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "26"
        );
    }
}
