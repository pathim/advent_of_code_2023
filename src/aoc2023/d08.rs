use std::collections::HashMap;

use regex::Regex;

fn find_dist_to_target(
    graph: &HashMap<String, (String, String)>,
    dirs: &str,
    dirs_offset: usize,
    start: &str,
) -> (String, usize) {
    let mut pos = start.to_owned();
    for i in dirs_offset.. {
        let i_mod = i % dirs.len();
        let dir = dirs.as_bytes()[i_mod];
        if i > dirs_offset && pos.as_bytes()[2] == b'Z' {
            return (pos.to_owned(), i - dirs_offset);
        }
        pos = if dir == b'L' {
            &graph[&pos].0
        } else {
            &graph[&pos].1
        }
        .clone();
    }
    unreachable!()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let re = Regex::new("([A-Z]*) = \\(([A-Z]*), ([A-Z]*)\\)").unwrap();
    let mut graph = HashMap::new();
    let mut lines = input.lines();
    let mut starts = Vec::new();
    let dirs = lines.next().unwrap().unwrap();
    lines.next();
    for l in lines {
        let l = l.unwrap();
        let c = re.captures(&l).unwrap();
        graph.insert(c[1].to_owned(), (c[2].to_owned(), c[3].to_owned()));
        if c[1].as_bytes()[2] == b'A' {
            starts.push(c[1].to_owned());
        }
    }

    let (_, res1) = find_dist_to_target(&graph, &dirs, 0, "AAA");
    let mut res2 = 1;
    for start in starts.iter() {
        let (target, offset) = find_dist_to_target(&graph, &dirs, 0, start);
        let (_, cycle) = find_dist_to_target(&graph, &dirs, offset, &target);
        res2 = res2 * cycle / gcd(res2, cycle);
    }
    (res1, res2).into()
}
