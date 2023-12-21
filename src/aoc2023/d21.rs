use std::collections::HashSet;

fn get_neighbors(pos: (usize, usize), grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut res = HashSet::with_capacity(4);
    let x = pos.0 as isize;
    let y = pos.1 as isize;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x + dx;
        let ny = y + dy;
        if nx < 0 || ny < 0 {
            continue;
        }
        if let Some(c) = grid.get(ny as usize).and_then(|v| v.get(nx as usize)) {
            if *c != '#' {
                res.insert((nx as usize, ny as usize));
            }
        }
    }
    res
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let (grid, start) = input.to_2d_array_finding(|c| c == 'S');
    if start.len() != 1 {
        panic!("Not exactly one start")
    }

    let mut reachable = HashSet::from_iter(start);
    for i in 0..64 {
        println!("{}, {}", i, reachable.len());

        let mut new_reachable = HashSet::new();
        for pos in reachable {
            let neighbors = get_neighbors(pos, &grid);
            new_reachable = new_reachable.union(&neighbors).copied().collect();
        }
        reachable = new_reachable;
    }

    let res1 = reachable.len();

    res1.into()
}
