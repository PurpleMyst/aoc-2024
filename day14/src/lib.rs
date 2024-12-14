use std::fmt::Display;

use rayon::prelude::*;

type Coord = i32;
type Pair = (Coord, Coord);

const WIDTH: Coord = 101;
const HEIGHT: Coord = 103;

const TIME: Coord = 100;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Pair,
    velocity: Pair,
}

impl Robot {
    fn parse(s: &str) -> Self {
        let (_, s) = s.split_once('=').unwrap();
        let (position, velocity) = s.split_once(" v=").unwrap();
        let position = position.trim().split_once(',').unwrap();
        let velocity = velocity.trim().split_once(',').unwrap();
        Self {
            position: (position.0.parse().unwrap(), position.1.parse().unwrap()),
            velocity: (velocity.0.parse().unwrap(), velocity.1.parse().unwrap()),
        }
    }

    fn advance(&self, t: Coord) -> Pair {
        (
            (self.position.0 + self.velocity.0 * t).rem_euclid(WIDTH),
            (self.position.1 + self.velocity.1 * t).rem_euclid(HEIGHT),
        )
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let robots = input.lines().map(Robot::parse).collect::<Vec<_>>();

    (solve_part1(&robots), solve_part2(&robots))
}

fn solve_part1(robots: &[Robot]) -> usize {
    robots
        .par_iter()
        .map(|r| r.advance(TIME))
        .filter_map(|(x, y)| {
            if x == WIDTH / 2 || y == HEIGHT / 2 {
                return None;
            }
            Some(match (x < WIDTH / 2, y < HEIGHT / 2) {
                (true, true) => [1, 0, 0, 0],
                (true, false) => [0, 1, 0, 0],
                (false, true) => [0, 0, 1, 0],
                (false, false) => [0, 0, 0, 1],
            })
        })
        .reduce(
            || [0, 0, 0, 0],
            |mut acc, x| {
                for (a, b) in acc.iter_mut().zip(x.iter()) {
                    *a += b;
                }
                acc
            },
        )
        .into_iter()
        .product()
}

// not the biggest fan of this puzzle, i guess it's nice but the requirements were really vague
// what even is a christmas tree?
fn solve_part2(robots: &[Robot]) -> i32 {
    (0..10_000)
        .into_par_iter()
        .find_first(|&t| {
            let mut screen = fixedbitset::FixedBitSet::with_capacity((WIDTH * HEIGHT) as usize);
            for r in robots {
                let (x, y) = r.advance(t);
                if screen.contains((y * WIDTH + x) as usize) {
                    return false;
                }
                screen.insert((y * WIDTH + x) as usize);
            }
            true
        })
        .unwrap()
}
