use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let re2 = regex::Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut enabled = true;

    for cap in re2.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a: u32 = cap[1].parse().unwrap();
                let b: u32 = cap[2].parse().unwrap();
                let c = a * b;
                p1 += c;
                if enabled {
                    p2 += c;
                }
            }
        }
    }

    (p1, p2)
}
