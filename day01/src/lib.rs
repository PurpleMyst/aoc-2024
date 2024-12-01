use std::{fmt::Display, iter::zip};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut l = Vec::new();
    let mut r = Vec::new();
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(a, b)| {
            l.push(a.trim().parse::<i64>().unwrap());
            r.push(b.trim().parse::<i64>().unwrap());
        });

    l.sort_unstable();
    r.sort_unstable();

    let p1 = zip(l.iter(), r.iter()).map(|(a, b)| (a - b).abs()).sum::<i64>();

    let p2 = l
        .iter()
        .map(|n| n * r.iter().filter(|&m| n == m).count() as i64)
        .sum::<i64>();

    (p1, p2)
}
