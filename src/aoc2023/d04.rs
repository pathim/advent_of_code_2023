use std::{collections::HashSet, io::BufRead};

pub fn f(input: std::fs::File) -> crate::aoc_result::AocResult {
    let input = std::io::BufReader::new(input);
    let mut res1 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (_, sets) = line.split_once(':').unwrap();
        let (winning, mine) = sets.split_once('|').unwrap();
        let winning: HashSet<_> = winning.trim().split_ascii_whitespace().collect();
        let mine: HashSet<_> = mine.trim().split(' ').collect();
        dbg!(&winning);
        dbg!(&mine);
        let my_winning = winning.intersection(&mine);
        let num_winning = my_winning.count();
        let value = if num_winning == 0 {
            0
        } else {
            1 << (num_winning - 1)
        };
        res1 += value;
    }
    (res1).into()
}
