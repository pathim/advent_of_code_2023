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
            Dir::Left => (self.0 - 1, self.1),
            Dir::Right => (self.0 + 1, self.1),
            Dir::Up => (self.0, self.1 - 1),
            Dir::Down => (self.0, self.1 + 1),
        };
        Pos(x, y)
    }
    fn get<T: Clone>(&self, tiles: &[Vec<T>]) -> T {
        tiles[self.1][self.0].clone()
    }
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let pieces = {
        let mut pieces = HashMap::new();
        pieces.insert('-', [Dir::Left, Dir::Right]);
        pieces.insert('|', [Dir::Up, Dir::Down]);
        pieces.insert('7', [Dir::Left, Dir::Down]);
        pieces.insert('J', [Dir::Left, Dir::Up]);
        pieces.insert('F', [Dir::Down, Dir::Right]);
        pieces.insert('L', [Dir::Up, Dir::Right]);
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

    let mut pos = start;
    let mut dir = Dir::Up;
    for d in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
        let new_pos = start.next(d);
        let pipe = new_pos.get(&pipes);
        if pieces.get(&pipe).unwrap().contains(&d.opposite()) {
            pos = new_pos;
            dir = d;
        }
    }

    let mut count = 1;
    while pos != start {
        let p = pieces.get(&pos.get(&pipes)).unwrap();
        if p[0] == dir.opposite() {
            dir = p[1];
        } else if p[1] == dir.opposite() {
            dir = p[0];
        } else {
            panic!("Invalid move")
        }
        pos = pos.next(dir);
        count += 1;
    }

    let res1 = count / 2;
    res1.into()
}
