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

    fn advance_x(&self, t: Coord) -> Coord {
        (self.position.0 + self.velocity.0 * t).rem_euclid(WIDTH)
    }

    fn advance_y(&self, t: Coord) -> Coord {
        (self.position.1 + self.velocity.1 * t).rem_euclid(HEIGHT)
    }

    fn advance(&self, t: Coord) -> Pair {
        (self.advance_x(t), self.advance_y(t))
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let robots = input.lines().map(Robot::parse).collect::<Vec<_>>();

    rayon::join(|| solve_part1(&robots), || solve_part2(&robots))
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
// modular arithmetic solution based on https://www.reddit.com/r/adventofcode/comments/1hdvhvu/2024_day_14_solutions/m1zws1g/
fn solve_part2(robots: &[Robot]) -> Coord {
    let (bx, by) = rayon::join(
        // Find the time `bx` with minimal variance in x coordinates
        || {
            (0..=WIDTH)
                .into_par_iter()
                .min_by_key(|&t| {
                    let xs: Vec<Coord> = robots.iter().map(|r| r.advance_x(t)).collect();
                    variance(&xs)
                })
                .unwrap()
        },
        // Find the time `by` with minimal variance in y coordinates
        || {
            (0..=HEIGHT)
                .into_par_iter()
                .min_by_key(|&t| {
                    let ys: Vec<Coord> = robots.iter().map(|r| r.advance_y(t)).collect();
                    variance(&ys)
                })
                .unwrap()
        },
    );

    // Compute the final time based on `bx` and `by` using the modular inverse, i.e. solve
    // t^* = b_x (mod W)
    // t^* = b_y (mod H)
    let inv_w = mod_inverse(WIDTH, HEIGHT).unwrap();
    bx + ((inv_w * (by - bx)) % HEIGHT) * WIDTH
}

/// Computes the variance of a slice of coordinates.
/// Uses floating-point arithmetic for accurate mean and variance calculations.
fn variance(data: &[Coord]) -> Coord {
    let len = data.len() as f64;
    if len == 0.0 {
        return Coord::MAX; // Return a large value if the data slice is empty
    }
    let mean = data.iter().map(|&x| x as f64).sum::<f64>() / len;
    let var = data
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / len;
    var as Coord
}

/// Computes the modular inverse of `a` modulo `m` using the Extended Euclidean Algorithm.
/// Returns `None` if the modular inverse does not exist.
fn mod_inverse(a: Coord, m: Coord) -> Option<Coord> {
    let (g, x, _) = extended_gcd(a, m);
    (g == 1).then(|| x.rem_euclid(m))
}

/// Extended Euclidean Algorithm.
/// Returns a tuple `(gcd, x, y)` such that `a*x + b*y = gcd`.
fn extended_gcd(a: Coord, b: Coord) -> (Coord, Coord, Coord) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x1, y1) = extended_gcd(b % a, a);
        (g, y1 - (b / a) * x1, x1)
    }
}
