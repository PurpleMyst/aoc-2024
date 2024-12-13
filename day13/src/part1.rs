use rayon::prelude::*;

type Coord = u16;
type Cost = u16;

const A_COST: f64 = 3.0;
const B_COST: f64 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct ClawMachine {
    pub a: Pair,
    pub b: Pair,
    pub prize: Pair,
}

fn parse_one(c: u8, mut s: impl Iterator<Item = u8>) -> Coord {
    let _ = s.find(|&x| x == c).unwrap();
    s.take_while(|&x| x.is_ascii_digit())
        .fold(0, |acc, x| acc * 10 + (x - b'0') as Coord)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    pub x: Coord,
    pub y: Coord,
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pair").field(&self.x).field(&self.y).finish()
    }
}

pub fn load_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut it = chunk.lines();

            let mut bs = it.next().unwrap().bytes();
            let a_x = parse_one(b'+', bs.by_ref());
            let a_y = parse_one(b'+', bs.by_ref());
            let a = Pair { x: a_x, y: a_y };

            let mut bs = it.next().unwrap().bytes();
            let b_x = parse_one(b'+', bs.by_ref());
            let b_y = parse_one(b'+', bs.by_ref());
            let b = Pair { x: b_x, y: b_y };

            let mut bs = it.next().unwrap().bytes();
            let p_x = parse_one(b'=', bs.by_ref());
            let p_y = parse_one(b'=', bs.by_ref());
            let prize = Pair { x: p_x, y: p_y };

            ClawMachine { a, b, prize }
        })
        .collect()
}

pub fn do_solve(machines: &[ClawMachine]) -> Cost {
    // calling A and B the vectors of each button, and a and b the number of times we've pressed, we've got
    // p = aA + bB
    // and cost given by
    // c = 3a + b

    // p_x = a_x * a + b_x * b
    // p_y = a_y * a + b_y * b
    //
    // => b = (p_x - a_x * a) / b_x
    // => p_y = a_y * a + b_y * ((p_x - a_x * a) / b_x)
    // solving for a
    // p_y = (a_y - b_y * a_x / b_x) * a + b_y * p_x / b_x
    // a = (p_y - b_y * p_x / b_x) / (a_y - b_y * a_x / b_x)

    machines
        .into_par_iter()
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
