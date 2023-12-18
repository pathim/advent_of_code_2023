use std::collections::HashSet;

fn find_inner(trench: &HashSet<(i32, i32)>) -> (i32, i32) {
    let max_x = trench.iter().max_by_key(|(x, _)| x).unwrap().0.clone();
    trench
        .iter()
        .filter(|(x, _)| *x == max_x)
        .map(|(x, y)| (*x - 1, *y))
        .filter(|a| !trench.contains(a))
        .next()
        .unwrap()
}

fn floodfill(trench: &mut HashSet<(i32, i32)>, start: (i32, i32)) {
    let mut to_fill = Vec::new();
    to_fill.push(start);
    while let Some(p) = to_fill.pop() {
        if trench.contains(&p) {
            continue;
        }
        trench.insert(p);
        for dx in [-1, 1] {
            to_fill.push((p.0 + dx, p.1));
        }
        for dy in [-1, 1] {
            to_fill.push((p.0, p.1 + dy));
        }
    }
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut trench = HashSet::new();
    let mut pos = (0, 0);
    trench.insert(pos);
    for l in input.lines() {
        let l = l.unwrap();
        let mut parts = l.split_ascii_whitespace();
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap();
        let color = parts.next().unwrap();
        let delta = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            x => panic!("invalid direction: {}", x),
        };
        for _ in 0..dist.parse().unwrap() {
            pos.0 += delta.0;
            pos.1 += delta.1;
            trench.insert(pos);
        }
    }
    let inner = find_inner(&trench);
    floodfill(&mut trench, inner);
    let res1 = trench.len();
    res1.into()
}
