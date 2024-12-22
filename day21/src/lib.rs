use std::fmt::Display;

use memoize::memoize;
use owo_colors::OwoColorize;

const NUMERIC_KEYPAD: &[&[u8]] = &[b"789", b"456", b"123", b" 0A"];
const NUMERIC_INITIAL_POSITION: (u8, u8) = (3, 2);

const DIRECTIONAL_KEYPAD: &[&[u8]] = &[b" ^A", b"<v>"];
const DIRECTIONAL_INITIAL_POSITION: (u8, u8) = (0, 2);

fn manhattan_distance((y1, x1): (u8, u8), (y2, x2): (u8, u8)) -> usize {
    y1.abs_diff(y2) as usize + x1.abs_diff(x2) as usize
}

fn comp_diff((y1, x1): (u8, u8), (y2, x2): (u8, u8)) -> (i8, i8) {
    ((y1 as i8 - y2 as i8).abs(), (x1 as i8 - x2 as i8).abs())
}

/// Return the sequence of moves on the directional keypad to move from the initial position to the
/// target position.
#[memoize]
fn transform_step(
    keypad: &'static [&'static [u8]],
    initial_position: (u8, u8),
    target_position: (u8, u8),
) -> [Option<Vec<u8>>; 2] {
    use std::iter::{once, repeat_n};

    let delta_y = target_position.0 as i8 - initial_position.0 as i8;
    let delta_x = target_position.1 as i8 - initial_position.1 as i8;

    let b_vert = match delta_y.signum() {
        1 => b'v',
        -1 => b'^',
        0 => b'\0',
        _ => unreachable!(),
    };
    let b_horiz = match delta_x.signum() {
        1 => b'>',
        -1 => b'<',
        0 => b'\0',
        _ => unreachable!(),
    };

    let first_vert = (keypad[target_position.0 as usize][initial_position.1 as usize] != b' ').then(|| {
        repeat_n(b_vert, delta_y.abs() as usize)
            .chain(repeat_n(b_horiz, delta_x.abs() as usize))
            .chain(once(b'A'))
            .collect()
    });

    let first_horiz = (keypad[initial_position.0 as usize][target_position.1 as usize] != b' ').then(|| {
        repeat_n(b_horiz, delta_x.abs() as usize)
            .chain(repeat_n(b_vert, delta_y.abs() as usize))
            .chain(once(b'A'))
            .collect()
    });

    [first_vert, first_horiz]
}

#[memoize]
fn transform(
    keypads: &'static [&'static [&'static [u8]]],
    initial_position: (u8, u8),
    mut moves: Vec<u8>,
    moves_so_far: Vec<u8>,
) -> Vec<u8> {
    let keypad = keypads[0];

    if moves.is_empty() {
        if keypads.len() == 1 {
            return moves_so_far;
        } else {
            return transform(&keypads[1..], DIRECTIONAL_INITIAL_POSITION, moves_so_far, Vec::new());
        }
    };
    let cur_move = moves.remove(0);
    let rest = moves;

    let target_position = find_keypad_position(keypad, cur_move);
    match transform_step(keypad, initial_position, target_position) {
        [Some(first_vert), None] => {
            let mut moves_so_far = moves_so_far;
            moves_so_far.extend(first_vert);
            transform(keypads, target_position, rest, moves_so_far)
        }
        [None, Some(first_horiz)] => {
            let mut moves_so_far = moves_so_far;
            moves_so_far.extend(first_horiz);
            transform(keypads, target_position, rest, moves_so_far)
        }

        [Some(first_vert), Some(first_horiz)] => {
            // continue both branches and take the shortest
            let mut moves_so_far_vert = moves_so_far.clone();
            moves_so_far_vert.extend(first_vert);

            let mut moves_so_far_horiz = moves_so_far;
            moves_so_far_horiz.extend(first_horiz);

            let (vert, horiz) = rayon::join(
                || transform(keypads, target_position, rest.clone(), moves_so_far_vert),
                || transform(keypads, target_position, rest.clone(), moves_so_far_horiz),
            );

            std::cmp::min_by_key(vert, horiz, Vec::len)
        }

        [None, None] => panic!(),
    }

    // *moves = moves
    //     .into_iter()
    //     .scan(DIRECTIONAL_INITIAL_POSITION, |state, &mut button| {
    //         let target_position = find_keypad_position(DIRECTIONAL_KEYPAD, button);
    //         let moves = transform(DIRECTIONAL_KEYPAD, *state, target_position);
    //         assert!(verify(DIRECTIONAL_KEYPAD, &moves, *state));
    //         *state = target_position;
    //         Some(moves)
    //     })
    //     .flatten()
    //     .collect();
}

/// Check we don't hit a space
fn verify<const N: usize, const M: usize>(keypad: [[u8; N]; M], moves: &[u8], initial_position: (u8, u8)) -> bool {
    let mut position = initial_position;
    for &button in moves {
        match button {
            b'>' => position.1 += 1,
            b'<' => position.1 -= 1,
            b'^' => position.0 -= 1,
            b'v' => position.0 += 1,
            b'A' => {}
            _ => unreachable!(),
        }
        if keypad[position.0 as usize][position.1 as usize] == b' ' {
            return false;
        }
    }
    true
}

fn find_keypad_position(keypad: &[&[u8]], key: u8) -> (u8, u8) {
    for (y, row) in keypad.iter().enumerate() {
        if let Some(x) = row.iter().position(|&k| k == key) {
            return (y as u8, x as u8);
        }
    }
    unreachable!("could not find {:?} in keypad", key as char);
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let p1 = include_str!("input.txt")
        .lines()
        .map(|line| dbg!(complexity(dbg!(line).as_bytes())))
        .sum::<usize>();

    (p1, "TODO")
}

fn complexity(code: &[u8]) -> usize {
    let moves = transform(
        &[NUMERIC_KEYPAD, DIRECTIONAL_KEYPAD, DIRECTIONAL_KEYPAD],
        NUMERIC_INITIAL_POSITION,
        code.to_vec(),
        Vec::new(),
    );
    println!("{}", moves.len());

    let numeric_part = code
        .iter()
        .filter(|&&b| matches!(b, b'0'..=b'9'))
        .map(|&b| b as char)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    moves.len() * numeric_part
}
