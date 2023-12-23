use std::collections::HashMap;

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
        while !nodes.contains(&cur_pos) && cur_pos.1 != input.len() - 1 && cur_pos.1 != 0 {
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
    mut visited: Vec<(usize, usize)>,
) -> usize {
    if start == end {
        return 0;
    }
    visited.push(start);
    let mut longest = 0;
    for e in edges.iter().filter(|e| e.from == start) {
        if visited.contains(&e.to) {
            continue;
        }
        let paths = find_paths(e.to, end, edges, nodes, visited.clone());
        longest = longest.max(paths + e.length)
    }
    longest
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let input = input.to_2d_array();
    let mut nodes = Vec::new();
    let mut edges1 = Vec::new();
    let mut edges2 = Vec::new();
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
        for (idx, (nx, ny)) in n.iter().enumerate() {
            if input[*ny][*nx] != '#' {
                let edge = Edge::new((*x, *y), (*nx, *ny), &input, &nodes);
                if idx < 2 {
                    edges1.push(edge.clone());
                }
                edges2.push(edge);
            }
        }
    }
    let start_edge = Edge::new(start, (start.0, start.1 + 1), &input, &nodes);
    edges1.push(start_edge.clone());
    edges2.push(start_edge);
    let res1 = find_paths(start, end, &edges1, &nodes, Vec::new());
    let res2 = find_paths(start, end, &edges2, &nodes, Vec::new());
    (res1, res2).into()
}
