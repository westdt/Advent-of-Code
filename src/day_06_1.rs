pub fn solve(input: &str) -> usize {
    let mut visited = vec![];
    let mut dir = 2;
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut pos = (0, 0);

    enum Object {
        Obstacle,
        Nothing,
    }

    let data = input
        .lines()
        .enumerate()
        .map(|(x, f)| {
            f.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        pos = (x as isize, y as isize);
                        Object::Nothing
                    }
                    '#' => Object::Obstacle,
                    _ => Object::Nothing,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = data[0].len() as isize;
    let height = data.len() as isize;

    while (0..width).contains(&pos.0) && (0..height).contains(&pos.1) {
        visited.push(pos.clone());

        let next_pos = loop {
            let next_pos = directions[dir % directions.len()];
            let next_pos = (pos.0 + next_pos.0, pos.1 + next_pos.1);

            match data
                .get(next_pos.0 as usize)
                .unwrap_or(&vec![])
                .get(next_pos.1 as usize)
            {
                Some(x) => match x {
                    Object::Obstacle => dir += 1,
                    _ => break next_pos,
                },
                _ => break next_pos,
            }
        };

        pos = next_pos;
    }

    visited.sort();
    visited.dedup();
    visited.len()
}
