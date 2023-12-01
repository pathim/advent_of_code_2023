use std::io::BufRead;

pub fn f(input: std::fs::File) -> crate::aoc_result::AocResult {
    let input = std::io::BufReader::new(input);
    let mut res1 = 0;
    let mut res2 = 0;
    let number_names = vec![
        "[0-9]", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers_re = number_names.join("|");
    let re_first = regex::Regex::new(&format!("({nre})", nre = numbers_re)).expect("Invalid regex");
    let re_last =
        regex::Regex::new(&format!(".*({nre})", nre = numbers_re)).expect("Invalid regex");
    for line in input.lines() {
        let line = line.unwrap();
        let mut first = None;
        let mut last = 0;
        for c in line.chars() {
            if let Some(val) = c.to_digit(10) {
                first.get_or_insert(val);
                last = val;
            }
        }
        let first = first.unwrap();
        res1 += 10 * first + last;
        let first = &re_first.captures(&line).unwrap()[1];
        let last = &re_last.captures(&line).unwrap()[1];
        let find = |val: &str| {
            number_names
                .iter()
                .position(|&x| x == val)
                .unwrap_or_else(|| val.chars().next().unwrap().to_digit(10).unwrap() as usize)
        };
        let first = find(first);
        let last = find(last);
        res2 += 10 * first + last;
    }
    (res1, res2).into()
}
