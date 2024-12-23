use std::{collections::{HashMap, HashSet}, fmt::Display};

use fixedbitset::FixedBitSet;

use petgraph::{algo::tarjan_scc, prelude::*};

fn to_index(node: &str) -> u16 {
    u16::from_ne_bytes(<[u8; 2]>::try_from(node.as_bytes()).unwrap())
}

fn bron_kerbosch(
    g: &HashMap<&'static str, Vec<&'static str>>,
    r: Vec<&'static str>,
    mut p: Vec<&'static str>,
    mut x: Vec<&'static str>,
) -> Vec<Vec<&'static str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut result = Vec::new();

    while let Some(v) = p.pop() {
        {
        let mut r = r.clone();
        r.push(v);

        let p = p.iter().filter(|&&n| g[v].contains(&n)).copied().collect();
        let x = x.iter().filter(|&&n| g[v].contains(&n)).copied().collect();

        result.extend(bron_kerbosch(g,r,p,x));
        }

        x.push(v);
    }

    result
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut adjacency = HashMap::<&str, Vec<&str>>::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        adjacency.entry(a).or_default().push(b);
        adjacency.entry(b).or_default().push(a);
    });

    let mut p1 = 0;
    let mut considered = FixedBitSet::with_capacity(1 << 16);

    for (node, neighbors) in adjacency.iter() {
        if !node.starts_with('t') {
            continue;
        }
        considered.insert(to_index(node).into());

        for (i, alice) in neighbors.iter().enumerate() {
            if considered.contains(to_index(alice).into()) {
                continue;
            }

            for bob in neighbors.iter().skip(i + 1) {
                if considered.contains(to_index(bob).into()) {
                    continue;
                }

                if adjacency[alice].contains(bob) {
                    p1 += 1;
                }
            }
        }
    }

    let cliques = bron_kerbosch(&adjacency, Vec::new(), adjacency.keys().copied().collect(), Vec::new());
    let mut p2_nodes = cliques.into_iter().max_by_key(|v| v.len()).unwrap();
    p2_nodes.sort_unstable();
    let p2 = p2_nodes.join(",");


    (p1, p2)
}
