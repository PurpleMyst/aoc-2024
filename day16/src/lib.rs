use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: (u8, u8),
    dir: (i8, i8),
}

impl State {
    fn advance(self, map: &[u8], side: usize) -> impl Iterator<Item = (Self, usize)> {
        [
            {
                let next_y = self.pos.0.wrapping_add_signed(self.dir.0);
                let next_x = self.pos.1.wrapping_add_signed(self.dir.1);
                (map[usize::from(next_y) * usize::from(side) + usize::from(next_x)] != b'#').then(|| {
                    (
                        State {
                            pos: (next_y, next_x),
                            dir: self.dir,
                        },
                        1,
                    )
                })
            },
            Some((
                State {
                    pos: self.pos,
                    dir: (-self.dir.1, self.dir.0),
                },
                1000,
            )),
            Some((
                State {
                    pos: self.pos,
                    dir: (self.dir.1, -self.dir.0),
                },
                1000,
            )),
        ]
        .into_iter()
        .flatten()
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let side = input.lines().next().unwrap().len();

    let map = input.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();

    let mut start = usize::MAX;
    let mut end = usize::MAX;

    for (i, b) in map.iter().enumerate() {
        match b {
            b'S' => start = i,
            b'E' => end = i,
            _ => {}
        }
    }
    let (end_y, end_x) = ((end / side) as u8, (end % side) as u8);

    let (paths, p1) = pathfinding::directed::astar::astar_bag(
        &State {
            pos: ((start / side) as u8, (start % side) as u8),
            dir: (0, 1),
        },
        |s| s.advance(&map, side),
        |s| usize::from(s.pos.0.abs_diff(end_y)) + usize::from(s.pos.1.abs_diff(end_x)),
        |s| s.pos == (end_y, end_x),
    )
    .unwrap();

    let mut bs = fixedbitset::FixedBitSet::with_capacity(map.len());
    for path in paths {
        for State { pos, .. } in path {
            bs.insert(usize::from(pos.0) * side + usize::from(pos.1));
        }
    }
    let p2 = bs.count_ones(..);

    (p1, p2)
}
