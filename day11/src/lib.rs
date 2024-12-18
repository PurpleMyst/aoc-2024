use std::fmt::Display;

use memoize::memoize;
use rustc_hash::FxHashMap;

fn num_digits(n: u64) -> u64 {
    n.ilog10() as u64 + 1
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn stones(n: u64, steps_left: usize) -> usize {
    if steps_left == 0 {
        return 1;
    }

    if n == 0 {
        return stones(1, steps_left - 1);
    }

    let d = num_digits(n);
    if d % 2 == 0 {
        let left = n / 10u64.pow((d / 2) as u32);
        let right = n % 10u64.pow((d / 2) as u32);
        return stones(left, steps_left - 1) + stones(right, steps_left - 1);
    }

    stones(n * 2024, steps_left - 1)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    input
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .map(|n| (stones(n, 25), stones(n, 75)))
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap()
}
