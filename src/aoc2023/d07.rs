use crate::AocResult;

fn card_to_value(c: char) -> u64 {
    match c {
        '2'..='9' => c as u64 - '0' as u64,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid character {}", c),
    }
}

fn hand_to_value(cards: &[u64; 5]) -> u64 {
    let mut value = 0;
    let mut card_counts = [0; 15];
    for v in cards {
        value = value * 16 + v;
        card_counts[*v as usize] += 1;
    }

    let pat_val = match card_counts.iter().max().unwrap() {
        5 => 6,
        4 => 5,
        3 => {
            if card_counts.contains(&2) {
                4
            } else {
                3
            }
        }
        2 => {
            if card_counts.iter().filter(|&&x| x == 2).count() == 2 {
                2
            } else {
                1
            }
        }
        _ => 0,
    };
    value += 16_u64.pow(6) * pat_val;
    value
}

pub fn f(input: crate::AocInput) -> AocResult {
    let mut plays = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        let (hand_c, bid) = line.split_once(' ').unwrap();
        let mut hand = [0; 5];
        for (i, v) in hand_c.chars().map(card_to_value).enumerate() {
            hand[i] = v;
        }
        let bid: u32 = bid.parse().unwrap();
        plays.push((hand_to_value(&hand), bid, hand_c.to_owned(), hand.clone()));
    }
    plays.sort_by_key(|x| x.0);
    let res1: u64 = plays
        .iter()
        .enumerate()
        .map(|(i, &(_, b, _, _))| (i as u64 + 1) * b as u64)
        .sum();

    res1.into()
}
