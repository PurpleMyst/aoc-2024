use std::fmt::Display;

use hibitset::BitSet;
use rayon::prelude::*;

// Directions, ordered clockwise and as (dy, dx) tuples starting from "up".
// We're utilizing 16-bit integers as positions to be faster and more memory efficient.
// For whatever reason, using 8-bit integers is slower. \_(ツ)_/
const DIRECTIONS: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    debug_assert_eq!(map.rows(), map.cols());
    let side = map.cols() as u16;
    debug_assert!(side < u16::MAX);

    // Find the starting position.
    let (start_pos, _) = map.indexed_iter().find(|&(_, &c)| c == b'^').unwrap();
    let start_pos = (start_pos.0 as u16, start_pos.1 as u16);

    // Convert the map into a bitset of walls for faster lookup.
    let mut walls = BitSet::with_capacity(side as u32 * side as u32);
    for (pos, &c) in map.indexed_iter() {
        if c == b'#' {
            walls.add((pos.0 * map.cols() + pos.1) as u32);
        }
    }

    // Walk the walk for part 1.
    let p1_visited_by_dir = do_solve(&walls, side, start_pos).0;

    // We've got a bitset for each direction, but we need to combine them into one for part one.
    let (first, rest) = p1_visited_by_dir.split_first().unwrap();
    let mut p1_visited = first.clone();
    for visited in rest {
        p1_visited |= visited;
    }
    let p1 = p1_visited.layer0_as_slice().iter().map(|n| n.count_ones()).sum::<u32>();

    // Let's move on to part 2.
    // We'll utilize the visited map for part one as candidates for obstacles; since the problem text specifies the
    // start is not an option, we'll just set remove it from consdieration.
    p1_visited.remove((start_pos.0 * side + start_pos.1) as u32);
    let p2 = (0..side)
        .into_par_iter()
        .flat_map(|y| (0..side).into_par_iter().map(move |x| (y, x)))
        .filter(|(y, x)| p1_visited.contains((y * side + x) as u32))
        .filter(|(y, x)| {
            let mut new_walls = walls.clone();
            new_walls.add((y * side + x) as u32);
            let (_, enters_loop) = do_solve(&new_walls, side, start_pos);
            enters_loop
        })
        .count();

    (p1, p2)
}

fn do_solve(walls: &BitSet, side: u16, (mut y, mut x): (u16, u16)) -> ([BitSet; DIRECTIONS.len()], bool) {
    let mut visited_by_dir: [_; DIRECTIONS.len()] = std::array::from_fn(|_| BitSet::new());

    loop {
        for ((dy, dx), visited) in DIRECTIONS.into_iter().zip(&mut visited_by_dir) {
            loop {
                let idx = (y * side + x) as u32;
                if visited.contains(idx) {
                    return (visited_by_dir, true);
                }
                visited.add(idx);

                // Move into the new direction, checking if we go out of bounds.
                // We're abusing wrapping here, since 0 - 1 = u16::MAX, so we're also implicitly assuming side to be way
                // smaller than that. Which is true.
                let new_y = y.wrapping_add_signed(dy);
                let new_x = x.wrapping_add_signed(dx);
                if new_y >= side || new_x >= side {
                    return (visited_by_dir, false);
                }

                if walls.contains((new_y * side + new_x) as u32) {
                    break;
                }

                y = new_y;
                x = new_x;
            }
        }
    }
}
