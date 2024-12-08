use std::fmt::Display;

use arrayvec::ArrayVec;
use rayon::prelude::*;

fn is_safe(report: &[u8]) -> bool {
    let cond1 = report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);

    let cond2 = report.windows(2).all(|w| {
        let d = w[0].abs_diff(w[1]);
        (1..=3).contains(&d)
    });

    cond1 && cond2
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let reports = input
        .par_lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<ArrayVec<_, 8>>()
        })
        .collect::<Vec<_>>();

    let p1 = reports.par_iter().filter(|report| is_safe(report)).count();

    let p2 = reports
        .into_par_iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let mut r = report.clone();
                    r.remove(i);
                    is_safe(&r)
                })
        })
        .count();

    (p1, p2)
}
