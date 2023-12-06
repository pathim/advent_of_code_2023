fn get_signed<T>(v: &Vec<T>, index: isize) -> Option<&T> {
    if index < 0 {
        return None;
    }
    v.get(index as usize)
}

fn find_number_start(blueprint: &Vec<Vec<char>>, x: usize, y: usize) -> Option<usize> {
    let line = blueprint.get(y)?;
    let mut start = None;
    let mut pos: isize = x as isize;
    while let Some(digit) = get_signed(line, pos) {
        if digit.is_ascii_digit() {
            start = Some(pos as usize);
        } else {
            return start;
        }
        pos -= 1;
    }
    start
}

fn get_number(
    blueprint: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    numbers_used: &mut std::collections::HashSet<(usize, usize)>,
) -> Option<u32> {
    let start = find_number_start(blueprint, x, y)?;
    if numbers_used.contains(&(start, y)) {
        return None;
    }
    numbers_used.insert((start, y));
    let line = blueprint.get(y).unwrap();
    let mut res = 0;
    let mut pos = start;
    while let Some(digit) = line.get(pos) {
        pos += 1;
        if digit.is_ascii_digit() {
            res = 10 * res + digit.to_digit(10).unwrap();
        } else {
            break;
        }
    }
    Some(res)
}

pub fn f(input: crate::AocInput) -> crate::aoc_result::AocResult {
    let blueprint = input.to_2d_array();
    let mut numbers_used = std::collections::HashSet::new();
    let mut res1 = 0;
    let mut res2 = 0;

    for (y, line) in blueprint.iter().enumerate() {
        for (x, digit) in line.iter().enumerate() {
            if digit.is_ascii_digit() || digit == &'.' {
                continue;
            }
            let mut adjacent_numbers = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let number = get_number(
                        &blueprint,
                        x.saturating_add_signed(dx),
                        y.saturating_add_signed(dy),
                        &mut numbers_used,
                    );
                    if let Some(number) = number {
                        adjacent_numbers.push(number);
                        res1 += number;
                    }
                }
            }
            if adjacent_numbers.len() == 2 && digit == &'*' {
                res2 += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }

    (res1, res2).into()
}
