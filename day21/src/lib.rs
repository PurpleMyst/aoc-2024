// This was a very interesting problem, where I managed to basically do 90% by myself, i.e.
// 1) figure out there was a greedy optimal move order;
// 2) figure out we could "memoize" based on chunks;
// 3) figure out we could memoize the keypad position.
// all great, but i wasn't able (rather, didn't want to spend too much time on it, as this already took me a full day)
// to figure out the correct greedy optimal move order, which i got from this solution:
// https://www.reddit.com/r/adventofcode/comments/1hjgyps/2024_day_21_part_2_i_got_greedyish/m36nr30/

use std::fmt::Display;

use memoize::memoize;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

const NUMERIC_KEYPAD: &[&[u8]] = &[b"789", b"456", b"123", b" 0A"];
const DIRECTIONAL_KEYPAD: &[&[u8]] = &[b" ^A", b"<v>"];

type Keypad = &'static [&'static [u8]];

type Chunk = Vec<u8>;
type ChunkBag = FxHashMap<Chunk, usize>;

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn find_keypad_position(keypad: &'static [&'static [u8]], key: u8) -> (u8, u8) {
    for (y, row) in keypad.iter().enumerate() {
        if let Some(x) = row.iter().position(|&k| k == key) {
            return (y as u8, x as u8);
        }
    }
    unreachable!("could not find {:?} in keypad", key as char);
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn calculate_move(keypad: Keypad, initial_position: (u8, u8), target_position: (u8, u8)) -> Vec<u8> {
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

    let first_vert =
        (delta_x != 0 && keypad[target_position.0 as usize][initial_position.1 as usize] != b' ').then(|| {
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

    match (first_vert, first_horiz) {
        (Some(vert), Some(horiz)) => {
            if b_horiz == b'<' {
                horiz
            } else {
                vert
            }
        }
        (Some(vert), None) => vert,
        (None, Some(horiz)) => horiz,
        (None, None) => unreachable!(),
    }
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn transform(keypad: Keypad, code: Vec<u8>) -> Vec<u8> {
    let mut prev = b'A';
    let mut moves = Vec::new();

    for key in code {
        let move_ = calculate_move(
            keypad,
            find_keypad_position(keypad, prev),
            find_keypad_position(keypad, key),
        );
        moves.extend(move_);
        prev = key;
    }

    moves
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // for benchmarking
    memoized_flush_transform();
    memoized_flush_calculate_move();
    memoized_flush_find_keypad_position();

    rayon::join(|| do_solve(2), || do_solve(25))
}

fn do_solve(k: usize) -> usize {
    include_str!("input.txt")
        .par_lines()
        .map(|line| complexity(line.as_bytes(), k))
        .sum::<usize>()
}

fn complexity(code: &[u8], k: usize) -> usize {
    let first = transform(NUMERIC_KEYPAD, code.to_vec());

    let mut chunks = ChunkBag::default();
    for chunk in first.split(|&b| b == b'A') {
        *chunks.entry(chunk.to_vec()).or_default() += 1;
    }

    let mut new_chunks = ChunkBag::default();
    for _ in 0..k {
        new_chunks.clear();
        for (mut chunk, count) in chunks.drain() {
            chunk.push(b'A');
            transform(DIRECTIONAL_KEYPAD, chunk)
                .split(|&b| b == b'A')
                .rev()
                .skip(1)
                .for_each(|chunk| {
                    *new_chunks.entry(chunk.to_vec()).or_default() += count;
                });
        }
        std::mem::swap(&mut chunks, &mut new_chunks);
    }

    let len = chunks
        .into_iter()
        .map(|(chunk, count)| count * (1 + chunk.len()))
        .sum::<usize>()
        - 1;
    let numeric_part = code
        .iter()
        .filter(|&&b| matches!(b, b'0'..=b'9'))
        .fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);

    len * numeric_part
}
