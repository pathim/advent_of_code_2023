use std::io::BufRead;

pub fn f(input: std::fs::File) -> crate::aoc_result::AocResult {
    let input = std::io::BufReader::new(input);
    let mut res = 0;
    for lines in input.lines() {
        let mut first = None;
        let mut last = 0;
        for c in lines.unwrap().chars() {
            if let Some(val) = c.to_digit(10) {
                first.get_or_insert(val);
                last = val;
            }
        }
        let first = first.unwrap();
        res += 10 * first + last;
    }
    res.into()
}
