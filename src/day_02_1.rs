pub fn solve(input: &str) -> usize {
    input.lines().into_iter().fold(0, |acc, line| {
        let line = line
            .split_whitespace()
            .map(|f| f.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        acc + (line
            .windows(2)
            .all(|f| (1..=3).contains(&f[0].abs_diff(f[1])) && f[0] > f[1])
            || line
                .windows(2)
                .all(|f| (1..=3).contains(&f[0].abs_diff(f[1])) && f[0] < f[1]))
            as usize
    })
}
