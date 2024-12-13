use rayon::prelude::*;

use crate::part1;

type Coord = u64;
type Cost = u64;

const A_COST: f64 = 3.0;
const B_COST: f64 = 1.0;

const OFFSET: Coord = 10_000_000_000_000;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Pair,
    b: Pair,
    prize: Pair,
}

impl From<&part1::ClawMachine> for ClawMachine {
    fn from(machine: &part1::ClawMachine) -> Self {
        Self {
            a: Pair {
                x: Coord::from(machine.a.x),
                y: Coord::from(machine.a.y),
            },
            b: Pair {
                x: Coord::from(machine.b.x),
                y: Coord::from(machine.b.y),
            },
            prize: Pair {
                x: Coord::from(machine.prize.x) + OFFSET,
                y: Coord::from(machine.prize.y) + OFFSET,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    x: Coord,
    y: Coord,
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pair").field(&self.x).field(&self.y).finish()
    }
}

pub fn do_solve(machines: &[crate::part1::ClawMachine]) -> Cost {
    machines
        .into_par_iter()
        .map(ClawMachine::from)
        .filter_map(|machine| {
            let a = (machine.prize.y as f64 - machine.b.y as f64 * machine.prize.x as f64 / machine.b.x as f64)
                / (machine.a.y as f64 - machine.b.y as f64 * machine.a.x as f64 / machine.b.x as f64);
            let b = (machine.prize.x as f64 - machine.a.x as f64 * a) / machine.b.x as f64;
            let cost = A_COST * a + B_COST * b;
            let cost_rounded = cost.round();
            let cost_is_whole = (cost_rounded - cost).abs() < 1e-3;
            cost_is_whole.then(|| cost_rounded as Cost)
        })
        .sum()
}
