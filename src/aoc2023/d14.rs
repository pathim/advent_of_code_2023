pub fn f(input: crate::AocInput) -> crate::AocResult {
    let (mut field, boulders) = input.to_2d_array_finding(|c| c == 'O');
    let mut res1 = 0;
    for (x, y) in boulders {
        let mut new_y = y;
        while new_y > 0 && field[new_y - 1][x] == '.' {
            new_y -= 1;
        }
        field[y][x] = '.';
        field[new_y][x] = 'O';
        res1 += field.len() - new_y;
    }
    res1.into()
}
