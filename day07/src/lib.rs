use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    rayon::join(|| do_solve::<false>(input), || do_solve::<true>(input))
}

fn ends_with(a: u64, b: u64) -> bool {
    b <= a && (a - b) % 10u64.pow(b.ilog10() + 1) == 0
}

// Solution translated from:
// https://www.reddit.com/r/adventofcode/comments/1h8l3z5/2024_day_7_solutions/m0tv6di/
fn is_tractable<const PART2: bool>(target: u64, values: &[u64]) -> bool {
    let (&n, head) = values.split_last().unwrap();
    if head.is_empty() {
        return n == target;
    }

    let q = target / n;
    let r = target % n;

    (r == 0 && is_tractable::<PART2>(q, head))
        || (PART2 && ends_with(target, n) && is_tractable::<PART2>(target / (10u64.pow(n.ilog10() + 1)), head))
        || (n <= target && is_tractable::<PART2>(target - n, head))
}

fn do_solve<const PART2: bool>(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let (target, values) = line.split_once(": ").unwrap();
            let target = target.parse::<u64>().unwrap();
            let values = values.split(' ').map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
            is_tractable::<PART2>(target, &values).then_some(target)
        })
        .sum::<u64>()
}
