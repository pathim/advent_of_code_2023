fn get_neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
}

#[derive(Debug, Clone)]
struct Edge {
    from: (usize, usize),
    to: (usize, usize),
    length: usize,
}

impl Edge {
    fn new(
        start: (usize, usize),
        step: (usize, usize),
        input: &[Vec<char>],
        nodes: &[(usize, usize)],
    ) -> Self {
        let mut path = vec![start];
        let mut cur_pos = step;
        while !nodes.contains(&cur_pos) && cur_pos.1 != input.len() - 1 {
            path.push(cur_pos);
            let n = get_neighbors(cur_pos.0, cur_pos.1);
            for (nx, ny) in n {
                if input[ny][nx] == '#' {
                    continue;
                }
                if path.contains(&(nx, ny)) {
                    continue;
                }
                cur_pos = (nx, ny);
            }
        }
        Edge {
            from: start,
            to: cur_pos,
            length: path.len(),
        }
    }
}

fn find_paths(
    start: (usize, usize),
    end: (usize, usize),
    edges: &[Edge],
    nodes: &[(usize, usize)],
) -> Vec<Vec<Edge>> {
    let mut res = vec![vec![]];
    if start == end {
        return res;
    }
    for e in edges.iter().filter(|e| e.from == start) {
        let mut paths = find_paths(e.to, end, edges, nodes);
        for mut p in paths {
            p.push(e.clone());
            res.push(p);
        }
    }
    res
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let input = input.to_2d_array();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '#' {
                continue;
            }
            if y == 0 {
                start = (x, y);
                continue;
            }
            if y == input.len() - 1 {
                end = (x, y);
                continue;
            }
            let n = get_neighbors(x, y);
            let num_walls = n
                .iter()
                .map(|(x, y)| input[*y][*x])
                .filter(|c| *c == '#')
                .count();
            if num_walls < 2 {
                nodes.push((x, y));
            }
        }
    }
    for (x, y) in nodes.iter() {
        let n = get_neighbors(*x, *y);
        for (nx, ny) in &n[..2] {
            if input[*ny][*nx] != '#' {
                edges.push(Edge::new((*x, *y), (*nx, *ny), &input, &nodes));
            }
        }
    }
    edges.push(Edge::new(start, (start.0, start.1 + 1), &input, &nodes));
    let paths = find_paths(start, end, &edges, &nodes);
    let res1: usize = paths
        .iter()
        .map(|p| p.iter().map(|e| e.length).sum())
        .max()
        .unwrap();
    res1.into()
}
