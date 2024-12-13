use rayon::prelude::*;

type Coord = u16;

const A_COST: i32 = 3;
const B_COST: i32 = 1;

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

fn whole_div(num: i32, den: i32) -> Option<i32> {
    (num % den == 0).then(|| num / den)
}

pub fn do_solve(machines: &[ClawMachine]) -> i32 {
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
            let a = whole_div(
                machine.prize.y as i32 * machine.b.x as i32 - machine.b.y as i32 * machine.prize.x as i32,
                machine.a.y as i32 * machine.b.x as i32 - machine.b.y as i32 * machine.a.x as i32,
            )?;
            let b = whole_div(
                machine.prize.x as i32 - machine.a.x as i32 * a as i32,
                machine.b.x as i32,
            )
            .unwrap();
            let cost = A_COST * a + B_COST * b;
            Some(cost)
        })
        .sum()
}
