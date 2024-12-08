use std::fmt::Display;

type Coord = i16;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let side = input.lines().next().unwrap().len();

    let mut antennas_by_frequency = vec![vec![]; 256];
    input.bytes().filter(|&b| b != b'\n').enumerate().for_each(|(i, b)| {
        if b.is_ascii_alphanumeric() {
            antennas_by_frequency[b as usize].push(((i / side) as Coord, (i % side) as Coord));
        }
    });

    let mut antinodes_part1 = vec![0u64; side];
    let mut antinodes_part2 = vec![0u64; side];

    for antennas in &antennas_by_frequency {
        for (y1, x1) in antennas.iter() {
            for (y2, x2) in antennas.iter() {
                let dy = y2 - y1;
                let dx = x2 - x1;
                if (dy, dx) == (0, 0) {
                    continue;
                }

                for k in 0.. {
                    let y0 = y1 + k * dy;
                    let x0 = x1 + k * dx;

                    if (0..side as Coord).contains(&x0) && (0..side as Coord).contains(&y0) {
                        if k == 2 {
                            antinodes_part1[y0 as usize] |= 1 << x0;
                        }
                        antinodes_part2[y0 as usize] |= 1 << x0;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (
        antinodes_part1.into_iter().map(|n| n.count_ones()).sum::<u32>(),
        antinodes_part2.into_iter().map(|n| n.count_ones()).sum::<u32>(),
    )
}
