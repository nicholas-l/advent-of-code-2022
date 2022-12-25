use std::io::BufRead;

fn parse_input(buf: &str) -> Vec<isize> {
    buf.lines()
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    let x = match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    };
                    x * 5u64.pow(i as u32) as isize
                })
                .sum()
        })
        .collect()
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let numbers = parse_input(&buf);
    let mut total: isize = numbers.iter().sum();
    let mut v = Vec::new();

    println!("{}", total);

    while total > 0 {
        let c = match total % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!(),
        };
        v.push(c);
        total /= 5;
        if c == '-' || c == '=' {
            total += 1;
        }
    }
    v.iter().rev().map(|x| x.to_string()).collect()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    "Merry Xmas".to_string()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            )),
            "2=-1=0"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(b"")), "Merry Xmas");
    }
}
