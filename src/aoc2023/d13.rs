fn count_diff(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).filter(|(a, b)| **a != **b).count()
}
fn find_v_sym(v: &[Vec<char>]) -> Option<usize> {
    'outer: for a in 1..v.len() {
        for n in 0..a {
            if a + n >= v.len() {
                return Some(a);
            }
            if v[a - n - 1] != v[a + n] {
                continue 'outer;
            }
        }
        return Some(a);
    }
    None
}
fn find_v_sym_smudge(v: &[Vec<char>]) -> Option<usize> {
    'outer: for a in 1..v.len() {
        let mut found_smudge = false;
        for n in 0..a {
            if a + n >= v.len() {
                if found_smudge {
                    return Some(a);
                } else {
                    continue 'outer;
                };
            }
            let diff = count_diff(&v[a - n - 1], &v[a + n]);
            if diff == 0 {
                continue;
            }
            if diff == 1 {
                if found_smudge {
                    continue 'outer;
                }
                found_smudge = true;
            } else {
                continue 'outer;
            }
        }
        if found_smudge {
            return Some(a);
        };
    }
    None
}
fn get_col(v: &[Vec<char>], col: usize) -> Vec<char> {
    v.iter().map(|x| x[col]).collect()
}

fn find_h_sym(v: &[Vec<char>]) -> Option<usize> {
    'outer: for a in 1..v[0].len() {
        for n in 0..a {
            if a + n >= v[0].len() {
                return Some(a);
            }
            if get_col(v, a - n - 1) != get_col(v, a + n) {
                continue 'outer;
            }
        }
        return Some(a);
    }
    None
}
fn find_h_sym_smudge(v: &[Vec<char>]) -> Option<usize> {
    'outer: for a in 1..v[0].len() {
        let mut found_smudge = false;
        for n in 0..a {
            if a + n >= v[0].len() {
                if found_smudge {
                    return Some(a);
                } else {
                    continue 'outer;
                };
            }
            let diff = count_diff(&get_col(v, a - n - 1), &get_col(v, a + n));
            if diff == 0 {
                continue;
            }
            if diff == 1 {
                if found_smudge {
                    continue 'outer;
                }
                found_smudge = true;
            } else {
                continue 'outer;
            }
        }
        if found_smudge {
            return Some(a);
        };
    }
    None
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut maps = Vec::new();
    let mut cur_map = Vec::new();
    for l in input.to_2d_array() {
        if l.is_empty() {
            maps.push(cur_map);
            cur_map = Vec::new();
        } else {
            cur_map.push(l);
        }
    }
    maps.push(cur_map);

    let res1: usize = maps
        .iter()
        .map(|m| find_h_sym(m).or_else(|| find_v_sym(m).map(|x| x * 100)))
        .map(|x| x.unwrap())
        .sum();

    let res2: usize = maps
        .iter()
        .map(|m| find_h_sym_smudge(m).or_else(|| find_v_sym_smudge(m).map(|x| x * 100)))
        .map(|x| x.unwrap())
        .sum();

    (res1, res2).into()
}
