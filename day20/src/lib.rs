use std::fmt::Display;

use fixedbitset::FixedBitSet;
use rayon::prelude::*;

type Point = (u8, u8);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: Point,
}

#[inline]
fn neighbors((y, x): Point) -> [Point; 4] {
    [
        (y.wrapping_sub(1), x),
        (y.wrapping_add(1), x),
        (y, x.wrapping_sub(1)),
        (y, x.wrapping_add(1)),
    ]
}

impl State {
    fn step(self, side: u8, walls: &FixedBitSet) -> impl Iterator<Item = Self> + '_ {
        neighbors(self.pos)
            .into_iter()
            .filter(move |&(y, x)| y < side && x < side)
            .filter(move |&(y, x)| !walls.contains(usize::from(y) * usize::from(side) + usize::from(x)))
            .map(|pos| State { pos })
    }

    fn cheat<const STEPS: i8>(self, side: u8, walls: &FixedBitSet) -> impl Iterator<Item = (usize, Self)> + '_ {
        (-STEPS..=STEPS)
            .flat_map(move |dy| (-STEPS..=STEPS).map(move |dx| (dy, dx)))
            .filter(|&(dy, dx)| dy.abs() + dx.abs() <= STEPS)
            .map(move |(dy, dx)| (self.pos.0.wrapping_add_signed(dy), self.pos.1.wrapping_add_signed(dx)))
            .filter(move |&(y, x)| y < side && x < side)
            .filter(move |&(y, x)| !walls.contains(usize::from(y) * usize::from(side) + usize::from(x)))
            .map(move |pos| {
                (
                    usize::from(self.pos.0.abs_diff(pos.0)) + usize::from(self.pos.1.abs_diff(pos.1)),
                    State { pos },
                )
            })
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let side = u8::try_from(input.lines().next().unwrap().len()).unwrap();

    let mut start = (u8::MAX, u8::MAX);
    let mut end = (u8::MAX, u8::MAX);
    let mut walls = FixedBitSet::with_capacity(usize::from(side) * usize::from(side));

    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.bytes().enumerate() {
            match cell {
                b'S' => start = (y as u8, x as u8),
                b'E' => end = (y as u8, x as u8),
                b'#' => walls.insert(usize::from(y) * usize::from(side) + usize::from(x)),
                b'.' | b'\n' => (),
                _ => unreachable!("{cell:?}"),
            }
        }
    }

    rayon::join(
        || do_solve::<2>(start, side, &walls, end),
        || do_solve::<20>(start, side, &walls, end),
    )
}

fn do_solve<const STEPS: i8>(start: (u8, u8), side: u8, walls: &FixedBitSet, end: (u8, u8)) -> usize {
    let states = pathfinding::prelude::bfs(
        &State { pos: start },
        |state| state.step(side, &walls),
        |state| state.pos == end,
    )
    .unwrap();
    let base_time = states.len() - 1;

    let cheats = states
        .into_iter()
        .enumerate()
        .flat_map(|(t, state0)| {
            state0
                .cheat::<STEPS>(side, &walls)
                .map(move |(t0, state)| (t0 + t, state0.pos, state))
        })
        .collect::<rustc_hash::FxHashSet<_>>();

    let mut dist_map = vec![usize::MAX; usize::from(side) * usize::from(side)];

    let mut states = vec![end];
    let mut new_states = Vec::new();
    let mut visited = FixedBitSet::with_capacity(usize::from(side) * usize::from(side));

    let mut t = 0;
    while !states.is_empty() {
        for state in states.drain(..) {
            let idx = usize::from(state.0) * usize::from(side) + usize::from(state.1);
            if visited.contains(idx) {
                continue;
            }
            visited.insert(idx);

            dist_map[idx] = t;

            for neighbor in neighbors(state) {
                if neighbor.0 >= side || neighbor.1 >= side {
                    continue;
                }
                let idx = usize::from(neighbor.0) * usize::from(side) + usize::from(neighbor.1);
                if walls.contains(idx) || visited.contains(idx) {
                    continue;
                }
                new_states.push(neighbor);
            }
        }
        t += 1;
        std::mem::swap(&mut states, &mut new_states);
    }

    cheats
        .into_par_iter()
        .map(|(t0, _, state)| t0 + dist_map[usize::from(state.pos.0) * usize::from(side) + usize::from(state.pos.1)])
        .filter(|&t| t <= base_time - 100)
        .count()
}
