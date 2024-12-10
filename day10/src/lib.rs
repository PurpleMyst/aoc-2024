use std::fmt::Display;

const START: u8 = 0;
const END: u8 = 9;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct State {
    pos: (usize, usize),
}

impl State {
    fn advance<'a>(&'a self, grid: &'a grid::Grid<u8>) -> impl Iterator<Item = Self> + 'a {
        let (y, x) = self.pos;
        let &cell = grid.get(y, x).unwrap();

        DIRS.into_iter().filter_map(move |(dy, dx)| {
            let new_y = y.checked_add_signed(dy)?;
            let new_x = x.checked_add_signed(dx)?;
            let new_pos = (new_y, new_x);
            let &new_cell = grid.get(new_y, new_x)?;
            (new_cell == cell + 1).then(|| State { pos: new_pos })
        })
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let side = input.lines().count();
    let grid = grid::Grid::from_vec(input.bytes().filter(|&b| b != b'\n').map(|b| b - b'0').collect(), side);

    let mut p2 = 0;
    let p1 = grid
        .indexed_iter()
        .filter(|(_, &c)| c == START)
        .map(|(pos, _)| State { pos })
        .map(|state| {
            let mut states = vec![state];
            let mut reachable = grid::Grid::new(side, side);
            reachable.fill(false);

            while let Some(state) = states.pop() {
                if let Some(&cell) = grid.get(state.pos.0, state.pos.1) {
                    if cell == END {
                        reachable[state.pos] = true;
                        p2 += 1;
                    } else {
                        states.extend(state.advance(&grid));
                    }
                }
            }

            reachable.iter().filter(|&&b| b).count()
        })
        .sum::<usize>();

    (p1, p2)
}
