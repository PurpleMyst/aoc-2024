use std::fmt::Display;

use rayon::prelude::*;

const START: u8 = 0;
const END: u8 = 9;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct State {
    pos: (usize, usize),
}

impl State {
    fn advance<'a>(&'a self, grid: &'a grid::Grid<u8>) -> impl Iterator<Item = Self> + 'a {
        let (y, x) = self.pos;
        let &cell = grid.get(y, x).unwrap();

        DIRS.into_iter().filter_map(move |(dy, dx)| {
            let new_y = y.checked_add_signed(dy)?;
            let new_x = x.checked_add_signed(dx)?;
            let new_pos = (new_y, new_x);
            let &new_cell = grid.get(new_y, new_x)?;
            (new_cell == cell + 1).then(|| State { pos: new_pos })
        })
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let side = input.lines().count();
    debug_assert!(side < 64);
    let grid = grid::Grid::from_vec(input.bytes().filter(|&b| b != b'\n').map(|b| b - b'0').collect(), side);

    grid.indexed_iter()
        .filter(|(_, &c)| c == START)
        .map(|(pos, _)| State { pos })
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|state| {
            let mut states = vec![state];
            let mut reachable = vec![0u64; side];
            let mut p2 = 0;

            while let Some(state) = states.pop() {
                if let Some(&cell) = grid.get(state.pos.0, state.pos.1) {
                    if cell == END {
                        reachable[state.pos.0] |= 1 << state.pos.1;
                        p2 += 1;
                    } else {
                        states.extend(state.advance(&grid));
                    }
                }
            }

            let p1 = reachable.iter().map(|r| r.count_ones() as u64).sum::<u64>();
            (p1, p2)
        })
        .reduce(|| (0, 0), |(p1, p2), (p1_, p2_)| (p1 + p1_, p2 + p2_))
}
