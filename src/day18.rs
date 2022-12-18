use std::{collections::HashSet, io::BufRead};

fn parse_input(buf: &str) -> impl Iterator<Item = (isize, isize, isize)> + '_ {
    buf.lines().map(|line| {
        let mut values = line.split(',').map(|v| v.parse::<isize>().unwrap());
        (
            values.next().unwrap(),
            values.next().unwrap(),
            values.next().unwrap(),
        )
    })
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_input(&buf);

    let mut left = HashSet::new();
    let mut right = HashSet::new();

    let mut bottom = HashSet::new();
    let mut top = HashSet::new();

    let mut front = HashSet::new();
    let mut back = HashSet::new();

    for p in map {
        left.insert(p);
        right.insert((p.0 + 1, p.1, p.2));

        bottom.insert(p);
        top.insert((p.0, p.1 + 1, p.2));

        front.insert(p);
        back.insert((p.0, p.1, p.2 + 1));
    }

    (left.symmetric_difference(&right).count()
        + bottom.symmetric_difference(&top).count()
        + front.symmetric_difference(&back).count())
    .to_string()
}

fn is_within(
    point: &(isize, isize, isize),
    bounds: &(isize, isize, isize, isize, isize, isize),
) -> bool {
    bounds.0 <= point.0
        && point.0 <= bounds.1
        && bounds.2 <= point.1
        && point.1 <= bounds.3
        && bounds.4 <= point.2
        && point.2 <= bounds.5
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let map = parse_input(&buf);

    let droplets: HashSet<(isize, isize, isize)> = map.collect();

    // Get bounds

    let bounds = (
        droplets.iter().min_by_key(|d| d.0).unwrap().0,
        droplets.iter().max_by_key(|d| d.0).unwrap().0,
        droplets.iter().min_by_key(|d| d.1).unwrap().1,
        droplets.iter().max_by_key(|d| d.1).unwrap().1,
        droplets.iter().min_by_key(|d| d.2).unwrap().2,
        droplets.iter().max_by_key(|d| d.2).unwrap().2,
    );

    let mut stack = vec![
        (bounds.0, bounds.2, bounds.4),
        (bounds.1, bounds.2, bounds.4),
        (bounds.0, bounds.3, bounds.4),
    ];
    let mut visited = HashSet::new();

    while let Some(curr) = stack.pop() {
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);

        if is_within(&curr, &bounds) && !droplets.contains(&curr) {
            stack.extend(
                [
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 1, 0),
                    (0, -1, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ]
                .iter()
                .map(|delta| (delta.0 + curr.0, delta.1 + curr.1, delta.2 + curr.2)),
            )
        }
    }

    let mut contained_air = HashSet::new();

    for i in bounds.0..=bounds.1 {
        for j in bounds.2..=bounds.3 {
            for k in bounds.4..=bounds.5 {
                let pos = (i, j, k);
                if !visited.contains(&pos) {
                    contained_air.insert(pos);
                }
            }
        }
    }

    // println!("{:?}", bounds);
    // println!("{:?}", visited);
    // println!("{:?}", droplets.contains(&(0, bounds.1, 0)));
    // println!("{:?}", is_within(&(0, bounds.1, 0), &bounds));
    // println!("{:?}", contained_air);

    let mut left = HashSet::new();
    let mut right = HashSet::new();

    let mut bottom = HashSet::new();
    let mut top = HashSet::new();

    let mut front = HashSet::new();
    let mut back = HashSet::new();

    for p in droplets {
        left.insert(p);
        right.insert((p.0 + 1, p.1, p.2));

        bottom.insert(p);
        top.insert((p.0, p.1 + 1, p.2));

        front.insert(p);
        back.insert((p.0, p.1, p.2 + 1));
    }

    for p in contained_air {
        left.insert(p);
        right.insert((p.0 + 1, p.1, p.2));

        bottom.insert(p);
        top.insert((p.0, p.1 + 1, p.2));

        front.insert(p);
        back.insert((p.0, p.1, p.2 + 1));
    }

    (left.symmetric_difference(&right).count()
        + bottom.symmetric_difference(&top).count()
        + front.symmetric_difference(&back).count())
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            )),
            "64"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            )),
            "58"
        );
    }
}
