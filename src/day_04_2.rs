pub fn solve(input: &str) -> usize {
    let values = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = values.len() as isize;
    let width = values[0].len() as isize;
    let offsets: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    values.iter().enumerate().fold(0, |acc, (x1, v)| {
        v.iter().enumerate().fold(0, |mut acc, (y1, c)| {
            if c == &'A' {
                let mut result = vec![];
                offsets.iter().for_each(|(ox, oy)| {
                    let x2 = x1 as isize + ox;
                    let y2 = y1 as isize + oy;

                    if (0..width).contains(&x2) && (0..height).contains(&y2) {
                        match values[x2 as usize][y2 as usize] {
                            'M' => result.push('M'),
                            'S' => result.push('S'),
                            _ => {}
                        }
                    }
                });

                if result.len() == 4 {
                    let a = result[0];
                    if result[3] != a {
                        if (result[2] == a && result[1] != a) || (result[1] == a && result[2] != a)
                        {
                            acc += 1
                        }
                    }
                }
            }
            acc
        }) + acc
    })
}
