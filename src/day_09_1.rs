pub fn solve(input: &str) -> usize {
    let mut empty = false;
    let mut result = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            empty = !empty;
            match empty {
                true => {
                    let v = Some(i / 2);
                    let mut result = vec![];
                    for _ in 0..count {
                        result.push(v);
                    }
                    result
                }
                false => {
                    let mut result = vec![];
                    for _ in 0..count {
                        result.push(None);
                    }
                    result
                }
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    while result.contains(&None) {
        let index = result
            .iter()
            .enumerate()
            .find_map(|(i, c)| match c {
                None => Some(i),
                _ => None,
            })
            .unwrap();

        let value = loop {
            match result.pop().unwrap() {
                Some(value) => break value,
                None => continue,
            }
        };

        result.remove(index);
        result.insert(index, Some(value));
    }

    result
        .iter()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + i * c.unwrap())
}
