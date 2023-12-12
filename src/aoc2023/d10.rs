use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(usize, usize);
impl Pos {
    fn next(&self, dir: Dir) -> Pos {
        let (x, y) = match dir {
            Dir::Left => (self.0.saturating_sub(1), self.1),
            Dir::Right => (self.0 + 1, self.1),
            Dir::Up => (self.0, self.1.saturating_sub(1)),
            Dir::Down => (self.0, self.1 + 1),
        };
        Pos(x, y)
    }
    fn get<T: Clone>(&self, tiles: &[Vec<T>]) -> T {
        tiles[self.1][self.0].clone()
    }
    fn get_mut<'a, T>(&self, tiles: &'a mut [Vec<T>]) -> Option<&'a mut T> {
        tiles.get_mut(self.1).and_then(|x| x.get_mut(self.0))
    }
    fn get_neighbors(&self) -> Vec<Self> {
        let mut res = Vec::new();
        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            res.push(self.next(d));
        }
        res
    }
}

fn get_zoomed_tile(tilemap: &Vec<Vec<char>>, x: usize, y: usize) -> [char; 9] {
    let mut res = [' '; 9];
    let mut n = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            let x = (x * 3 + 1).saturating_add_signed(dx);
            let y = (y * 3 + 1).saturating_add_signed(dy);
            res[n] = tilemap[y][x];
            n += 1;
        }
    }
    res
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let pieces = {
        let mut pieces = HashMap::new();
        pieces.insert('-', ([Dir::Left, Dir::Right], b"...###..."));
        pieces.insert('|', ([Dir::Up, Dir::Down], b".#..#..#."));
        pieces.insert('7', ([Dir::Left, Dir::Down], b"...##..#."));
        pieces.insert('J', ([Dir::Left, Dir::Up], b".#.##...."));
        pieces.insert('F', ([Dir::Down, Dir::Right], b"....##.#."));
        pieces.insert('L', ([Dir::Up, Dir::Right], b".#..##..."));
        pieces
    };
    let pipes = input.to_2d_array();
    let start = pipes
        .iter()
        .enumerate()
        .filter_map(|l| {
            l.1.iter()
                .enumerate()
                .find(|&c| *c.1 == 'S')
                .map(|(x, _)| (x, l.0))
        })
        .next()
        .unwrap();
    let start = Pos(start.0, start.1);

    let mut zoomed_pipes = Vec::new();
    for line in &pipes {
        for zline in 0..3 {
            let mut zoomed_line = Vec::new();
            for c in line {
                for pc in zline * 3..(zline + 1) * 3 {
                    zoomed_line.push(if let Some(cc) = pieces.get(&c) {
                        cc.1[pc].into()
                    } else {
                        c.clone()
                    });
                }
            }
            zoomed_pipes.push(zoomed_line);
        }
    }

    let mut pos = start;
    let mut dir = Dir::Up;
    for d in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
        let new_pos = start.next(d);
        let pipe = new_pos.get(&pipes);
        if pieces.get(&pipe).unwrap().0.contains(&d.opposite()) {
            pos = new_pos;
            dir = d;
        }
    }

    let mut count = 1;
    while pos != start {
        for zx in -1..2 {
            for zy in -1..2 {
                let z = &mut zoomed_pipes[((pos.1 as i32) * 3 + 1 + zy) as usize]
                    [((pos.0 as i32) * 3 + 1 + zx) as usize];
                if *z != '.' {
                    *z = 'X';
                }
            }
        }
        let p = pieces.get(&pos.get(&pipes)).unwrap();
        if p.0[0] == dir.opposite() {
            dir = p.0[1];
        } else if p.0[1] == dir.opposite() {
            dir = p.0[0];
        } else {
            panic!("Invalid move")
        }
        pos = pos.next(dir);
        count += 1;
    }

    let mut filled = Vec::new();
    for (y, l) in zoomed_pipes.iter_mut().enumerate() {
        l[0] = 'O';
        filled.push(Pos(0, y));
        let x = l.len() - 1;
        l[x] = 'O';
        filled.push(Pos(x, y));
    }
    while let Some(p) = filled.pop() {
        for n in p.get_neighbors() {
            if let Some(v) = n.get_mut(&mut zoomed_pipes) {
                if *v == '.' {
                    *v = 'O';
                    filled.push(n);
                }
            }
        }
    }

    let mut res2 = 0;
    for (y, l) in pipes.iter().enumerate() {
        for x in 0..l.len() {
            let cs = get_zoomed_tile(&zoomed_pipes, x, y);
            if !cs.contains(&'O') && !cs.contains(&'X') && !cs.contains(&'S') {
                res2 += 1;
            }
        }
    }

    let res1 = count / 2;
    (res1, res2).into()
}
