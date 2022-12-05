use std::io::BufRead;

/// Returns true if the second is contained by the first
fn range_contains(first: &(usize, usize), second: &(usize, usize)) -> bool {
    first.0 <= second.0 && first.1 >= second.1
}

fn range_overlaps(first: &(usize, usize), second: &(usize, usize)) -> bool {
    first.0 >= second.0 && first.0 <= second.1 || second.0 >= first.0 && second.0 <= first.1
}

pub fn star_one(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let first_range = {
                let (start, end) = first.split_once('-').unwrap();
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            };

            let second_range = {
                let (start, end) = second.split_once('-').unwrap();
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            };

            (first_range, second_range)
        })
        .filter(|(first, second)| range_contains(first, second) || range_contains(second, first));
    data.count().to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let first_range = {
                let (start, end) = first.split_once('-').unwrap();
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            };

            let second_range = {
                let (start, end) = second.split_once('-').unwrap();
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            };

            (first_range, second_range)
        })
        .filter(|(first, second)| range_overlaps(first, second) || range_overlaps(second, first));
    data.count().to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            "2"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            "4"
        );
    }
}
