use std::fmt::Display;

use std::collections::HashMap;

const OUTPUT_BITS: usize = 45;

type Gates = HashMap<&'static str, Gate>;

#[derive(Clone, Copy)]
enum Gate {
    Constant(bool),
    Xor(&'static str, &'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
}

impl Gate {
    #[must_use]
    fn is_xor(&self) -> bool {
        matches!(self, Self::Xor(..))
    }

    #[must_use]
    fn is_and(&self) -> bool {
        matches!(self, Self::And(..))
    }

    #[must_use]
    fn is_or(&self) -> bool {
        matches!(self, Self::Or(..))
    }

    #[must_use]
    fn first_operand(&self) -> &str {
        match self {
            Self::Xor(lhs, _) | Self::And(lhs, _) | Self::Or(lhs, _) => lhs,
            _ => "",
        }
    }

    #[must_use]
    fn second_operand(&self) -> &str {
        match self {
            Self::Xor(_, rhs) | Self::And(_, rhs) | Self::Or(_, rhs) => rhs,
            _ => "",
        }
    }

    #[must_use]
    fn has_operand(&self, name: &str) -> bool {
        self.first_operand() == name || self.second_operand() == name
    }
}

fn eval(gates: &Gates, name: &str) -> bool {
    match gates.get(name).unwrap() {
        Gate::Constant(val) => *val,
        Gate::Xor(lhs, rhs) => eval(gates, lhs) ^ eval(gates, rhs),
        Gate::And(lhs, rhs) => eval(gates, lhs) & eval(gates, rhs),
        Gate::Or(lhs, rhs) => eval(gates, lhs) | eval(gates, rhs),
    }
}

fn eval_num(gates: &Gates, prefix: char) -> u64 {
    let n = gates
        .keys()
        .filter(|k| k.starts_with(prefix))
        .max()
        .unwrap()
        .strip_prefix(prefix)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    (0..=n)
        .rev()
        .map(|n| format!("{prefix}{n:02}"))
        .map(|k| eval(&gates, k.as_str()))
        .fold(0u64, |acc, x| (acc << 1) | if x { 1 } else { 0 })
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut gates = HashMap::new();
    include_str!("input.txt").lines().for_each(|line| {
        if line.contains(":") {
            let (dst, val) = line.split_once(": ").unwrap();
            gates.insert(dst, Gate::Constant(val == "1"));
        } else if line.contains(" -> ") {
            let (operands, dst) = line.split_once(" -> ").unwrap();
            let mut it = operands.split(' ');
            let lhs = it.next().unwrap();
            let op = it.next().unwrap();
            let rhs = it.next().unwrap();
            gates.insert(
                dst,
                match op {
                    "AND" => Gate::And(lhs, rhs),
                    "OR" => Gate::Or(lhs, rhs),
                    "XOR" => Gate::Xor(lhs, rhs),
                    _ => panic!("Unknown operator: {}", op),
                },
            );
        }
    });

    rayon::join(|| eval_num(&gates, 'z'), || find_swaps(&gates))
}

// Originally I solved this, as can be seen in commit abede62, by manually building the "expected" forms of the gates,
// and panick-ing if they couldn't be built (i.e. a XOR was missing, something that matched a zNN bit wasn't called zNN,
// et cetera), which worked to get a solution. This is u/lscddit's solution, which is much more elegant and general.
fn find_swaps(gates: &HashMap<&str, Gate>) -> String {
    let mut wrong = Vec::with_capacity(8);
    let is_xyz = |s: &str| s.starts_with('x') || s.starts_with('y') || s.starts_with('z');

    for (&dst, &gate) in gates {
        if dst.starts_with('z') && dst != format!("z{OUTPUT_BITS:02}") && !gate.is_xor() {
            wrong.push(dst);
        }

        if gate.is_xor() && !is_xyz(&dst) && !is_xyz(gate.first_operand()) && !is_xyz(gate.second_operand()) {
            wrong.push(dst);
        }

        if gate.is_and() && !gate.has_operand("x00") {
            for (_, sub_gate) in gates {
                if !sub_gate.is_or() && sub_gate.has_operand(dst) {
                    wrong.push(dst);
                    break;
                }
            }
        }

        if gate.is_xor() {
            for (_, sub_gate) in gates {
                if sub_gate.is_or() && sub_gate.has_operand(dst) {
                    wrong.push(dst);
                    break;
                }
            }
        }
    }

    wrong.sort_unstable();
    wrong.dedup();

    let p2 = wrong.join(",");
    p2
}
