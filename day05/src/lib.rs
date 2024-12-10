use std::{cmp::Ordering, fmt::Display};

use rayon::prelude::*;

struct Order {
    map: [u128; 100],
}

impl Order {
    fn new() -> Self {
        Self { map: [0; 100] }
    }

    fn add(&mut self, before: u8, after: u8) {
        debug_assert!(before < 100);
        debug_assert!(after < 100);
        self.map[before as usize] |= 1 << after;
    }

    fn contains(&self, before: u8, after: u8) -> bool {
        debug_assert!(before < 100);
        debug_assert!(after < 100);
        self.map[before as usize] & (1 << after) != 0
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (order_s, updates) = input.split_once("\n\n").unwrap();

    let mut order = Order::new();
    order_s
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
        })
        .for_each(|(before, after)| order.add(before, after));

    let updates = updates
        .par_lines()
        .map(|update| update.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>());

    updates
        .map(|mut update| {
            let midpoint = update.len() / 2;
            if update.is_sorted_by(|&n, &m| order.contains(n, m)) {
                (update[midpoint] as u16, 0)
            } else {
                (
                    0,
                    *update
                        .select_nth_unstable_by(midpoint, |&n, &m| {
                            if order.contains(n, m) {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        })
                        .1 as u16,
                )
            }
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d))
}
