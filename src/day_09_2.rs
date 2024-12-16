pub fn solve(input: &str) -> usize {
    let mut empty = false;
    let mut result = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            empty = !empty;
            match empty {
                true => (Some(i / 2), count),
                false => (None, count),
            }
        })
        .collect::<Vec<_>>();

    for i in (0..result.len()).rev() {
        let (item, item_count) = result[i];
        match item {
            Some(v) => {
                for j in 0..result.len() {
                    let (slot, slot_count) = result[j];
                    match slot {
                        None => {
                            if slot_count >= item_count {
                                if j < i {
                                    let diff = slot_count - item_count;
                                    if diff == 0 {
                                        result.swap(i, j);
                                    } else {
                                        result.swap(i, j);
                                        result[i] = (None, item_count);
                                        result.insert(j + 1, (None, diff));
                                    }

                                    break;
                                }
                            }
                        }
                        Some(_) => {}
                    }
                }
            }
            _ => {}
        }
    }

    result
        .iter()
        .map(|(v, c)| match v {
            Some(v) => v.to_string().repeat(*c),
            None => 0.to_string().repeat(*c),
        })
        .collect::<String>()
        .chars()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            acc + i * c.to_digit(10).unwrap_or(0) as usize
        })
}
