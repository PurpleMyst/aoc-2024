use std::{fmt::Display, iter::zip};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.trim().parse::<u32>().unwrap()))
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let p1 = zip(left.iter(), right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u32>();

    let mut freq = vec![0; 100_000];
    right.iter().for_each(|&n| freq[n as usize] += 1);
    let p2 = left.iter().map(|n| n * freq[*n as usize]).sum::<u32>();

    (p1, p2)
}
