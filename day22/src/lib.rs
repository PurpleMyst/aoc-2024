use std::{
    fmt::Display,
    sync::atomic::{AtomicU16, AtomicU64, Ordering},
};

use rayon::prelude::*;

// to_index(&[9,9,9,9]) + 1 = 130321
const SIZE: usize = 130_321;

fn step(mut x: u64) -> u64 {
    x = x ^ (x << 6) & 0xffffff;
    x = x ^ (x >> 5) % 0xffffff;
    x = x ^ (x << 11) % 0x0100_0000;
    x
}

fn generator(seed: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(seed), |&x| Some(step(x)))
}

fn to_index(deltas: &[i8]) -> usize {
    let mut index = 0;
    for &d in deltas {
        index = 19 * index + (d + 9) as usize;
    }
    index
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let p1 = AtomicU64::new(0);

    let scores = (0..SIZE).map(|_| AtomicU16::new(0)).collect::<Vec<_>>();

    include_str!("input.txt")
        .par_lines()
        .map(|line| {
            let n = line.parse::<u64>().unwrap();
            generator(n).take(2001).collect::<Vec<_>>()
        })
        .for_each(|values| {
            p1.fetch_add(values[values.len() - 1], Ordering::Relaxed);
            let deltas = values
                .iter()
                .skip(1)
                .scan(values[0], |prev, &x| {
                    let diff = x as i64 % 10 - *prev as i64 % 10;
                    *prev = x;
                    Some(diff as i8)
                })
                .collect::<Vec<_>>();

            let mut seen = fixedbitset::FixedBitSet::with_capacity(SIZE);
            deltas.windows(4).enumerate().for_each(|(i, w)| {
                let idx = to_index(w);
                if !seen.contains(idx) {
                    seen.insert(idx);
                    let price = (values[i + 4] % 10) as u16;
                    scores[idx].fetch_add(price, Ordering::Relaxed);
                }
            })
        });

    let p1 = p1.load(Ordering::Relaxed);
    let p2 = scores
        .into_iter()
        .map(|score| score.load(Ordering::Relaxed))
        .max()
        .unwrap();

    (p1, p2)
}
