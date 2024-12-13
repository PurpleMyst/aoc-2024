use rayon::prelude::*;

const A_COST: i64 = 3;
const B_COST: i64 = 1;

const OFFSET: i64 = 10_000_000_000_000;

fn whole_div(num: i64, den: i64) -> Option<i64> {
    (num % den == 0).then(|| num / den)
}

pub fn do_solve(machines: &[crate::part1::ClawMachine]) -> i64 {
    machines
        .into_par_iter()
        .filter_map(|machine| {
            let a = whole_div(
                (OFFSET + machine.prize.y as i64) * machine.b.x as i64
                    - machine.b.y as i64 * (OFFSET + machine.prize.x as i64),
                machine.a.y as i64 * machine.b.x as i64 - machine.b.y as i64 * machine.a.x as i64,
            )?;
            let b = whole_div(
                (OFFSET + machine.prize.x as i64) - machine.a.x as i64 * a,
                machine.b.x as i64,
            )?;
            let cost = A_COST * a + B_COST * b;
            Some(cost)
        })
        .sum()
}
