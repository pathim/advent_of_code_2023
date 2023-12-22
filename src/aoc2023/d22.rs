use std::collections::HashSet;

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

#[derive(Debug, Clone)]
struct Block {
    subblocks: Vec<Pos>,
    has_dropped: bool,
}

impl Block {
    fn new(start: Pos, end: Pos) -> Self {
        let mut subblocks = Vec::new();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    subblocks.push(Pos { x, y, z });
                }
            }
        }
        subblocks.sort_by_key(|a| a.z);
        Self {
            subblocks,
            has_dropped: false,
        }
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

fn drop_all(blocks: &mut [Block], filled_space: &mut HashSet<Pos>) -> usize {
    let mut has_dropped = true;
    while has_dropped {
        has_dropped = false;
        for b in blocks.iter_mut() {
            if b.drop(filled_space) {
                has_dropped = true;
                b.has_dropped = true;
            }
        }
    }
    let c = blocks.iter().filter(|x| x.has_dropped).count();
    for b in blocks.iter_mut() {
        b.has_dropped = false;
    }
    c
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
        b.appear(&mut filled_space);
    }
    drop_all(&mut blocks, &mut filled_space);

    let mut res1 = 0;
    let mut res2 = 0;
    for b in blocks.iter() {
        let mut f = filled_space.clone();
        let mut bs = blocks.clone();
        b.vanish(&mut f);
        let drop_count = drop_all(&mut bs, &mut f);
        if drop_count == 0 {
            res1 += 1;
        }
        res2 += drop_count;
    }
    (res1, res2).into()
}
