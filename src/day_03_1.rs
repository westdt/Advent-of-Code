pub fn solve(input: &str) -> usize {
    input
        .split("mul(")
        .filter_map(|f| match f.split(")").next() {
            Some(value) => match value.split_once(',') {
                Some((left, right)) => {
                    Some(left.parse::<usize>().unwrap_or(0) * right.parse::<usize>().unwrap_or(0))
                }
                None => None,
            },
            None => None,
        })
        .sum()
}
