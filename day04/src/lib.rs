use std::fmt::Display;

use grid::Grid;
use rayon::prelude::*;

const MAS: &[u8] = b"MAS";
const DIRECTIONS: [(isize, isize); 8] = [(0, -1), (0, 1), (-1, 0), (1, 0), (1, 1), (-1, 1), (-1, -1), (1, -1)];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let grid = Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    rayon::join(|| solve_part1(&grid), || solve_part2(&grid))
}

fn solve_part1(grid: &Grid<u8>) -> usize {
    DIRECTIONS
        .into_par_iter()
        .map(|dir| {
            grid.indexed_iter()
                .filter(|(_, &c)| c == b'X')
                .filter(|&((y, x), _)| {
                    (0..MAS.len())
                        .scan((y, x), |(y, x), _| {
                            *y = y.checked_add_signed(dir.0)?;
                            *x = x.checked_add_signed(dir.1)?;
                            grid.get(*y, *x)
                        })
                        .eq(MAS)
                })
                .count()
        })
        .sum()
}

fn solve_part2(grid: &Grid<u8>) -> usize {
    grid.indexed_iter()
        .filter(|(_, &c)| c == b'A')
        .filter(|((y, x), _)| {
            let first_diagonal = [
                (y.checked_sub(1), x.checked_sub(1)),
                (y.checked_add(1), x.checked_add(1)),
            ];
            let second_diagonal = [
                (y.checked_sub(1), x.checked_add(1)),
                (y.checked_add(1), x.checked_sub(1)),
            ];

            let do_count = |corners: [(Option<usize>, Option<usize>); 2]| {
                let mut m = 0u8;
                let mut s = 0u8;
                for (y, x) in corners {
                    match y.zip(x).and_then(|(y, x)| grid.get(y, x)) {
                        Some(b'M') => m += 1,
                        Some(b'S') => s += 1,
                        _ => {}
                    }
                }
                (m, s)
            };

            do_count(first_diagonal) == (1, 1) && do_count(second_diagonal) == (1, 1)
        })
        .count()
}
