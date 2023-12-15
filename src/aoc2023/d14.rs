fn north(field: &mut [Vec<char>], boulders: &mut [(usize, usize)]) {
    boulders.sort_by_key(|&(x, y)| y * 1000 + x);
    for (x, y) in boulders.iter_mut() {
        let mut new_y = *y;
        while new_y > 0 && field[new_y - 1][*x] == '.' {
            new_y -= 1;
        }
        field[*y][*x] = '.';
        field[new_y][*x] = 'O';
        *y = new_y;
    }
}

fn south(field: &mut [Vec<char>], boulders: &mut [(usize, usize)]) {
    boulders.sort_by_key(|&(x, y)| (field.len() - y) * 1000 + x);
    for (x, y) in boulders.iter_mut() {
        let mut new_y = *y;
        while new_y < field.len() - 1 && field[new_y + 1][*x] == '.' {
            new_y += 1;
        }
        field[*y][*x] = '.';
        field[new_y][*x] = 'O';
        *y = new_y;
    }
}
fn east(field: &mut [Vec<char>], boulders: &mut [(usize, usize)]) {
    boulders.sort_by_key(|&(x, y)| (field[0].len() - x) * 1000 + y);
    for (x, y) in boulders.iter_mut() {
        let mut new_x = *x;
        while new_x < field[0].len() - 1 && field[*y][new_x + 1] == '.' {
            new_x += 1;
        }
        field[*y][*x] = '.';
        field[*y][new_x] = 'O';
        *x = new_x;
    }
}
fn west(field: &mut [Vec<char>], boulders: &mut [(usize, usize)]) {
    boulders.sort_by_key(|&(x, y)| x * 1000 + y);
    for (x, y) in boulders.iter_mut() {
        let mut new_x = *x;
        while new_x > 0 && field[*y][new_x - 1] == '.' {
            new_x -= 1;
        }
        field[*y][*x] = '.';
        field[*y][new_x] = 'O';
        *x = new_x;
    }
}

fn cycle(field: &mut [Vec<char>], boulders: &mut [(usize, usize)]) {
    north(field, boulders);
    west(field, boulders);
    south(field, boulders);
    east(field, boulders);
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let (mut field, mut boulders) = input.to_2d_array_finding(|c| c == 'O');
    north(&mut field, &mut boulders);
    let res1: usize = boulders.iter().map(|&(_, y)| field.len() - y).sum();

    let mut n = 0usize;
    let mut last_boulders = Vec::new();
    let period = loop {
        last_boulders.push(boulders.clone());
        cycle(&mut field, &mut boulders);
        n += 1;
        if let Some((p, _)) = last_boulders
            .iter()
            .rev()
            .enumerate()
            .find(|(_, b)| **b == boulders)
        {
            break p + 1;
        }
    };

    let to_go = (1000000000 - n) % period;
    for _ in 0..to_go {
        cycle(&mut field, &mut boulders);
    }
    let res2: usize = boulders.iter().map(|&(_, y)| field.len() - y).sum();

    (res1, res2).into()
}
