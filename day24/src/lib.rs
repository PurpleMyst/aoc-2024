use std::fmt::Display;

use std::collections::HashMap;

type Gates = HashMap<&'static str, Gate>;

enum Gate {
    Constant(bool),
    Xor(&'static str, &'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
}

impl Gate {
    #[must_use]
    fn has_operands(&self, a: &str, b: &str) -> bool {
        match self {
            &Gate::Xor(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
            &Gate::And(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
            &Gate::Or(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
            _ => false,
        }
    }

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
}

fn eval(gates: &Gates, name: &str) -> bool {
    match gates.get(name).unwrap() {
        Gate::Constant(val) => *val,
        Gate::Xor(lhs, rhs) => eval(gates, lhs) ^ eval(gates, rhs),
        Gate::And(lhs, rhs) => eval(gates, lhs) & eval(gates, rhs),
        Gate::Or(lhs, rhs) => eval(gates, lhs) | eval(gates, rhs),
    }
}

fn show(gates: &Gates, name: &str) -> String {
    match gates.get(name).unwrap() {
        Gate::Constant(..) => format!("{name}"),
        Gate::Xor(lhs, rhs) => format!("({} ^ {})", show(gates, lhs), show(gates, rhs)),
        Gate::And(lhs, rhs) => format!("({} & {})", show(gates, lhs), show(gates, rhs)),
        Gate::Or(lhs, rhs) => format!("({} | {})", show(gates, lhs), show(gates, rhs)),
    }
}

fn eval_num(gates: &Gates, prefix: char) -> u64 {
    let n = gates.keys().filter(|k| k.starts_with(prefix)).max().unwrap().strip_prefix(prefix).unwrap().parse::<usize>().unwrap();

    (0..=n)
        .rev()
        .map(|n| format!("{prefix}{n:02}"))
        .map(|k| eval(&gates, k.as_str()))
        .fold(0u64, |acc, x| (acc << 1) | if x { 1 } else { 0 })
}

fn expected_for_bit_build(gates: &Gates, n: u64) {
    let mut carry_in: Option<&str> = None;

    // let xor = |a: &str, b: &str| gates.iter().find(|v| match v.1 {
    //     &Gate::Xor(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
    //     _ => false,
    // }).expect("XOR gate not found").0;
    // let and = |a: &str, b: &str| gates.iter().find(|v| match v.1 {
    //     &Gate::And(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
    //     _ => false,
    // }).expect("AND gate not found").0;
    // let or = |a: &str, b: &str| gates.iter().find(|v| match v.1 {
    //     &Gate::Or(lhs, rhs) => (lhs, rhs) == (a, b) || (lhs, rhs) == (b, a),
    //     _ => false,
    // }).expect("OR gate not found").0;

    let xor = |a: &str, b: &str| gates.iter().find(|(_, v)| v.is_xor() && v.has_operands(a, b)).expect(format!("XOR gate not found: {} ^ {}", a, b).as_str()).0;
    let and = |a: &str, b: &str| gates.iter().find(|(_, v)| v.is_and() && v.has_operands(a, b)).expect(format!("AND gate not found: {} & {}", a, b).as_str()).0;
    let or = |a: &str, b: &str| gates.iter().find(|(_, v)| v.is_or() && v.has_operands(a, b)).expect(format!("OR gate not found: {} | {}", a, b).as_str()).0;

    // let x = |i| format!("x{i:02}");
    // let y = |i| format!("y{i:02}");
    fn x(i: u64) -> String {
        format!("x{i:02}")
    }
    fn y(i: u64) -> String {
        format!("y{i:02}")
    }

    for i in 0..=n {
        let bit = 
            if let Some(ref carry) = carry_in {
                xor(dbg!(xor(&x(i), &y(i))), carry)
            } else {
                xor(&x(i), &y(i))
            };
        if !bit.starts_with("z") {
            panic!("Unexpected bit name {bit:?}, should be z{i:02}");
        }

        let carry = 
            if let Some(ref carry_in) = carry_in {
                or(dbg!(and(&x(i), &y(i))), dbg!(and(carry_in, dbg!(xor(&x(i), &y(i))))))
            } else {
                and(&x(i), &y(i))
            };

        println!("z{i:02} = {}", bit);
        println!("c{i:02} = {}", carry);
        println!();

        carry_in = Some(carry);
    }
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
            gates.insert(dst, match op {
                "AND" => Gate::And(lhs, rhs),
                "OR" => Gate::Or(lhs, rhs),
                "XOR" => Gate::Xor(lhs, rhs),
                _ => panic!("Unknown operator: {}", op),
            });
        }
    });

    let p1 = eval_num(&gates, 'z');

    let x = eval_num(&gates, 'x');
    let y = eval_num(&gates, 'y');

    println!("{x:046b} + {y:046b} = {:048b}", p1 ^ (x + y));

    dbg!(eval_num(&gates, 'x'));
    dbg!(eval_num(&gates, 'y'));

    expected_for_bit_build(&gates, 44);

    for n in 0..=45 {
        println!("z{n:02} = {}", show(&gates, format!("z{n:02}").as_str()));
        println!();
    }

    (p1, "TODO")
}
