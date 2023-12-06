use std::{fmt::Debug, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct MapRange {
    src: core::ops::Range<u64>,
    dst: core::ops::Range<u64>,
}

impl FromStr for MapRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s.split_ascii_whitespace().map(|v| v.parse()).collect();
        if values.len() != 3 {
            let _: u64 = "Wrong Length".parse()?;
        }
        let dst_start = values[0].clone()?;
        let src_start = values[1].clone()?;
        let length = values[2].clone()?;
        let src = src_start..(src_start + length);
        let dst = dst_start..(src_start + length);
        Ok(Self { src, dst })
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new<E: Debug, T: Iterator<Item = Result<String, E>>>(lines: &mut T) -> Self {
        let mut ranges = Vec::new();
        while let Some(l) = lines.next() {
            let l = l.unwrap();
            if let Ok(range) = l.parse() {
                ranges.push(range);
                break;
            }
        }
        while let Some(l) = lines.next() {
            let l = l.unwrap();
            if let Ok(range) = l.parse() {
                ranges.push(range);
            } else {
                break;
            }
        }
        Self { ranges }
    }

    fn do_map(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.src.contains(&value) {
                return range.dst.start + (value - range.src.start);
            }
        }
        value
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

fn chain_map(maps: &Vec<Map>, mut value: u64) -> u64 {
    for m in maps {
        value = m.do_map(value);
    }
    value
}
pub fn f(input: crate::AocInput) -> crate::AocResult {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut maps = Vec::new();
    loop {
        let map = Map::new(&mut lines);
        if map.is_empty() {
            break;
        }
        maps.push(map);
    }
    let res1 = seeds.iter().map(|&s| chain_map(&maps, s)).min().unwrap();
    res1.into()
}
