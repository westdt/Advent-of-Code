pub fn solve(input: &str) -> usize {
    let data = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = data[0].len() as isize;
    let height = data[1].len() as isize;

    let mut antinodes = vec![];

    for x1 in 0..width {
        for y1 in 0..height {
            for x2 in 0..width {
                for y2 in 0..height {
                    if (x1, y1) == (x2, y2) {
                        continue;
                    }

                    let c1 = data[x1 as usize][y1 as usize];
                    let c2 = data[x2 as usize][y2 as usize];

                    if c1 != c2 || c1 == '.' {
                        continue;
                    }

                    let xd = x2 - x1;
                    let yd = y2 - y1;

                    antinodes.push((x1, y1));
                    antinodes.push((x2, y2));

                    let mut i = 0;
                    loop {
                        i += 1;
                        let x3 = x1 - xd * i;
                        let y3 = y1 - yd * i;
                        let x4 = x2 + xd * i;
                        let y4 = y2 + yd * i;

                        let mut power = 0;

                        if (0..width).contains(&x3) && (0..height).contains(&y3) {
                            antinodes.push((x3, y3));
                            power += 1;
                        }

                        if (0..width).contains(&x4) && (0..height).contains(&y4) {
                            antinodes.push((x4, y4));
                            power += 1;
                        }

                        if power == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
}
