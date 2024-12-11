pub fn solve(input: &str) -> usize {
    let mut r#do = true;
    input
        .split("mul(")
        .filter_map(|f| match f.split_once(")") {
            Some((value, after)) => {
                let result = match value.split_once(',') {
                    Some((left, right)) => Some(
                        left.parse::<usize>().unwrap_or(0)
                            * right.parse::<usize>().unwrap_or(0)
                            * r#do as usize,
                    ),
                    None => None,
                };

                match after.find("do()") {
                    Some(a) => match after.find("don't()") {
                        Some(b) => r#do = a < b,
                        None => r#do = true,
                    },
                    None => match after.find("don't") {
                        Some(_) => r#do = false,
                        None => (),
                    },
                }

                result
            }
            None => None,
        })
        .sum()
}
