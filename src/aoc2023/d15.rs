fn hash(s: &str) -> u8 {
    let mut hash = 0u8;
    for c in s.bytes() {
        hash = hash.overflowing_add(c).0.overflowing_mul(17).0;
    }
    hash
}
struct Lens {
    label: String,
    length: u64,
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());
    let line = input.lines().next().unwrap().unwrap();
    let mut res1 = 0;
    for cmd in line.split(',') {
        res1 += hash(cmd) as u64;
        if let Some((label, length)) = cmd.split_once('=') {
            let length = length.parse().unwrap();
            let box_id = hash(label) as usize;
            if let Some(lens) = boxes[box_id].iter_mut().find(|x| x.label == label) {
                lens.length = length;
            } else {
                boxes[box_id].push(Lens {
                    label: label.to_owned(),
                    length,
                });
            }
        } else {
            let label = &cmd[0..cmd.len() - 1];
            let box_id = hash(label) as usize;
            if let Some(index) = boxes[box_id].iter().position(|x| x.label == label) {
                boxes[box_id].remove(index);
            }
        }
    }
    let res2: u64 = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (i as u64 + 1)
                * b.iter()
                    .enumerate()
                    .map(|(slot, l)| (slot as u64 + 1) * l.length)
                    .sum::<u64>()
        })
        .sum();
    (res1, res2).into()
}
