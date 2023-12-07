use std::{fmt::Debug, num::ParseIntError, ops::Range, str::FromStr};

trait RangeOps: Sized {
    fn intersect(&self, other: &Self) -> (Self, Self, Self);
}

impl<T: Ord + Copy> RangeOps for Range<T> {
    fn intersect(&self, other: &Self) -> (Self, Self, Self) {
        let mut within_start = self.start;
        let mut within_end = self.end;
        let before = if other.start >= self.start {
            within_start = other.start;
            self.start..self.start
        } else {
            if other.end <= self.start {
                return (other.clone(), self.start..self.start, self.end..self.end);
            }
            other.start..self.start
        };
        let after = if other.end <= self.end {
            within_end = other.end;
            self.end..self.end
        } else {
            if other.start >= self.end {
                return (
                    self.start..self.start,
                    self.start..self.start,
                    other.clone(),
                );
            }
            self.end..other.end
        };
        let within = within_start..within_end;
        (before, within, after)
    }
}

#[derive(Debug)]
struct MapRange {
    src: Range<u64>,
    dst: Range<u64>,
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
impl MapRange {
    fn do_map(&self, value: &Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        let (before, within, after) = self.src.intersect(value);
        let rest = [before, after]
            .iter()
            .filter(|x| !x.is_empty())
            .cloned()
            .collect();
        let result = if !within.is_empty() {
            Some(
                (self.dst.start + (within.start - self.src.start))
                    ..(self.dst.start + (within.end - self.src.start)),
            )
        } else {
            None
        };
        (result, rest)
    }
}
#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new<E: Debug, T: Iterator<Item = Result<String, E>>>(lines: &mut T) -> Self {
        let mut ranges = Vec::new();
        for l in lines.by_ref() {
            let l = l.unwrap();
            if let Ok(range) = l.parse() {
                ranges.push(range);
                break;
            }
        }
        for l in lines.by_ref() {
            let l = l.unwrap();
            if let Ok(range) = l.parse() {
                ranges.push(range);
            } else {
                break;
            }
        }
        Self { ranges }
    }

    fn do_map(&self, value: Range<u64>) -> Vec<Range<u64>> {
        let mut result = Vec::new();
        let mut still_to_map = vec![value];
        while !still_to_map.is_empty() {
            let mut was_mapped = false;
            let value = still_to_map.pop().unwrap();
            for range in self.ranges.iter() {
                let (mapped, mut rest) = range.do_map(&value);
                if let Some(r) = mapped {
                    result.push(r);
                    still_to_map.append(&mut rest);
                    was_mapped = true;
                    break;
                }
            }
            if !was_mapped {
                result.push(value);
            }
        }
        result
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

fn chain_map(maps: &Vec<Map>, value: Range<u64>) -> u64 {
    let mut value = vec![value];
    for m in maps {
        value = value.into_iter().flat_map(|v| m.do_map(v)).collect();
    }
    value.into_iter().map(|x| x.start).min().unwrap()
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
    let res1 = seeds
        .iter()
        .map(|&s| s..s + 1)
        .map(|s| chain_map(&maps, s))
        .min()
        .unwrap();
    let res2 = seeds[..]
        .chunks(2)
        .map(|x| x[0]..x[0] + x[1])
        .map(|s| chain_map(&maps, s))
        .min()
        .unwrap();
    (res1, res2).into()
}
