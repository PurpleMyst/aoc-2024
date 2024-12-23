use std::fmt::Display;

use petgraph::prelude::*;
use rayon::prelude::*;

type Graph = UnGraph<&'static str, ()>;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let graph = UnGraphMap::<&str, ()>::from_edges(input.lines().map(|line| line.split_once('-').unwrap()));
    let graph = graph.into_graph();

    rayon::join(|| solve_part1(&graph), || solve_part2(&graph))
}

fn triangles(graph: &Graph) -> impl Iterator<Item = [NodeIndex; 3]> + '_ {
    graph
        .node_indices()
        .flat_map(move |node| triangles_starting_from(graph, node))
}

fn triangles_starting_from(graph: &Graph, node: NodeIndex) -> impl Iterator<Item = [NodeIndex; 3]> + '_ {
    graph.neighbors(node).enumerate().flat_map(move |(i, alice)| {
        graph
            .neighbors(node)
            .skip(i + 1)
            .filter_map(move |bob| graph.contains_edge(alice, bob).then_some([node, alice, bob]))
    })
}

fn solve_part1(graph: &Graph) -> usize {
    triangles(graph)
        .filter(|nodes| {
            nodes
                .into_iter()
                .any(|node| graph.node_weight(*node).unwrap().starts_with("t"))
        })
        .count()
        / 3
}

// https://observablehq.com/@jwolondon/advent-of-code-2024-day-23
fn solve_part2(graph: &Graph) -> String {
    let counts = (0..graph.node_count())
        .into_par_iter()
        .map(|node_idx| {
            triangles_starting_from(graph, NodeIndex::new(node_idx)).count() as u8
        })
        .collect::<Vec<_>>();

    let max_size = counts.iter().max().unwrap();
    let mut p2_nodes = graph
        .node_indices()
        .filter(|node| counts[node.index()] == *max_size)
        .map(|node| *graph.node_weight(node).unwrap())
        .collect::<Vec<_>>();
    p2_nodes.sort_unstable();
    p2_nodes.join(",")
}
