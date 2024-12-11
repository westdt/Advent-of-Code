pub fn solve(input: &str) -> usize {
    input.lines().into_iter().fold(0, |acc, line| {
        let line = line
            .split_whitespace()
            .map(|f| f.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let check = |mut line: Vec<usize>| {
            for i in 0..line.len() {
                let x = line.remove(i);
                match line
                    .windows(2)
                    .all(|f| (1..=3).contains(&f[0].abs_diff(f[1])) && f[0] > f[1])
                    || line
                        .windows(2)
                        .all(|f| (1..=3).contains(&f[0].abs_diff(f[1])) && f[0] < f[1])
                {
                    true => {
                        line.insert(i, x);
                        return 1;
                    }
                    false => line.insert(i, x),
                }
            }
            return 0;
        };

        acc + check(line)
    })
}
