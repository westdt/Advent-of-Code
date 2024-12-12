pub fn solve(input: &str) -> usize {
    input.lines().fold(0, |acc, f| {
        let (result, data) = f.split_once(":").unwrap();
        let (result, data) = (
            result.parse::<usize>().unwrap(),
            data.trim()
                .split_whitespace()
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );

        fn recurse(data: Vec<usize>) -> Vec<usize> {
            if data.len() <= 2 {
                vec![
                    data[0] + data[1],
                    data[0] * data[1],
                    format!("{}{}", data[0], data[1]).parse::<usize>().unwrap(),
                ]
            } else {
                let a = data[0] + data[1];
                let b = data[0] * data[1];
                let c = format!("{}{}", data[0], data[1]).parse::<usize>().unwrap();
                let mut dataa = data[2..data.len()].to_vec();
                let mut datab = data[2..data.len()].to_vec();
                let mut datac = data[2..data.len()].to_vec();
                dataa.insert(0, a);
                datab.insert(0, b);
                datac.insert(0, c);
                let mut result = recurse(dataa);
                result.extend(recurse(datab));
                result.extend(recurse(datac));

                result
            }
        }

        let test = recurse(data);

        if test.contains(&result) {
            acc + result
        } else {
            acc
        }
    })
}
