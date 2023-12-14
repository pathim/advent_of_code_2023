fn gen(broken: Vec<u32>, space: Vec<u32>) -> Vec<char> {
    let mut res = Vec::new();
    res.append(&mut vec!['.'; space[0] as usize]);
    for i in 0..broken.len() {
        res.append(&mut vec!['#'; broken[i] as usize]);
        res.append(&mut vec!['.'; space[i + 1] as usize]);
    }
    res
}

fn check(pattern: Vec<char>, test: Vec<char>, n: usize) -> bool {
    if pattern.len() != test.len() {
        panic!("Invalid length");
    }
    for (p, c) in pattern[..n].iter().zip(test.iter()) {
        if *p != '?' && *p != *c {
            return false;
        }
    }
    true
}

pub fn f(input: crate::AocInput) -> crate::AocResult {
    for l in input.lines() {
        let l = l.unwrap();
        let (pattern, numbers) = l.split_once(' ').unwrap();
        let numbers = numbers
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        let num_unk = pattern.chars().filter(|x| *x == '?').count();
        let total = pattern.len() as u32;
        let broken: u32 = numbers.iter().sum();
        let total_spacing = total - broken;
        let mut spacing = vec![1; numbers.len() + 1];
        spacing[0] = 0;
        spacing[numbers.len()] = 0;
        let to_distribute = total_spacing - spacing.iter().sum::<u32>();
        dbg!((spacing.len().pow(to_distribute)));
    }
    dbg!(2u32.pow(19));
    todo!()
}
