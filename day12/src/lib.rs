use std::fmt::Display;

use itertools::iproduct;

// Directions for the eight neigihbors of a cell, arranged so that:
// * the lower four are the cardinal directions; 
// * the upper four are the diagonals.
const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
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

struct Bitset { // 256xN bits
    data: Vec<[u128; 2]>,
}

impl Bitset {
    fn new(side: usize) -> Self {
        Self {
            data: vec![[0, 0]; side],
        }
    }

    fn set(&mut self, y: usize, x: usize) {
        debug_assert!(y < self.data.len());
        debug_assert!(x < 256);
        let (i, j) = (x / 128, x % 128);
        self.data[y][i] |= 1 << j;
    }

    fn get(&self, y: usize, x: usize) -> bool {
        debug_assert!(y < self.data.len());
        debug_assert!(x < 256);
        let (i, j) = (x / 128, x % 128);
        self.data[y][i] & (1 << j) != 0
    }

    fn reset(&mut self) {
        for row in &mut self.data {
            row[0] = 0;
            row[1] = 0;
        }
    }
}

fn do_solve(input: &str) -> (usize, usize) {
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

    let mut p1 = 0;
    let mut p2 = 0;

    let mut visited = Bitset::new(side);

    let mut stack = vec![];
    let mut seen = Bitset::new(side);
    let mut neighbor_dirs = vec![0u8; (1 + side * 2) * (1 + side * 2)];

    // Each cell could be the seed of a region, so let's iterate over all of them.
    for (y, x) in iproduct!(0..side, 0..side) {
        // If this is already part of a region, skip it.
        if visited.get(y, x) {
            continue;
        }
        visited.set(y, x);

        // Area and perimeter of the current region, easy to calculate while constructing.
        let mut area = 0;
        let mut perimeter = 0;

        // The region's bounding box.
        let mut region_min_y = usize::MAX;
        let mut region_max_y = 0;
        let mut region_min_x = usize::MAX;
        let mut region_max_x = 0;

        // Start the DFS from this cell.
        let start_idx = idx(y, x);
        stack.push(start_idx);
        seen.reset();
        neighbor_dirs.fill(0);
        while let Some(cur_idx) = stack.pop() {
            let (y, x) = coords(cur_idx);
            if seen.get(y, x) {
                continue;
            }
            seen.set(y, x);

            region_min_y = region_min_y.min(y);
            region_max_y = region_max_y.max(y);
            region_min_x = region_min_x.min(x);
            region_max_x = region_max_x.max(x);

            // Each cell contributes 1 to the area and 4 to the perimeter.
            area += 1;
            perimeter += 4;
            for (ny, nx) in neighbors(y, x) {
                let nidx = idx(ny, nx);

                // Any neighbors that are part of the same region subtract 1 from the perimeter, and are added to the
                // stack.
                if map[cur_idx] == map[nidx] {
                    stack.push(nidx);
                    perimeter -= 1;
                }
            }

            // Update this cell's neighbors' to mark there is a neighbor in this direction.
            for (i, (dy, dx)) in DIRS.iter().copied().enumerate() {
                let ny = 1 + (2 * y as isize + dy);
                let nx = 1 + (2 * x as isize + dx);
                neighbor_dirs[ny as usize * (1 + side * 2) + nx as usize] |= 1 << i;
            }
        }

        // Sides'll correspond to the number of corners in the region.
        let mut sides = 0;

        // For each cell in the region...
        for y in 2 * (region_min_y as isize)..=2 * (region_max_y as isize + 1) {
            for x in 2 * (region_min_x as isize)..=2 * (region_max_x as isize + 1) {
                let y = y as usize;
                let x = x as usize;

                if y % 2 == 0 && x % 2 == 0 {
                    let flags = neighbor_dirs[y * (1 + side * 2) + x];

                    // If this has anything in the cardinal directions, it's not a corner.
                    if flags & 0x0f != 0 {
                        continue;
                    }

                    if (flags & 0xf0).count_ones() % 2 == 1 {
                        // Simple corner, with one or three neighbors.
                        sides += 1;
                    } else if flags & 0b1001_0000 == 0b1001_0000 && flags & 0b0110_0000 == 0 {
                        // Double corner!
                        sides += 2;
                    } else if flags & 0b0110_0000 == 0b0110_0000 && flags & 0b1001_0000 == 0 {
                        // Double corner, but mirrored!
                        sides += 2;
                    }
                }
            }
        }

        p1 += area * perimeter;
        p2 += area * sides;

        // Update the visited bitset within the region's bounding box.
        for y in region_min_y..=region_max_y {
            for x in region_min_x..=region_max_x {
                if seen.get(y, x) {
                    visited.set(y, x);
                }
            }
        }
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
