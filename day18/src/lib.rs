use std::fmt::Display;

use fixedbitset::FixedBitSet;

const SIDE: usize = 71;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let mut walls = FixedBitSet::with_capacity(SIDE * SIDE);

    input.lines().take(1024).for_each(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        walls.insert(y * SIDE + x);
    });

    let (_, p1) = do_solve(&walls).unwrap();

    walls.clear();
    let mut used = FixedBitSet::with_capacity(SIDE * SIDE);
    do_solve(&walls).unwrap().0.into_iter().for_each(|(x, y)| {
        used.insert(usize::from(y) * SIDE + usize::from(x));
    });

    let p2 = input
        .lines()
        .find_map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            let idx = y * SIDE + x;
            walls.insert(y * SIDE + x);

            if used.contains(idx) {
                if let Some((path, _)) = do_solve(&walls) {
                    used.clear();
                    path.into_iter().for_each(|(x, y)| {
                        used.insert(usize::from(y) * SIDE + usize::from(x));
                    });
                } else {
                    return Some(format!("{x},{y}"));
                }
            }

            None
        })
        .unwrap();

    (p1, p2)
}

fn do_solve(walls: &FixedBitSet) -> Option<(Vec<(u8, u8)>, usize)> {
    pathfinding::prelude::astar(
        &(0u8, 0u8),
        |&(x, y)| {
            [
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .filter(|&(x, y)| usize::from(x) < SIDE && usize::from(y) < SIDE)
            .filter(|&(x, y)| !walls.contains(usize::from(y) * SIDE + usize::from(x)))
            .map(|(x, y)| ((x, y), 1))
        },
        |&(x, y)| usize::from(x).abs_diff(SIDE - 1) + usize::from(y).abs_diff(SIDE - 1),
        |&(x, y)| usize::from(x) == (SIDE - 1) && usize::from(y) == (SIDE - 1),
    )
}
