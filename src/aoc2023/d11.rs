pub fn f(input: crate::AocInput) -> crate::AocResult {
    let universe = input.to_2d_array();
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|&(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    let width = universe[0].len();
    let height = universe.len();
    let mut row_empty = [true].repeat(height);
    let mut col_empty = [true].repeat(width);
    for (x, y) in &galaxies {
        row_empty[*y] = false;
        col_empty[*x] = false;
    }
    let num_galaxies = galaxies.len();
    let mut res1 = 0;
    let mut res2: u64 = 0;
    for i in 0..num_galaxies - 1 {
        for j in i + 1..num_galaxies {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            for row in y1.min(y2)..y1.max(y2) {
                res1 += if row_empty[row] { 2 } else { 1 };
                res2 += if row_empty[row] { 1_000_000 } else { 1 };
            }
            for col in x1.min(x2)..x1.max(x2) {
                res1 += if col_empty[col] { 2 } else { 1 };
                res2 += if col_empty[col] { 1_000_000 } else { 1 };
            }
        }
    }
    (res1, res2).into()
}
