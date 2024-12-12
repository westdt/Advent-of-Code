enum Object {
    Obstacle,
    Nothing,
}

pub fn solve(input: &str) -> usize {
    let mut obstructors = vec![];
    let mut dir;
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut pos = (0, 0);
    let mut start_pos = (0, 0);

    let data = input
        .lines()
        .enumerate()
        .map(|(x, f)| {
            f.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        pos = (x as isize, y as isize);
                        start_pos = pos;
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

    for obx in 0..width {
        for oby in 0..height {
            if (obx, oby) == pos {
                continue;
            }

            pos = start_pos;
            dir = 2;

            'inner: while (0..width).contains(&pos.0) && (0..height).contains(&pos.1) {
                let next_pos = 'find: loop {
                    let next_pos = directions[dir % directions.len()];
                    let next_pos = (pos.0 + next_pos.0, pos.1 + next_pos.1);

                    if next_pos == (obx, oby) {
                        dir += 1
                    } else {
                        match data
                            .get(next_pos.0 as usize)
                            .unwrap_or(&vec![])
                            .get(next_pos.1 as usize)
                        {
                            Some(x) => match x {
                                Object::Obstacle => dir += 1,
                                _ => break 'find next_pos,
                            },
                            _ => break 'find next_pos,
                        }
                    }
                };

                pos = next_pos;

                if dir as isize > width * height {
                    obstructors.push((obx, oby));
                    break 'inner;
                }
            }
        }
    }

    obstructors.len() + 1
}
