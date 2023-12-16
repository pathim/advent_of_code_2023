use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Beam {
    fn step(self, width: usize, height: usize) -> Option<Self> {
        let mut x = self.x as isize;
        let mut y = self.y as isize;
        match self.dir {
            Dir::Up => y -= 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
        };
        if !(0..width as isize).contains(&x) || !(0..height as isize).contains(&y) {
            return None;
        }
        Some(Self {
            x: x as usize,
            y: y as usize,
            dir: self.dir,
        })
    }
    fn with_dir(&self, dir: Dir) -> Self {
        Self {
            x: self.x,
            y: self.y,
            dir,
        }
    }
}

fn count_energized(field: &[Vec<char>], start: Beam) -> usize {
    let width = field[0].len();
    let height = field.len();
    let mut visited = HashMap::new();
    let mut beams = vec![start];
    while let Some(beam) = beams.pop() {
        if let Some(v) = visited.get_mut(&(beam.x, beam.y)) {
            if *v & (beam.dir as u8) != 0 {
                continue;
            } else {
                *v |= beam.dir as u8;
            }
        } else {
            visited.insert((beam.x, beam.y), beam.dir as u8);
        }
        let new_beams = match field[beam.y][beam.x] {
            '.' => [beam.step(width, height), None],
            '-' => match beam.dir {
                Dir::Down | Dir::Up => [
                    Some(beam.with_dir(Dir::Left)),
                    Some(beam.with_dir(Dir::Right)),
                ],
                _ => [beam.step(width, height), None],
            },
            '|' => match beam.dir {
                Dir::Left | Dir::Right => [
                    beam.with_dir(Dir::Up).step(width, height),
                    beam.with_dir(Dir::Down).step(width, height),
                ],
                _ => [beam.step(width, height), None],
            },
            '/' => match beam.dir {
                Dir::Left => [beam.with_dir(Dir::Down).step(width, height), None],
                Dir::Right => [beam.with_dir(Dir::Up).step(width, height), None],
                Dir::Up => [beam.with_dir(Dir::Right).step(width, height), None],
                Dir::Down => [beam.with_dir(Dir::Left).step(width, height), None],
            },
            '\\' => match beam.dir {
                Dir::Left => [beam.with_dir(Dir::Up).step(width, height), None],
                Dir::Right => [beam.with_dir(Dir::Down).step(width, height), None],
                Dir::Up => [beam.with_dir(Dir::Left).step(width, height), None],
                Dir::Down => [beam.with_dir(Dir::Right).step(width, height), None],
            },
            c => panic!("Invalid char {}", c),
        };
        beams.extend(new_beams.iter().filter_map(|x| *x));
    }
    visited.len()
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    let field = input.to_2d_array();
    let width = field[0].len();
    let height = field.len();

    let mut starts = Vec::new();
    for x in 0..width {
        starts.push(Beam {
            x,
            y: 0,
            dir: Dir::Down,
        });
        starts.push(Beam {
            x,
            y: height - 1,
            dir: Dir::Up,
        });
    }
    for y in 0..height {
        starts.push(Beam {
            x: 0,
            y,
            dir: Dir::Right,
        });
        starts.push(Beam {
            x: width - 1,
            y,
            dir: Dir::Left,
        });
    }

    let res1 = count_energized(
        &field,
        Beam {
            x: 0,
            y: 0,
            dir: Dir::Right,
        },
    );
    let res2 = starts
        .iter()
        .map(|x| count_energized(&field, *x))
        .max()
        .unwrap();
    (res1, res2).into()
}
