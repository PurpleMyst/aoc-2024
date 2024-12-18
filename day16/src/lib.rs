use std::fmt::Display;

use ::bucket_queue::*;
use fixedbitset::FixedBitSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: (u8, u8),
    dir: (i8, i8),
}

impl State {
    fn advance(self, walkable: &FixedBitSet, side: usize) -> impl Iterator<Item = (Self, usize)> {
        [
            {
                let next_y = self.pos.0.wrapping_add_signed(self.dir.0);
                let next_x = self.pos.1.wrapping_add_signed(self.dir.1);
                (walkable[usize::from(next_y) * side + usize::from(next_x)]).then_some({
                    (
                        Self {
                            pos: (next_y, next_x),
                            dir: self.dir,
                        },
                        1,
                    )
                })
            },
            Some((
                Self {
                    pos: self.pos,
                    dir: (-self.dir.1, self.dir.0),
                },
                1000,
            )),
            Some((
                Self {
                    pos: self.pos,
                    dir: (self.dir.1, -self.dir.0),
                },
                1000,
            )),
        ]
        .into_iter()
        .flatten()
    }

    // branchless to_index
    fn to_index(self, side: usize) -> usize {
        let pos_index = (usize::from(self.pos.0) * side + usize::from(self.pos.1)) * 4;
        let dir0_pos = (self.dir.0 > 0) as usize; // 1 if dir.0 > 0, else 0
        let dir0_neg = (self.dir.0 < 0) as usize; // 1 if dir.0 < 0, else 0
        let dir1_neg = (self.dir.1 < 0) as usize; // 1 if dir.1 < 0, else 0
        let dir_index = dir0_pos + (dir0_neg * 3) + (dir1_neg * 2);
        pos_index + dir_index
    }

    fn rewind(self, walkable: &FixedBitSet, side: usize) -> impl Iterator<Item = (Self, usize)> {
        [
            {
                let next_y = self.pos.0.wrapping_add_signed(-self.dir.0);
                let next_x = self.pos.1.wrapping_add_signed(-self.dir.1);
                (walkable[usize::from(next_y) * side + usize::from(next_x)]).then_some({
                    (
                        Self {
                            pos: (next_y, next_x),
                            dir: self.dir,
                        },
                        1,
                    )
                })
            },
            Some((
                Self {
                    pos: self.pos,
                    dir: (-self.dir.1, self.dir.0),
                },
                1000,
            )),
            Some((
                Self {
                    pos: self.pos,
                    dir: (self.dir.1, -self.dir.0),
                },
                1000,
            )),
        ]
        .into_iter()
        .flatten()
    }
}

fn distance_map<const REVERSE: bool>(start: State, goal: (u8, u8), map: &FixedBitSet, side: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; side * side * 4];
    let mut pq = BucketQueue::<Vec<_>>::new();
    pq.push(start, 0);
    dist[start.to_index(side)] = 0;
    while let Some(d) = pq.min_priority() {
        let state = pq.pop_min().unwrap();
        if state.pos == goal {
            break;
        }

        if REVERSE {
            for (next, w) in state.rewind(map, side) {
                let i = next.to_index(side);
                if dist[i] > d + w {
                    dist[i] = d + w;
                    pq.push(next, d + w);
                }
            }
        } else {
            for (next, w) in state.advance(map, side) {
                let i = next.to_index(side);
                if dist[i] > d + w {
                    dist[i] = d + w;
                    pq.push(next, d + w);
                }
            }
        }
    }

    dist
}

// logic adapted from https://www.reddit.com/r/adventofcode/comments/1hfboft/2024_day_16_solutions/m2akf0n/
#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let side = input.lines().next().unwrap().len();

    let map = input.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();

    let mut start = usize::MAX;
    let mut end = usize::MAX;
    let mut walkable = FixedBitSet::with_capacity(side * side);

    for (i, b) in map.iter().enumerate() {
        match b {
            b'S' => start = i,
            b'E' => end = i,
            b'#' => continue,
            _ => {}
        }
        walkable.insert(i);
    }
    let (start_y, start_x) = ((start / side) as u8, (start % side) as u8);
    let (end_y, end_x) = ((end / side) as u8, (end % side) as u8);

    let (forward_dist_by_state, reverse_dist_by_state) = rayon::join(
        || {
            distance_map::<false>(
                State {
                    pos: (start_y, start_x),
                    dir: (0, 1),
                },
                (end_y, end_x),
                &walkable,
                side,
            )
        },
        || {
            distance_map::<true>(
                State {
                    pos: (end_y, end_x),
                    dir: (-1, 0),
                },
                (start_y, start_x),
                &walkable,
                side,
            )
        },
    );

    let p1 = forward_dist_by_state[State {
        pos: (end_y, end_x),
        dir: (-1, 0),
    }
    .to_index(side)];

    let mut sit_set = FixedBitSet::with_capacity(side * side);
    for ((s1_idx, d1), d2) in forward_dist_by_state.into_iter().enumerate().zip(reverse_dist_by_state) {
        if d1 != usize::MAX && d2 != usize::MAX && d1 + d2 == p1 {
            sit_set.insert(s1_idx / 4);
        }
    }
    let p2 = sit_set.count_ones(..);

    (p1, p2)
}
