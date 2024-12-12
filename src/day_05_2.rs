pub fn solve(input: &str) -> usize {
    let mut rules: Vec<(usize, Vec<usize>)> = vec![];
    let mut updates = vec![];

    input.lines().for_each(|f| {
        if f.contains('|') {
            let (x, y) = f.split_once('|').unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            match rules
                .clone()
                .iter()
                .enumerate()
                .find_map(|(i, (a, _))| match a == &x {
                    true => Some(i),
                    false => None,
                }) {
                Some(i) => rules[i].1.push(y),
                None => rules.push((x, vec![y])),
            }
        } else if f.contains(',') {
            updates.push(
                f.split(',')
                    .map(|f| f.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        }
    });

    updates
        .iter()
        .filter(|v1| {
            !v1.iter()
                .enumerate()
                .all(|(i, x)| match rules.iter().find(|(a, _)| a == x) {
                    Some((_, v2)) => v2.iter().all(|y| {
                        if v1.contains(y) {
                            v1[i..v1.len()].contains(y)
                        } else {
                            true
                        }
                    }),
                    None => true,
                })
        })
        .map(|v1| {
            let mut clone = v1.clone();
            clone.sort_by(|y, x| match rules.iter().find(|(a, _)| a == x) {
                Some((_, v2)) => match v2.iter().any(|v| v == y) {
                    true => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Less,
                },
                None => std::cmp::Ordering::Less,
            });
            clone
        })
        .fold(0, |acc, v| acc + v[v.len() / 2])
}
