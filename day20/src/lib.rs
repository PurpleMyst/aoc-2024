use std::fmt::Display;

use fixedbitset::FixedBitSet;
use rayon::prelude::*;

type Point = (u8, u8);

#[inline]
fn neighbors((y, x): Point) -> [Point; 4] {
    [
        (y.wrapping_sub(1), x),
        (y.wrapping_add(1), x),
        (y, x.wrapping_sub(1)),
        (y, x.wrapping_add(1)),
    ]
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
    let (start_dist_map, end_dist_map) = rayon::join(
        || compute_dist_map(side, start, walls),
        || compute_dist_map(side, end, walls),
    );

    let base_time = start_dist_map[usize::from(end.0) * usize::from(side) + usize::from(end.1)];

    (0..side)
        .into_par_iter()
        .flat_map(move |y| (0..side).into_par_iter().map(move |x| (y, x)))
        .filter(|&(src_y, src_x)| {
            start_dist_map[usize::from(src_y) * usize::from(side) + usize::from(src_x)] != usize::MAX
        })
        .flat_map(|(src_y, src_x)| {
            (-STEPS..=STEPS)
                .into_par_iter()
                .flat_map(move |dy| (-STEPS..=STEPS).into_par_iter().map(move |dx| (dy, dx)))
                .filter(move |&(dy, dx)| {
                    let jump = dy.abs() + dx.abs();
                    jump <= STEPS
                })
                .map(move |(dy, dx)| {
                    (
                        src_y,
                        src_x,
                        src_y.wrapping_add_signed(dy),
                        src_x.wrapping_add_signed(dx),
                    )
                })
                .filter(move |&(_src_y, _src_x, dst_y, dst_x)| {
                    dst_y < side
                        && dst_x < side
                        && !walls.contains(usize::from(dst_y) * usize::from(side) + usize::from(dst_x))
                })
        })
        .filter(|&(src_y, src_x, dst_y, dst_x)| {
            let src_idx = usize::from(src_y) * usize::from(side) + usize::from(src_x);
            let dst_idx = usize::from(dst_y) * usize::from(side) + usize::from(dst_x);
            start_dist_map[src_idx]
                + end_dist_map[dst_idx]
                + src_y.abs_diff(dst_y) as usize
                + src_x.abs_diff(dst_x) as usize
                <= base_time - 100
        })
        .count()
}

fn compute_dist_map(side: u8, from_: (u8, u8), walls: &FixedBitSet) -> Vec<usize> {
    let mut dist_map = vec![usize::MAX; usize::from(side) * usize::from(side)];

    let mut states = vec![from_];
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
    dist_map
}
