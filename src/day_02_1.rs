use std::isize;

pub fn solve(input: &str) -> usize {
    input.lines().into_iter().fold(0, |acc, line| {
        let line = line
            .split_whitespace()
            .map(|f| f.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut sort = line.clone();
        let mut sort_rev = line.clone();

        sort.sort_by(|x, y| y.cmp(x));
        sort_rev.sort_by(|x, y| x.cmp(y));

        acc + if sort == line || sort_rev == line {
            match line
                .into_iter()
                .enumerate()
                .fold(None, |acc, (i, val)| match i {
                    0 => Some(val),
                    _ => match acc {
                        Some(acc) => match (1..=3).contains(&acc.abs_diff(val)) {
                            true => Some(val),
                            _ => None,
                        },
                        _ => None,
                    },
                }) {
                None => 0,
                _ => 1,
            }
        } else {
            0
        }
    })
}
