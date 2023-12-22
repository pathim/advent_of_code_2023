use std::{collections::HashSet, ops::RangeInclusive};

fn abs_range(a: i64, b: i64) -> RangeInclusive<i64> {
    let min = a.min(b);
    let max = a.max(b);
    a..=b
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn below(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

struct Block {
    subblocks: Vec<Pos>,
}

impl Block {
    fn new(start: Pos, end: Pos) -> Self {
        let mut subblocks = Vec::new();
        for x in abs_range(start.x, end.x) {
            for y in abs_range(start.y, end.y) {
                for z in abs_range(start.z, end.z) {
                    subblocks.push(Pos { x, y, z });
                }
            }
        }
        subblocks.sort_by_key(|a| a.z);
        Self { subblocks }
    }
    fn can_drop(&self, filled_space: &HashSet<Pos>) -> bool {
        let z_min = self.subblocks[0].z;
        if z_min == 1 {
            return false;
        }
        for sb in self.subblocks.iter() {
            if sb.z != z_min {
                break;
            }
            if filled_space.contains(&sb.below()) {
                return false;
            }
        }
        true
    }
    fn drop(&mut self, filled_space: &mut HashSet<Pos>) -> bool {
        if !self.can_drop(filled_space) {
            return false;
        }

        for sb in self.subblocks.iter_mut() {
            filled_space.remove(sb);
            *sb = sb.below();
            filled_space.insert(*sb);
        }
        true
    }
    fn vanish(&self, filled_space: &mut HashSet<Pos>) {
        for sb in self.subblocks.iter() {
            filled_space.remove(sb);
        }
    }
    fn appear(&self, filled_space: &mut HashSet<Pos>) {
        for sb in self.subblocks.iter() {
            filled_space.insert(*sb);
        }
    }
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let re = regex::Regex::new("(\\d+),(\\d+),(\\d+)~(\\d+),(\\d+),(\\d+)").unwrap();
    let mut blocks = Vec::new();
    for l in input.lines() {
        let l = l.unwrap();
        let captures = re.captures(&l).unwrap();
        let block = Block::new(
            Pos {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
                z: captures[3].parse().unwrap(),
            },
            Pos {
                x: captures[4].parse().unwrap(),
                y: captures[5].parse().unwrap(),
                z: captures[6].parse().unwrap(),
            },
        );
        blocks.push(block);
    }
    let mut filled_space = HashSet::new();

    for b in &blocks {
        for p in b.subblocks.iter() {
            filled_space.insert(*p);
        }
    }
    let mut has_dropped = true;
    while has_dropped {
        has_dropped = false;
        for b in blocks.iter_mut() {
            has_dropped |= b.drop(&mut filled_space);
        }
    }
    let mut res1 = 0;
    'outer: for b in blocks.iter() {
        b.vanish(&mut filled_space);
        for b2 in blocks.iter() {
            if b2.can_drop(&filled_space) {
                b.appear(&mut filled_space);
                continue 'outer;
            }
        }
        b.appear(&mut filled_space);
        res1 += 1;
    }
    res1.into()
}
