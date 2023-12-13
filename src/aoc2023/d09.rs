fn extrapolate(data: &[i64]) -> i64 {
    return if data.iter().all(|&x| x == 0) {
        0
    } else {
        data[data.len() - 1]
            + extrapolate(&data[..].windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>())
    };
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for mut x in input.lines().map(|x| x.unwrap()).map(|x| {
        x.split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
    }) {
        res1 += extrapolate(&x);
        x.reverse();
        res2 += extrapolate(&x);
    }

    (res1, res2).into()
}
