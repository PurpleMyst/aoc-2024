use std::fmt::Display;

use owo_colors::OwoColorize;

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
    // sort by letter
    // distinct_regions.sort_by_key(|&r0| map.iter().zip(&regions).filter(|&(_, r)| r0 == *r).map(|(&b, _)| b).min().unwrap());

    let mut p1 = 0;
    let mut p2 = 0;

    for region in distinct_regions {
        let mut area = 0;
        let mut perimeter = 0;

        let start_idx = (0..side * side).find(|&idx| regions[idx] == region).unwrap();
        let (start_y, start_x) = coords(start_idx);

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

        // let (mut cy, mut cx) = (start_y as isize - 1, start_x as isize);
        //
        // let (mut dy, mut dx) = (0, 1);
        //
        // let mut seen = HashSet::new();
        //
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
        //
        // // hole = point not in region surrounded by regionn
        // let mut holes =  0;
        // for (y, x) in (0..side).flat_map(|y| (0..side).map(move |x| (y, x))) {
        //     if regions[idx(y, x)] != region {
        //         (-1..=1)
        //             .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
        //             .filter(|&(dy, dx)| (dy, dx) != (0, 0))
        //             .all(|(dy, dx)| {
        //                 let ny = y as isize + dy;
        //                 let nx = x as isize + dx;
        //                 (0..side as isize).contains(&ny) && (0..side as isize).contains(&nx) &&
        //                 regions[idx(ny as usize, nx as usize)] == region
        //             }).then(|| holes += 1);
        //     }
        // }
        //     let mut sides = 0;
        //
        // loop {
        //     // println!("  pos=({cy:2}, {cx:2}) dir={:?} cell={:?} sides={sides:2}", 
        //     //     match (dy, dx) {
        //     //         (0, 1) => "R",
        //     //         (1, 0) => "D",
        //     //         (0, -1) => "L",
        //     //         (-1, 0) => "U",
        //     //         _ => unreachable!(),
        //     //     },
        //     //     ((0..side as isize).contains(&cy) && (0..side as isize).contains(&cx)).then(||
        //     //     map.get(idx(cy as usize, cx as usize)).map(|&b| b as char)).flatten());
        //
        //     println!("{sides:4}");
        //     for y in region_min_y as isize - 1..=region_max_y as isize + 1 {
        //         for x in region_min_x as isize -1..=region_max_x as isize + 1 {
        //             if (y, x) == (cy, cx) {
        //                 print!("\\x1b[31m{}\\x1b[0m", match (dy,dx) {
        //                     (0, 1) => '>',
        //                     (1, 0) => 'v',
        //                     (0, -1) => '<',
        //                     (-1, 0) => '^',
        //                     _ => unreachable!(),
        //                 });
        //                 continue;
        //             }
        //
        //             let c = if 
        //
        //                 (0..side as isize).contains(&y) && (0..side as isize).contains(&x) 
        //                 && regions[idx(y as usize, x as usize)] == region
        //             {
        //                 map[idx(y as usize, x as usize)] as char
        //             } else {
        //                 '.'
        //             };
        //             print!("{}", c);
        //         }
        //         println!();
        //     }
        //     println!();
        //
        //     assert!(seen.insert((cy, cx, dy, dx)));
        //
        //     let right_dy = dx;
        //     let right_dx = -dy;
        //
        //     let left_dy = -dx;
        //     let left_dx = dy;
        //
        //     let can_move = |dy: isize, dx: isize|
        //         (!(0..side as isize).contains(&(cy + dy)) ||
        //         !(0..side as isize).contains(&(cx + dx)) ||
        //         regions[idx((cy + dy) as usize, (cx + dx) as usize)] != region)
        //          && /* check if there's at least one neighbor within the region */
        //           (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
        //             .filter(|&(dy, dx)| (dy, dx) != (0, 0))
        //             .any(|(dy, dx)| {
        //                 let ny = cy + dy;
        //                 let nx = cx + dx;
        //                 (0..side as isize).contains(&ny) && (0..side as isize).contains(&nx) &&
        //                 regions[idx(ny as usize, nx as usize)] == region
        //             })
        //         ;
        //
        //     if can_move(right_dy, right_dx) {
        //         dy = right_dy;
        //         dx = right_dx;
        //         sides += 1;
        //     } else if can_move(dy, dx) {
        //         // continue straight
        //     } else {
        //         dy = left_dy;
        //         dx = left_dx;
        //         sides += 1;
        //         continue;
        //     }
        //     cy += dy;
        //     cx += dx;
        //
        //     if cy == start_y as isize - 1 && cx == start_x as isize {
        //         break;
        //     }
        // }

        // bitset method
        let mut sides = 0;

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

        let mut bitset = vec![0u64; (1 + side * 2) * (1 + side * 2)];
        for y in 0..side {
            for x in 0..side {
                if regions[idx(y, x)] != region {
                    continue;
                }
                for (i, (dy, dx)) in DIRS.iter().copied().enumerate() {
                    let ny = 1+(2*y as isize + dy);
                    let nx = 1+(2*x as isize + dx);
                    assert!((0..=side as isize * 2).contains(&ny));
                    assert!((0..=side as isize * 2).contains(&nx));

                    bitset[ny as usize * (1 + side * 2) + nx as usize] |= 1 << i;
                }
            }
        }

        // draw regular grid
        for y in region_min_y..=region_max_y {
            for x in region_min_x..=region_max_x {
                let i = idx(y, x);
                if regions[i] == region {
                    print!("{}", map[i] as char);
                } else {
                    print!("{}", ".".dimmed());
                }
            }
            println!();
        }

        // draw expanded grid with bitset
        // for y in 0..=side*2 {
        //     for x in 0..=side*2 {
        for y in 2 * (region_min_y as isize)..=2 * (region_max_y as isize + 1) {
            for x in 2 * (region_min_x as isize)..=2 * (region_max_x as isize + 1) {
                let y = y as usize;
                let x = x as usize;

                if y % 2 == 1 && x % 2 == 1 {
                    let i = idx(y / 2, x / 2);
                    if regions[i] == region {
                        print!("{}", map[i] as char);
                    } else {
                        print!("{}", ".".dimmed());
                    }
                } else {
                    // this is a boundary point
                    let flags = bitset[y * (1 + side * 2) + x];
                    if flags & 0xf0 != 0 && flags & 0x0f == 0 
                        && (flags & 0xf0).count_ones() % 2 == 1 
                    {
                        print!("{}", "*".red());
                        sides += 1;
                    } else if flags & 0b1001_0000 == 0b1001_0000 
                        && flags & 0b0110_0000 == 0
                        && flags & 0x0f == 0 {
                        print!("{}", "*".green());
                        sides += 2;
                    } else if flags & 0b0110_0000 == 0b0110_0000 
                        && flags & 0b1001_0000 == 0
                        && flags & 0x0f == 0 {
                        print!("{}", "*".blue());
                        sides += 2;
                    } else {
                        // print!("{}", flags.count_ones().dimmed());
                        print!("{}", ".".dimmed());
                    }
                }
            }
            println!();
        }
        println!();


        println!("Region {}:", (map[start_idx] as char).bold());
        println!("  area      = {:3}", area.red());
        println!("  perimeter = {:3}", perimeter.green());
        println!("  sides     = {:3}", sides.blue());
        // sides += 4 * holes;


        p1 += area * perimeter;
        p2 += area * sides;
    }

    debug_assert_ne!(p2, 845490);

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
