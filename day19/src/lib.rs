use std::fmt::Display;

use memoize::memoize;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn can_make(tiles: &'static [&'static [u8]], pattern: &'static [u8]) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    tiles
        .iter()
        .copied()
        .map(|tile| pattern.strip_prefix(tile).map_or(0, |rest| can_make(tiles, rest)))
        .sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    memoized_flush_can_make();
    let input = include_str!("input.txt");

    let (tiles, patterns) = input.split_once("\n\n").unwrap();
    let tiles = tiles.split(", ").map(|t| t.as_bytes()).collect::<Vec<_>>().leak();

    patterns
        .par_lines()
        .map(|p| can_make(&*tiles, p.as_bytes()))
        .map(|n| (if n > 0 { 1 } else { 0 }, n))
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
}
