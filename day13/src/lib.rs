use std::fmt::Display;

mod part1;
mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let machines = part1::load_input(input);

    let p1 = part1::do_solve(&machines);
    let p2 = part2::do_solve(&machines);

    (p1, p2)
}
