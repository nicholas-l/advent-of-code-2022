use std::{collections::HashSet, io::BufRead};

fn get_priority(c: &char) -> usize {
    let offset = if c.is_ascii_lowercase() {
        'a' as usize - 1
    } else {
        'A' as usize - 27
    };
    *c as usize - offset
}

pub fn star_one(input: impl BufRead) -> String {
    let data = input.lines().map(|line| line.unwrap()).map(|line| {
        let (first, last) = line.split_at(line.len() / 2);
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let last_set = HashSet::from_iter(last.chars());

        first_set
            .intersection(&last_set)
            .map(get_priority)
            .sum::<usize>()
    });
    data.sum::<usize>().to_string()
}

pub fn star_two(input: impl BufRead) -> String {
    let data = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|lines| {
            let line0_set: HashSet<char> = HashSet::from_iter(lines[0].chars());
            let line1_set = HashSet::from_iter(lines[1].chars());
            let line2_set = HashSet::from_iter(lines[2].chars());

            let int: HashSet<_> = line0_set
                .intersection(&line1_set)
                .map(|s| s.to_owned())
                .collect();

            let possible_badges = int
                .intersection(&line2_set)
                .map(|c| c.to_owned())
                .collect::<Vec<char>>();

            possible_badges
        })
        .collect::<Vec<_>>();

    // data.sort_by_key(|possible_badges| possible_badges.len());

    data.iter().flatten().map(get_priority).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            "157"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            "70"
        );
    }
}
