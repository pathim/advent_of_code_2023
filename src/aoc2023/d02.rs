pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (id, sets) = line.split_once(':').unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let id: u32 = id.parse().unwrap();
        let mut is_impossible = false;
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        for set in sets.split(';') {
            for per_color in set.split(',') {
                let (num, color) = per_color.trim().split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                is_impossible |= match color {
                    "red" => {
                        min_r = min_r.max(num);
                        num > 12
                    }
                    "green" => {
                        min_g = min_g.max(num);
                        num > 13
                    }
                    "blue" => {
                        min_b = min_b.max(num);
                        num > 14
                    }
                    _ => panic!("Invalid color: '{}'", color),
                };
            }
        }
        if !is_impossible {
            res1 += id;
        }
        res2 += min_b * min_g * min_r;
    }

    (res1, res2).into()
}
