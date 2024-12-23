use std::{collections::VecDeque, fmt::Display};

use fixedbitset::FixedBitSet;

const SIDE: u8 = 71;
const MAP_SIZE: usize = SIDE as usize * SIDE as usize;

fn pos2idx(x: u8, y: u8) -> usize {
    usize::from(y) * usize::from(SIDE) + usize::from(x)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    rayon::join(|| solve_part1(input), || solve_part2(input))
}

fn solve_part1(input: &str) -> u16 {
    let mut walls = FixedBitSet::with_capacity(MAP_SIZE);

    input.lines().take(1024).for_each(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<u8>().unwrap();
        let y = y.parse::<u8>().unwrap();
        walls.insert(pos2idx(x, y));
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
            .filter(|&(x, y)| x < SIDE && y < SIDE)
            .filter(|&(x, y)| !walls.contains(pos2idx(x, y)))
            .map(|(x, y)| ((x, y), 1))
        },
        |&(x, y)| u16::from(x.abs_diff(SIDE - 1)) + u16::from(y.abs_diff(SIDE - 1)),
        |&(x, y)| x == (SIDE - 1) && y == (SIDE - 1),
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
            let x = x.parse::<u8>().unwrap();
            let y = y.parse::<u8>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut walls = FixedBitSet::with_capacity(MAP_SIZE);
    walls_in_order.iter().for_each(|&(x, y)| walls.insert(pos2idx(x, y)));

    let mut queue = VecDeque::new();
    let mut visited = FixedBitSet::with_capacity(MAP_SIZE);

    queue.push_back((0u8, 0u8));

    loop {
        while let Some((x, y)) = queue.pop_front() {
            if x == (SIDE - 1) && y == (SIDE - 1) {
                return input.lines().nth(walls.count_ones(..)).unwrap();
            }

            visited.insert(pos2idx(x, y));

            queue.extend(
                [
                    (x.wrapping_add(1), y),
                    (x.wrapping_sub(1), y),
                    (x, y.wrapping_add(1)),
                    (x, y.wrapping_sub(1)),
                ]
                .into_iter()
                .filter(|&(x, y)| x < SIDE && y < SIDE)
                .filter(|&(x, y)| !walls.contains(pos2idx(x, y)))
                .filter(|&(x, y)| !visited.contains(pos2idx(x, y)))
            )
        }

        let (x, y) = walls_in_order.pop().unwrap();
        walls.remove(pos2idx(x, y));

        if [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|&(x, y)| x < SIDE && y < SIDE)
        .any(|(x, y)| visited.contains(pos2idx(x, y)))
        {
            queue.push_back((x, y));
        }
    }
}
