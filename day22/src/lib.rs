use std::fmt::Display;
use rayon::prelude::*;
use fixedbitset::FixedBitSet;

const SIZE: usize = 130_321;
const CHUNK_SIZE: usize = 2256 / 20;

fn step(mut x: u64) -> u64 {
    x = x ^ (x << 6) & 0xffffff;
    x = x ^ (x >> 5) & 0xffffff;
    x = x ^ (x << 11) & 0xffffff;
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

#[derive(Default)]
struct ChunkResult {
    sum: u64,
    scores: Vec<u16>,
}

impl ChunkResult {
    fn new() -> Self {
        ChunkResult {
            sum: 0,
            scores: vec![0; SIZE],
        }
    }

    fn merge(mut self, other: Self) -> Self {
        self.sum += other.sum;
        for (s, &o) in self.scores.iter_mut().zip(other.scores.iter()) {
            *s += o;
        }
        self
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // Collect all lines first
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    
    // Process in chunks
    let results = lines
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| {
            let mut chunk_result = ChunkResult::new();
            
            for line in chunk {
                let n = line.parse::<u64>().unwrap();
                let values: Vec<_> = generator(n).take(2001).collect();
                
                // Part 1
                chunk_result.sum += values[values.len() - 1];
                
                // Part 2
                let deltas: Vec<_> = values
                    .iter()
                    .skip(1)
                    .scan(values[0], |prev, &x| {
                        let diff = x as i64 % 10 - *prev as i64 % 10;
                        *prev = x;
                        Some(diff as i8)
                    })
                    .collect();

                let mut seen = FixedBitSet::with_capacity(SIZE);
                deltas.windows(4).enumerate().for_each(|(i, w)| {
                    let idx = to_index(w);
                    if !seen.contains(idx) {
                        seen.insert(idx);
                        let price = (values[i + 4] % 10) as u16;
                        chunk_result.scores[idx] += price;
                    }
                });
            }

            chunk_result
        })
        .reduce(ChunkResult::new, |a, b| a.merge(b));

    let p1 = results.sum;
    let p2 = results.scores.into_iter().max().unwrap();

    (p1, p2)
}
