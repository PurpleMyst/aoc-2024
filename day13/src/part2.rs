use rayon::prelude::*;

const A_COST: f64 = 3.0;
const B_COST: f64 = 1.0;

const OFFSET: f64 = 10_000_000_000_000.0;

pub fn do_solve(machines: &[crate::part1::ClawMachine]) -> u64 {
    machines
        .into_par_iter()
        .filter_map(|machine| {
            let a = ((OFFSET + machine.prize.y as f64)
                - machine.b.y as f64 * (OFFSET + machine.prize.x as f64) / machine.b.x as f64)
                / (machine.a.y as f64 - machine.b.y as f64 * machine.a.x as f64 / machine.b.x as f64);
            let b = ((OFFSET + machine.prize.x as f64) - machine.a.x as f64 * a) / machine.b.x as f64;
            let cost = A_COST * a + B_COST * b;
            let cost_rounded = cost.round();
            let cost_is_whole = (cost_rounded - cost).abs() < 1e-3;
            cost_is_whole.then(|| cost_rounded as u64)
        })
        .sum()
}
