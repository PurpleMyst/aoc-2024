use std::fmt::Display;

use rayon::prelude::*;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let map = grid::Grid::from_vec(
        input.bytes().filter(|&b| b != b'\n').collect(),
        input.lines().next().unwrap().len(),
    );

    let (start_pos, _) = map.indexed_iter().find(|&(_, &c)| c == b'^').unwrap();

    let p1 = do_solve(&map, start_pos)
        .0
        .into_vec()
        .into_iter()
        .filter(|&v| v != 0)
        .count();

    let p2 = map
        .indexed_iter()
        .par_bridge()
        .filter(|&(_, &c)| c == b'.')
        .filter(|(pos, _)| {
            let mut new_map = map.clone();
            new_map[*pos] = b'#';
            let (_, enters_loop) = do_solve(&new_map, start_pos);
            enters_loop
        })
        .count();

    (p1, p2)
}

fn do_solve(map: &grid::Grid<u8>, mut pos: (usize, usize)) -> (grid::Grid<u8>, bool) {
    let mut visited = grid::Grid::new(map.rows(), map.cols());
    visited.fill(0u8);

    loop {
        for (dir, (dy, dx)) in DIRECTIONS.into_iter().enumerate() {
            let mask = 1 << dir;
            loop {
                if visited[pos] & mask != 0 {
                    return (visited, true);
                }
                visited[pos] |= mask;
                let new_pos = pos.0.checked_add_signed(dy).zip(pos.1.checked_add_signed(dx));
                match new_pos.and_then(|p| map.get(p.0, p.1)) {
                    Some(b'#') => {
                        break;
                    }
                    Some(..) => pos = new_pos.unwrap(),
                    None => return (visited, false),
                }
            }
        }
    }
}
