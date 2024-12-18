use std::{collections::VecDeque, fmt::Display};

use fixedbitset::FixedBitSet;

const SIDE: usize = 71;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    rayon::join(|| solve_part1(input), || solve_part2(input))
}

fn solve_part1(input: &str) -> usize {
    let mut walls = FixedBitSet::with_capacity(SIDE * SIDE);

    input.lines().take(1024).for_each(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        walls.insert(y * SIDE + x);
    });

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
    .unwrap()
    .1
}

// code inspired by
// https://www.reddit.com/r/adventofcode/comments/1hgv0mt/2024_day_18_part_2_if_it_aint_broke_dont_fix_it/m2mmfx7/:walls_in_order
fn solve_part2(input: &str) -> &str {
    let mut walls_in_order = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut walls = FixedBitSet::with_capacity(SIDE * SIDE);
    walls_in_order.iter().for_each(|&(x, y)| walls.insert(y * SIDE + x));

    let mut queue = VecDeque::new();
    let mut visited = FixedBitSet::with_capacity(SIDE * SIDE);

    queue.push_back((0, 0));

    loop {
        while let Some((x, y)) = queue.pop_front() {
            if x == (SIDE - 1) && y == (SIDE - 1) {
                return input.lines().nth(walls.count_ones(..)).unwrap();
            }

            visited.insert(y * SIDE + x);

            queue.extend(
                [
                    (x.wrapping_add(1), y),
                    (x.wrapping_sub(1), y),
                    (x, y.wrapping_add(1)),
                    (x, y.wrapping_sub(1)),
                ]
                .into_iter()
                .filter(|&(x, y)| x < SIDE && y < SIDE)
                .filter(|&(x, y)| !walls.contains(y * SIDE + x))
                .filter(|&(x, y)| !visited.contains(y * SIDE + x)),
            )
        }

        let (x, y) = walls_in_order.pop().unwrap();
        let idx = y * SIDE + x;
        walls.remove(idx);

        if [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|&(x, y)| x < SIDE && y < SIDE)
        .any(|(x, y)| visited.contains(y * SIDE + x))
        {
            queue.push_back((x, y));
        }
    }
}
