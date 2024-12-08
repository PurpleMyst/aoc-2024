use std::fmt::Display;

type Coord = isize;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut antennas_by_frequency = vec![vec![]; 256];
    input.bytes().filter(|&b| b != b'\n').enumerate().for_each(|(i, b)| {
        if b.is_ascii_alphanumeric() {
            antennas_by_frequency[b as usize].push(((i / width) as Coord, (i % width) as Coord));
        }
    });

    let mut antinodes_part1 = vec![false; width * height];
    let mut antinodes_part2 = vec![false; width * height];

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

                    if (0..width as Coord).contains(&x0) && (0..height as Coord).contains(&y0) {
                        let i0 = (y0 * width as Coord + x0) as usize;
                        if k == 2 {
                            antinodes_part1[i0] = true;
                        }
                        antinodes_part2[i0] = true;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (
        antinodes_part1.into_iter().filter(|&b| b).count(),
        antinodes_part2.into_iter().filter(|&b| b).count(),
    )
}
