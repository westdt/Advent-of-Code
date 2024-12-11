pub fn solve(input: &str) -> usize {
    let (a, b) = input.split_whitespace().into_iter().enumerate().fold(
        (vec![], vec![]),
        |mut acc, (i, val)| {
            if i % 2 == 0 {
                acc.0.push(val.parse::<usize>().unwrap());
            } else {
                acc.1.push(val.parse::<usize>().unwrap());
            }
            acc
        },
    );

    a.iter().fold(0, |acc, val| {
        acc + val
            * b.iter()
                .fold(0, |acc2, val2| acc2 + (*val == *val2) as usize)
    })
}
