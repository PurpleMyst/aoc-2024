use std::fmt::Display;

const DIRS: [(isize, isize); 8] = [
    // first four are horizontal
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    // next four are diagonal
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    do_solve(input)
}

fn do_solve(input: &str) -> (i32, i32) {
    let map = input.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();
    let side = input.bytes().position(|b| b == b'\n').unwrap();

    let idx = |y, x| usize::from(y) * side + usize::from(x);
    let coords = |idx| (idx / side, idx % side);

    let neighbors = |y: usize, x: usize| {
        [
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
            (y, x.wrapping_add(1)),
            (y.wrapping_add(1), x),
        ]
        .into_iter()
        .filter(|&(y, x)| y < side && x < side)
    };

    let mut regions = (0..side * side).collect::<Vec<_>>();
    loop {
        let mut changed = false;
        'a: for (y, x) in (0..side * side).map(coords) {
            for (ny, nx) in neighbors(y, x) {
                if map[idx(y, x)] == map[idx(ny, nx)] && regions[idx(ny, nx)] != regions[idx(y, x)] {
                    let common = std::cmp::min(regions[idx(y, x)], regions[idx(ny, nx)]);
                    regions[idx(y, x)] = common;
                    regions[idx(ny, nx)] = common;
                    changed = true;
                    break 'a;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let mut distinct_regions = regions.clone();
    distinct_regions.sort();
    distinct_regions.dedup();

    let mut p1 = 0;
    let mut p2 = 0;

    for region in distinct_regions {
        let mut area = 0;
        let mut perimeter = 0;

        let start_idx = (0..side * side).find(|&idx| regions[idx] == region).unwrap();

        let mut stack = vec![start_idx];
        let mut seen = vec![false; side * side];

        while let Some(cur_idx) = stack.pop() {
            let (y, x) = coords(cur_idx);
            if regions[cur_idx] != region {
                continue;
            }
            if seen[cur_idx] {
                continue;
            }
            seen[cur_idx] = true;

            area += 1;

            perimeter += 4;
            for (ny, nx) in neighbors(y, x) {
                let nidx = idx(ny, nx);

                if regions[nidx] == region {
                    stack.push(nidx);
                    perimeter -= 1;
                }
            }
        }

        let mut region_min_y = usize::MAX;
        let mut region_max_y = 0;
        let mut region_min_x = usize::MAX;
        let mut region_max_x = 0;
        for (y, x) in (0..side).flat_map(|y| (0..side).map(move |x| (y, x))) {
            if regions[idx(y, x)] == region {
                region_min_y = region_min_y.min(y);
                region_max_y = region_max_y.max(y);
                region_min_x = region_min_x.min(x);
                region_max_x = region_max_x.max(x);
            }
        }
        let mut sides = 0;

        let mut bitset = vec![0u64; (1 + side * 2) * (1 + side * 2)];
        for y in 0..side {
            for x in 0..side {
                if regions[idx(y, x)] != region {
                    continue;
                }
                for (i, (dy, dx)) in DIRS.iter().copied().enumerate() {
                    let ny = 1 + (2 * y as isize + dy);
                    let nx = 1 + (2 * x as isize + dx);
                    assert!((0..=side as isize * 2).contains(&ny));
                    assert!((0..=side as isize * 2).contains(&nx));

                    bitset[ny as usize * (1 + side * 2) + nx as usize] |= 1 << i;
                }
            }
        }

        for y in 2 * (region_min_y as isize)..=2 * (region_max_y as isize + 1) {
            for x in 2 * (region_min_x as isize)..=2 * (region_max_x as isize + 1) {
                let y = y as usize;
                let x = x as usize;

                if y % 2 == 1 && x % 2 == 1 {
                } else {
                    // this is a boundary point
                    let flags = bitset[y * (1 + side * 2) + x];
                    if flags & 0xf0 != 0 && flags & 0x0f == 0 && (flags & 0xf0).count_ones() % 2 == 1 {
                        sides += 1;
                    } else if flags & 0b1001_0000 == 0b1001_0000 && flags & 0b0110_0000 == 0 && flags & 0x0f == 0 {
                        sides += 2;
                    } else if flags & 0b0110_0000 == 0b0110_0000 && flags & 0b1001_0000 == 0 && flags & 0x0f == 0 {
                        sides += 2;
                    }
                }
            }
        }

        p1 += area * perimeter;
        p2 += area * sides;
    }

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let examples = [
            (include_str!("sample_input.txt"), 80),
            (include_str!("sample_input2.txt"), 1206),
            (include_str!("sample_input3.txt"), 436),
            (include_str!("sample_input4.txt"), 236),
            (include_str!("sample_input5.txt"), 368),
        ];

        for (input, expected) in &examples {
            assert_eq!(do_solve(input).1, *expected);
        }
    }
}
