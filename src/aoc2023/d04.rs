use std::{collections::HashSet, io::BufRead};

pub fn f(input: std::fs::File) -> crate::aoc_result::AocResult {
    let input = std::io::BufReader::new(input);
    let mut res1 = 0;
    let mut res2 = 0;
    let mut copies = std::collections::VecDeque::new();
    for line in input.lines() {
        let num_copies = copies.pop_front().unwrap_or(1);
        res2 += num_copies;
        let line = line.unwrap();
        let (_, sets) = line.split_once(':').unwrap();
        let (winning, mine) = sets.split_once('|').unwrap();
        let winning: HashSet<_> = winning.trim().split_ascii_whitespace().collect();
        let mine: HashSet<_> = mine.trim().split(' ').collect();
        let my_winning = winning.intersection(&mine);
        let num_winning = my_winning.count();
        for index in 0..num_winning {
            if let Some(value) = copies.get_mut(index) {
                *value += num_copies;
            } else {
                copies.push_back(1 + num_copies);
            }
        }
        let value = if num_winning == 0 {
            0
        } else {
            1 << (num_winning - 1)
        };
        res1 += value;
    }
    (res1, res2).into()
}
