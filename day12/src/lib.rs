// translated from https://old.reddit.com/r/adventofcode/comments/1hcdnk0/2024_day_12_solutions/m1nio0w/
use std::fmt::Display;

use fixedbitset::FixedBitSet;
use petgraph::unionfind::UnionFind;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

const SIDE: usize = 140;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn pos2idx<T>(x: T, y: T) -> usize
where
    T: Into<usize>,
{
    y.into() * SIDE + x.into()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let grid: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();

    // Find regions
    let mut uf = UnionFind::<usize>::new(SIDE * SIDE);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let current_char = grid[pos2idx(x, y)];
            let current_index = pos2idx(x, y);

            // Check right neighbor
            if x + 1 < SIDE && grid[pos2idx(x + 1, y)] == current_char {
                uf.union(current_index, pos2idx(x + 1, y));
            }

            // Check down neighbor
            if y + 1 < SIDE && grid[pos2idx(x, y + 1)] == current_char {
                uf.union(current_index, pos2idx(x, y + 1));
            }
        }
    }

    // Collect subsets (connected components)
    let mut regions: HashMap<u16, Vec<(u8, u8)>> = HashMap::default();
    for y in 0..SIDE {
        for x in 0..SIDE {
            let index = pos2idx(x, y);
            let root = uf.find(index);
            regions.entry(root as u16).or_default().push((x as u8, y as u8));
        }
    }

    regions
        .values()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|subset| {
            let a = subset.len();
            let mut p = 0;
            let mut s = 0;

            let set: FixedBitSet = subset.iter().map(|&(x, y)| pos2idx(x, y)).collect();
            let in_region = |x, y| set.contains(pos2idx(x, y));

            for &(x, y) in subset {
                // Perimeter calculation
                for &(dx, dy) in &DIRECTIONS {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if !(0..SIDE as isize).contains(&nx)
                        || !(0..SIDE as isize).contains(&ny)
                        || !set.contains(pos2idx(nx as usize, ny as usize))
                    {
                        p += 1;
                    }
                }

                // Outer corners
                // Top-left
                if !in_region(x.wrapping_sub(1), y) && !in_region(x, y.wrapping_sub(1)) {
                    s += 1;
                }
                // Top-right
                if !in_region(x + 1, y) && !in_region(x, y.wrapping_sub(1)) {
                    s += 1;
                }
                // Bottom-left
                if !in_region(x.wrapping_sub(1), y) && !in_region(x, y + 1) {
                    s += 1;
                }
                // Bottom-right
                if !in_region(x + 1, y) && !in_region(x, y + 1) {
                    s += 1;
                }

                // Inner corners
                // Top-left
                if in_region(x.wrapping_sub(1), y)
                    && in_region(x, y.wrapping_sub(1))
                    && !in_region(x.wrapping_sub(1), y.wrapping_sub(1))
                {
                    s += 1;
                }
                // Top-right
                if in_region(x + 1, y) && in_region(x, y.wrapping_sub(1)) && !in_region(x + 1, y.wrapping_sub(1)) {
                    s += 1;
                }
                // Bottom-left
                if in_region(x.wrapping_sub(1), y) && in_region(x, y + 1) && !in_region(x.wrapping_sub(1), y + 1) {
                    s += 1;
                }
                // Bottom-right
                if in_region(x + 1, y) && in_region(x, y + 1) && !in_region(x + 1, y + 1) {
                    s += 1;
                }
            }

            (a * p, a * s)
        })
        .reduce(|| (0, 0), |(t1_acc, t2_acc), (t1, t2)| (t1_acc + t1, t2_acc + t2))
}
