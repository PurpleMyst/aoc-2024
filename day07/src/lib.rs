use std::{fmt::Display, mem::swap};

use rayon::prelude::*;

fn concat(a: u64, ob: u64) -> u64 {
    let mut b = ob;
    let mut c = 1;
    while b > 0 {
        c *= 10;
        b /= 10;
    }
    a * c + ob
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    rayon::join(
        || do_solve(input, |state, v| [state + v, state * v]),
        || do_solve(input, |state, v| [state + v, state * v, concat(state, v)]),
    )
}

fn do_solve<F, I>(input: &str, f: F) -> u64
where
    F: Fn(u64, u64) -> I + Sync,
    I: IntoIterator<Item = u64>,
{
    input
        .par_lines()
        .filter_map(|line| {
            let (target, values) = line.split_once(": ").unwrap();
            let target = target.parse::<u64>().unwrap();
            let mut values = values.split(' ').map(|v| v.parse::<u64>().unwrap());

            let mut states = vec![values.next().unwrap()];
            let mut next_states = Vec::new();

            for v in values {
                next_states.clear();
                next_states.extend(states.drain(..).flat_map(|state| f(state, v)));
                swap(&mut states, &mut next_states);
            }

            states.into_iter().any(|s| s == target).then_some(target)
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 345), 12345);
    }
}
