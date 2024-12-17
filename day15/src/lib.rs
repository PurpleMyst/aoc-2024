use std::fmt::Display;

const UP: u8 = b'^';
const DOWN: u8 = b'v';
const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';

const ROBOT: u8 = b'@';
const WALL: u8 = b'#';
const BOX: u8 = b'O';
const EMPTY: u8 = b'.';

const WIDE_BOX_LEFT: u8 = b'[';
const WIDE_BOX_RIGHT: u8 = b']';

fn do_move(
    map: &mut [u8],
    (y, x): (usize, usize),
    width: usize,
    height: usize,
    (dy, dx): (isize, isize),
) -> Option<(usize, usize)> {
    let cur_idx = y * width + x;

    let next_y = y.wrapping_add_signed(dy);
    let next_x = x.wrapping_add_signed(dx);

    let next_idx = next_y * width + next_x;

    match map[next_idx] {
        EMPTY => {
            map[next_idx] = map[cur_idx];
            map[cur_idx] = EMPTY;
            Some((next_y, next_x))
        }

        WALL => None,

        BOX => {
            if do_move(map, (next_y, next_x), width, height, (dy, dx)).is_some() {
                map[next_idx] = map[cur_idx];
                map[cur_idx] = EMPTY;
                Some((next_y, next_x))
            } else {
                None
            }
        }

        WIDE_BOX_LEFT | WIDE_BOX_RIGHT if dy == 0 => {
            if do_move(map, (next_y, next_x), width, height, (dy, dx)).is_some() {
                map[next_idx] = map[cur_idx];
                map[cur_idx] = EMPTY;
                Some((next_y, next_x))
            } else {
                None
            }
        }

        WIDE_BOX_LEFT | WIDE_BOX_RIGHT => {
            let partner_dx = if map[next_idx] == WIDE_BOX_LEFT { 1 } else { -1 };
            let old_map = map.to_owned();

            // both the halves have to move in the vertical dir
            if do_move(map, (next_y, next_x), width, height, (dy, dx)).is_some() {
                if do_move(
                    map,
                    (next_y, next_x.wrapping_add_signed(partner_dx)),
                    width,
                    height,
                    (dy, dx),
                )
                .is_some()
                {
                    map[next_idx] = map[cur_idx];
                    map[cur_idx] = EMPTY;
                    Some((next_y, next_x))
                } else {
                    map.copy_from_slice(&old_map);
                    None
                }
            } else {
                None
            }
        }

        _ => unreachable!("trying to move to {:?} with dir {:?}", map[next_idx] as char, (dy, dx)),
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let (map, moves) = input.split_once("\n\n").unwrap();

    let width = map.lines().next().unwrap().len();
    let height = width;

    let map = map.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();

    rayon::join(
        || do_solve::<false>(map.clone(), width, moves, height),
        || do_solve::<true>(map.clone(), width, moves, height),
    )
}

fn do_solve<const PART2: bool>(mut map: Vec<u8>, mut width: usize, moves: &str, height: usize) -> usize {
    if PART2 {
        map = map
            .into_iter()
            .flat_map(|b| match b {
                ROBOT => [ROBOT, EMPTY],
                WALL => [WALL, WALL],
                BOX => [WIDE_BOX_LEFT, WIDE_BOX_RIGHT],
                EMPTY => [EMPTY, EMPTY],
                _ => unreachable!(),
            })
            .collect();
        width *= 2;
    }

    let moves = moves.bytes().filter(|&b| b != b'\n');

    let robot_idx = map.iter().position(|&b| b == ROBOT).unwrap();
    let (mut robot_y, mut robot_x) = (robot_idx / width, robot_idx % width);

    for m in moves {
        let dir = match m {
            UP => (-1, 0),
            DOWN => (1, 0),
            LEFT => (0, -1),
            RIGHT => (0, 1),
            _ => unreachable!(),
        };

        if let Some((y, x)) = do_move(&mut map, (robot_y, robot_x), width, height, dir) {
            robot_y = y;
            robot_x = x;
        }
    }

    map.into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            let (y, x) = (i / width, i % width);
            matches!(b, BOX | WIDE_BOX_LEFT).then(|| y * 100 + x)
        })
        .sum()
}
