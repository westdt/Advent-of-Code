pub fn solve(input: &str) -> usize {
    let (mut a, mut b) = input.split_whitespace().into_iter().enumerate().fold(
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

    a.sort();
    b.sort();

    a.iter().zip(b).fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}
