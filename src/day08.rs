use std::io::BufRead;


/// Not a scalable solution but does the job.

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let map: Vec<Vec<u32>> = buf
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visible: Vec<Vec<bool>> = buf
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, _c)| i == 0 || i == (map.len() - 1) || j == 0 || j == (map[i].len() - 1))
                .collect()
        })
        .collect();

    for i in 0..map.len() {
        let mut max_height = map[i][0];
        for j in 0..map[i].len() {
            visible[i][j] = visible[i][j] || map[i][j] > max_height;
            max_height = max_height.max(map[i][j]);
        }
    }

    for j in 0..map[0].len() {
        let mut max_height = map[0][j];
        for i in 0..map.len() {
            visible[i][j] = visible[i][j] || map[i][j] > max_height;
            max_height = max_height.max(map[i][j]);
        }
    }

    for i in (0..map.len()).rev() {
        let mut max_height = map[i][map[i].len() - 1];
        for j in (0..map[i].len()).rev() {
            visible[i][j] = visible[i][j] || map[i][j] > max_height;
            max_height = max_height.max(map[i][j]);
        }
    }

    for j in (0..map[0].len()).rev() {
        let mut max_height = map[map.len() - 1][j];
        for i in (0..map.len()).rev() {
            visible[i][j] = visible[i][j] || map[i][j] > max_height;
            max_height = max_height.max(map[i][j]);
        }
    }

    visible
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>()
        .to_string()
}

fn get_score(map: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let up = {
        let mut max_distance = 0;
        for y in (0..i).rev() {
            max_distance += 1;
            if map[y][j] >= map[i][j] {
                break;
            }
        }
        max_distance
    };

    let down = {
        let mut max_distance = 0;
        for y in (i + 1)..map.len() {
            max_distance += 1;
            if map[y][j] >= map[i][j] {
                break;
            }
        }
        max_distance
    };

    let left = {
        let mut max_distance = 0;
        for x in (0..j).rev() {
            max_distance += 1;
            if map[i][x] >= map[i][j] {
                break;
            }
        }
        max_distance
    };

    let right = {
        let mut max_distance = 0;
        for x in (j + 1)..map[i].len() {
            max_distance += 1;
            if map[i][x] >= map[i][j] {
                break;
            }
        }
        max_distance
    };

    up * down * right * left
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);
    let map: Vec<Vec<u32>> = buf
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut max_score = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, _tree) in row.iter().enumerate() {
            max_score = max_score.max(get_score(&map, i, j));
        }
    }
    max_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"30373
25512
65332
33549
35390"
            )),
            "21"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"30373
25512
65332
33549
35390"
            )),
            "8"
        );
    }
}
