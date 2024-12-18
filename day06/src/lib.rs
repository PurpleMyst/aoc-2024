use fixedbitset::FixedBitSet;
use rayon::prelude::*;
use std::fmt::Display;

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const SIDE: u8 = 130;

fn pos2idx(y: u8, x: u8) -> usize {
    usize::from(y) * usize::from(SIDE) + usize::from(x)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );
    debug_assert_eq!(map.rows(), map.cols());

    // Find the starting position.
    let (start_pos, _) = map.indexed_iter().find(|&(_, &c)| c == b'^').unwrap();
    let start_pos = (start_pos.0 as u8, start_pos.1 as u8);

    // Convert the map into a bitset of walls for faster lookup.
    let mut walls = FixedBitSet::with_capacity(usize::from(SIDE) * usize::from(SIDE));
    for (pos, &c) in map.indexed_iter() {
        if c == b'#' {
            walls.insert(pos2idx(pos.0 as u8, pos.1 as u8));
        }
    }

    // Walk the walk for part 1.
    let p1_visited_by_dir = do_solve(&walls, None, start_pos).0;

    // We've got a bitset for each direction, but we need to combine them into one for part one.
    let (first, rest) = p1_visited_by_dir.split_first().unwrap();
    let mut p1_visited = first.clone();
    for visited in rest {
        p1_visited |= visited;
    }
    let p1 = p1_visited.count_ones(..);

    // Let's move on to part 2.
    // We'll utilize the visited map for part one as candidates for obstacles; since the problem text specifies the
    // start is not an option, we'll just remove it from consideration.
    p1_visited.remove(pos2idx(start_pos.0, start_pos.1));
    let p2 = (0..SIDE)
        .into_par_iter()
        .flat_map(|y| (0..SIDE).into_par_iter().map(move |x| (y, x)))
        .filter(|&(y, x)| p1_visited.contains(pos2idx(y, x)))
        .filter(|&(y, x)| {
            let (_, enters_loop) = do_solve(&walls, Some((y, x)), start_pos);
            enters_loop
        })
        .count();

    (p1, p2)
}

fn do_solve(
    walls: &FixedBitSet,
    extra_wall: Option<(u8, u8)>,
    (mut y, mut x): (u8, u8),
) -> ([FixedBitSet; DIRECTIONS.len()], bool) {
    let mut visited_by_dir: [FixedBitSet; DIRECTIONS.len()] =
        std::array::from_fn(|_| FixedBitSet::with_capacity(usize::from(SIDE) * usize::from(SIDE)));

    loop {
        for ((dy, dx), visited) in DIRECTIONS.into_iter().zip(&mut visited_by_dir) {
            loop {
                let idx = pos2idx(y, x);
                if visited.contains(idx) {
                    return (visited_by_dir, true);
                }
                visited.insert(idx);

                // Move into the new direction, checking if we go out of bounds.
                // Since SIDE is small enough, overflow'll never happen and underflow is beneficial (it allows us to
                // avoid checking for negative values).
                let new_y = y.wrapping_add_signed(dy);
                let new_x = x.wrapping_add_signed(dx);
                if new_y >= SIDE || new_x >= SIDE {
                    return (visited_by_dir, false);
                }

                let new_idx = pos2idx(new_y, new_x);
                if Some((new_y, new_x)) == extra_wall || walls.contains(new_idx) {
                    break;
                }

                y = new_y;
                x = new_x;
            }
        }
    }
}
