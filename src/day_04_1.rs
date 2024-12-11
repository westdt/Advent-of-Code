pub fn solve(input: &str) -> usize {
    let chars = ['X', 'M', 'A', 'S'];

    let values = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = values.len() as isize;
    let width = values[0].len() as isize;
    let offsets: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    values.iter().enumerate().fold(0, |acc, (x1, v)| {
        v.iter().enumerate().fold(0, |mut acc, (y1, _)| {
            for (ox, oy) in offsets {
                for i in 0..chars.len() {
                    let i = i as isize;
                    let x2 = x1 as isize + ox * i;
                    let y2 = y1 as isize + oy * i;

                    if (0..width).contains(&x2)
                        && (0..height).contains(&y2)
                        && values[x2 as usize][y2 as usize] == chars[i as usize]
                    {
                        if i as usize == chars.len() - 1 {
                            acc += 1
                        }
                    } else {
                        break;
                    }
                }
            }
            acc
        }) + acc
    })
}
