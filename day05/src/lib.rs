use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (order, updates) = input.split_once("\n\n").unwrap();
    let order = order
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse::<u8>().unwrap(), after.parse::<u8>().unwrap())
        })
        .collect::<Vec<_>>();

    let updates = updates
        .lines()
        .map(|update| update.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>());

    let p1 = updates
        .clone()
        .filter_map(|update| -> Option<u8> { is_ordered(&update, &order).then(|| update[update.len() / 2]) })
        .map(|n| n as u16)
        .sum::<u16>();

    let p2 = updates
        .filter_map(|mut update| -> Option<u8> {
            if is_ordered(&update, &order) {
                return None;
            }

            let mut ordered = Vec::new();

            for _ in 0..update.len() {
                let next = update
                    .iter()
                    .copied()
                    .position(|n| !update.iter().copied().any(|m| order.contains(&(m, n))))
                    .expect("circular dependency");
                ordered.push(update.swap_remove(next))
            }

            Some(ordered[ordered.len() / 2])
        })
        .map(|n| n as u16)
        .sum::<u16>();

    (p1, p2)
}

fn is_ordered(update: &[u8], order: &[(u8, u8)]) -> bool {
    update
        .iter()
        .copied()
        .enumerate()
        .all(|(i, n)| !update[..i].iter().copied().any(|m| order.contains(&(n, m))))
}
