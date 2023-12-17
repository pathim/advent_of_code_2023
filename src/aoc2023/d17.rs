use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path {
    cost: u32,
    pos: (usize, usize),
    dir: (isize, isize),
    same_dir_count: u8,
}

impl Path {
    fn do_move(&self, dir: (isize, isize), field: &[Vec<u32>]) -> Option<Path> {
        let new_pos = (self.pos.0 as isize + dir.0, self.pos.1 as isize + dir.1);
        if !(0..field[0].len() as isize).contains(&new_pos.0)
            || !(0..field.len() as isize).contains(&new_pos.1)
        {
            return None;
        }
        let pos = (new_pos.0 as usize, new_pos.1 as usize);
        let same_dir_count = if self.dir == dir {
            if self.same_dir_count == 3 {
                return None;
            }
            self.same_dir_count + 1
        } else {
            1
        };
        let cost = self.cost + field[pos.1][pos.0];
        Some(Path {
            cost,
            pos,
            dir,
            same_dir_count,
        })
    }
    fn do_move2(&self, dir: (isize, isize), field: &[Vec<u32>]) -> Option<Path> {
        let new_pos = (self.pos.0 as isize + dir.0, self.pos.1 as isize + dir.1);
        if !(0..field[0].len() as isize).contains(&new_pos.0)
            || !(0..field.len() as isize).contains(&new_pos.1)
        {
            return None;
        }
        let pos = (new_pos.0 as usize, new_pos.1 as usize);
        if self.same_dir_count < 4 && self.dir != dir {
            return None;
        }
        let same_dir_count = if self.dir == dir {
            if self.same_dir_count >= 10 {
                return None;
            }
            self.same_dir_count + 1
        } else {
            1
        };
        let cost = self.cost + field[pos.1][pos.0];
        Some(Path {
            cost,
            pos,
            dir,
            same_dir_count,
        })
    }
    fn get_next(&self, field: &[Vec<u32>]) -> Vec<Path> {
        let mut res = Vec::new();
        res.push(self.do_move(self.dir, field));
        if self.dir.0 != 0 {
            res.push(self.do_move((0, 1), field));
            res.push(self.do_move((0, -1), field));
        } else {
            res.push(self.do_move((1, 0), field));
            res.push(self.do_move((-1, 0), field));
        }
        res.into_iter().flatten().collect()
    }
    fn get_next2(&self, field: &[Vec<u32>]) -> Vec<Path> {
        let mut res = Vec::new();
        res.push(self.do_move2(self.dir, field));
        if self.dir.0 != 0 {
            res.push(self.do_move2((0, 1), field));
            res.push(self.do_move2((0, -1), field));
        } else {
            res.push(self.do_move2((1, 0), field));
            res.push(self.do_move2((-1, 0), field));
        }
        res.into_iter().flatten().collect()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let costs = input.to_2d_array_mapped(|c| c.to_digit(10).unwrap());
    let mut paths = BinaryHeap::new();
    let mut visited: HashMap<((usize, usize), (isize, isize), u8), u32> = HashMap::new();
    paths.push(Path {
        pos: (0, 0),
        cost: 0,
        dir: (1, 0),
        same_dir_count: 0,
    });
    paths.push(Path {
        pos: (0, 0),
        cost: 0,
        dir: (0, 1),
        same_dir_count: 0,
    });
    let res1 = loop {
        let path = paths.pop().unwrap();
        if path.pos.0 == costs[0].len() - 1 && path.pos.1 == costs.len() - 1 {
            break path.cost;
        }
        for p in path.get_next(&costs) {
            if let Some(x) = visited.get_mut(&(p.pos, p.dir, p.same_dir_count)) {
                if *x <= p.cost {
                    continue;
                }
                *x = p.cost;
                paths.push(p);
            } else {
                visited.insert((p.pos, p.dir, p.same_dir_count), p.cost);
                paths.push(p);
            }
        }
    };
    visited.clear();
    paths.clear();
    paths.push(Path {
        pos: (0, 0),
        cost: 0,
        dir: (1, 0),
        same_dir_count: 0,
    });
    paths.push(Path {
        pos: (0, 0),
        cost: 0,
        dir: (0, 1),
        same_dir_count: 0,
    });
    let res2 = loop {
        let path = paths.pop().unwrap();
        if path.pos.0 == costs[0].len() - 1
            && path.pos.1 == costs.len() - 1
            && path.same_dir_count >= 4
        {
            break path.cost;
        }
        for p in path.get_next2(&costs) {
            if let Some(x) = visited.get_mut(&(p.pos, p.dir, p.same_dir_count)) {
                if *x <= p.cost {
                    continue;
                }
                *x = p.cost;
                paths.push(p);
            } else {
                visited.insert((p.pos, p.dir, p.same_dir_count), p.cost);
                paths.push(p);
            }
        }
    };
    (res1, res2).into()
}
