use std::{cmp::Reverse, fmt::Display, iter::repeat_n};

const EMPTY: i16 = -1;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    rayon::join(solve_part1, solve_part2)
}

fn solve_part1() -> u64 {
    let input = include_str!("input.txt").trim();

    let mut disk = Vec::new();
    let mut first_free = usize::MAX;
    let mut last_used = 0;
    let mut next_id = 0;

    for (i, val) in input.bytes().enumerate() {
        let val = (val - b'0') as usize;

        if i % 2 == 0 {
            disk.extend(repeat_n(next_id, val));
            last_used = disk.len() - 1;
            next_id += 1;
        } else {
            disk.extend(repeat_n(EMPTY, val));
            if first_free == usize::MAX {
                first_free = disk.len() - val;
            }
        }
    }

    while first_free < last_used {
        disk.swap(first_free, last_used);
        while disk[first_free] != EMPTY {
            first_free += 1;
        }
        while disk[last_used] == EMPTY {
            last_used -= 1;
        }
    }

    checksum(disk)
}

fn solve_part2() -> u64 {
    let input = include_str!("input.txt").trim();

    let mut next_id = 0;
    let mut spaces: [std::collections::BinaryHeap<Reverse<usize>>; 10] = std::array::from_fn(|_| Default::default());

    let mut disk = Vec::new();
    for (i, val) in input.bytes().enumerate() {
        let val = (val - b'0') as usize;

        if i % 2 == 0 {
            // It's a file
            disk.extend(repeat_n(next_id, val));
            next_id += 1;
        } else {
            // It's free space
            // push the start of this free space block
            let start_idx = disk.len();
            spaces[val].push(Reverse(start_idx));
            disk.extend(repeat_n(EMPTY, val));
        }
    }

    let mut i = disk.len() - 1;
    while i != 0 {
        if disk[i] == EMPTY {
            i -= 1;
            continue;
        }

        let file_id = disk[i];
        let mut file_size = 0;
        while disk[i] == file_id {
            file_size += 1;
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if let Some((smallest_i, best_width)) = (file_size..10)
            .filter_map(|size| spaces[size].peek().copied().map(|i| (i.0, size)))
            .min_by_key(|&(i, _)| i)
            .filter(|&(j, _)| j <= i)
        {
            disk[smallest_i..smallest_i + file_size].fill(file_id);
            disk[i + 1..i + 1 + file_size].fill(EMPTY);

            spaces[best_width].pop();

            if best_width > file_size {
                let leftover_start = smallest_i + file_size;
                let leftover_size = best_width - file_size;
                spaces[leftover_size].push(Reverse(leftover_start));
            }
        }
    }

    checksum(disk)
}

fn checksum(file_system: Vec<i16>) -> u64 {
    file_system
        .into_iter()
        .enumerate()
        .filter(|&(_, f)| f != EMPTY)
        .map(|(i, f)| i as u64 * f as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let (p1, p2) = solve();
        let expected: (usize, usize) = (6367087064415, 6390781891880);
        assert_eq!(p1.to_string(), expected.0.to_string());
        assert_eq!(p2.to_string(), expected.1.to_string());
    }
}
