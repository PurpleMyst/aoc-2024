use std::fmt::Display;

use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

fn step(mut x: u64) -> u64 {
    x = x ^ (x * 64) % 0x0100_0000;
    x = x ^ (x / 32) % 0x0100_0000;
    x = x ^ (x * 2048) % 0x0100_0000;
    x
}

fn generator(seed: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(seed), |&x| Some(step(x)))
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let values = include_str!("input.txt")
        .lines()
        .map(|line| {
            let n = line.parse::<u64>().unwrap();
            generator(n).take(2001).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = values.iter().map(|v| v.last().unwrap()).sum::<u64>();

    let deltas = values
        .iter()
        .map(|values| {
            let deltas = values
                .iter()
                .skip(1)
                .scan(values[0], |prev, &x| {
                    let diff = x as i64 % 10 - *prev as i64 % 10;
                    *prev = x;
                    Some(diff)
                })
                .collect::<Vec<_>>();

            deltas
                .windows(4)
                .enumerate()
                .rev()
                .map(|(i, w)| {
                    let mut needle = [0; 4];
                    needle.copy_from_slice(w);
                    (needle, values[i + 4] % 10)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let p2 = itertools::iproduct!((-9..=9), (-9..=9), (-9..=9), (-9..=9),)
        .par_bridge()
        .map(|needle| {
            let needle = <[i64; 4]>::from(needle);
            deltas.iter().filter_map(|ds| ds.get(&needle)).sum::<u64>()
        })
        .max()
        .unwrap();

    (p1, p2)
}
