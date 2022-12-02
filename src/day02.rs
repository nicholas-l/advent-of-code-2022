use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> usize {
  let datat = input.lines().map(|line| line.unwrap()).map(|line| line.split_once(' ')).map(|s| s.unwrap()).map(|(a, b)| {
    match a {
      "A" => match b {
        "X" => 3 + 0,
        "Y" => 8,
        "Z" => todo!(),
      }
      x => panic!("{}", x)
    }
  });
  0
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
        assert_eq!(
            star_two(Cursor::new(
                b""
            )),
            45000
        );
    }
}
