use std::fmt::Display;

struct Order {
    map: [u128; 100],
}

impl Order {
    fn new() -> Self {
        Self { map: [0; 100] }
    }

    fn add(&mut self, before: u8, after: u8) {
        debug_assert!(before < 100);
        debug_assert!(after < 100);
        self.map[before as usize] |= 1 << after;
    }

    fn contains(&self, before: u8, after: u8) -> bool {
        debug_assert!(before < 100);
        debug_assert!(after < 100);
        self.map[before as usize] & (1 << after) != 0
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (order_s, updates) = input.split_once("\n\n").unwrap();

    let mut order = Order::new();
    order_s
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
        })
        .for_each(|(before, after)| order.add(before, after));

    let updates = updates
        .lines()
        .map(|update| update.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>());

    let mut p1 = 0u16;
    let mut p2 = 0u16;

    for update in updates {
        let midpoint = update.len() / 2;
        if update.is_sorted_by(|&n, &m| order.contains(n, m)) {
            p1 += update[midpoint] as u16;
        } else {
            p2 += update
                .iter()
                .copied()
                .find(|&n| update.iter().filter(|&&m| order.contains(m, n)).count() == midpoint)
                .unwrap() as u16;
        }
    }

    (p1, p2)
}
