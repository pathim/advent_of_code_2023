fn calc_win_range(time: f64, dist: f64) -> u64 {
    let t = time;
    let d = dist;
    let sqrt_val = (t * t / 4.0 - d).sqrt();
    let min = (t / 2.0 - sqrt_val).ceil() as u64;
    let max = (t / 2.0 + sqrt_val).floor() as u64;
    max - min + 1
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut lines = input.lines();
    let times_line = lines.next().unwrap().unwrap();
    let times_str: Vec<_> = times_line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .collect();
    let times1 = times_str.iter().map(|x| x.parse::<f64>().unwrap());
    let times2: f64 = times_str.join("").parse().unwrap();
    let distances_line = lines.next().unwrap().unwrap();
    let distances_str: Vec<_> = distances_line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .collect();
    let distances1 = distances_str.iter().map(|x| x.parse::<f64>().unwrap());
    let distances2: f64 = distances_str.join("").parse().unwrap();
    let res1: u64 = times1
        .zip(distances1)
        .map(|(t, d)| calc_win_range(t, d))
        .product();
    let res2 = calc_win_range(times2, distances2);

    (res1, res2).into()
}
