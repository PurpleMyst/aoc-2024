use std::fmt::Display;

use std::iter::repeat_n;

#[derive(Debug, Clone, Copy)]
struct ContiguousBlock {
    file_id: Option<usize>,
    len: usize,
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").trim();

    let mut files = Vec::with_capacity(input.len() / 2);
    let mut free_spaces = Vec::with_capacity(input.len() / 2);
    for (i, b) in input.bytes().enumerate() {
        if i % 2 == 0 {
            files.push(b - b'0');
        } else {
            free_spaces.push(b - b'0');
        }
    }

    let mut file_system = Vec::with_capacity(input.len());
    for ((file_id, &file), &space) in files.iter().enumerate().zip(free_spaces.iter()) {
        file_system.extend(repeat_n(Some(file_id), file as usize));
        file_system.extend(repeat_n(None, space as usize));
    }
    if free_spaces.len() > files.len() {
        file_system.extend(repeat_n(None, *free_spaces.last().unwrap() as usize));
    } else if files.len() > free_spaces.len() {
        file_system.extend(repeat_n(Some(files.len() - 1), *files.last().unwrap() as usize));
    }

    let mut file_system_blocks = Vec::new();
    for ((file_id, &file), &space) in files.iter().enumerate().zip(free_spaces.iter()) {
        file_system_blocks.push(ContiguousBlock {
            file_id: Some(file_id),
            len: file as usize,
        });
        file_system_blocks.push(ContiguousBlock {
            file_id: None,
            len: space as usize,
        });
    }
    if free_spaces.len() > files.len() {
        file_system_blocks.push(ContiguousBlock {
            file_id: None,
            len: *free_spaces.last().unwrap() as usize,
        });
    } else if files.len() > free_spaces.len() {
        file_system_blocks.push(ContiguousBlock {
            file_id: Some(files.len() - 1),
            len: *files.last().unwrap() as usize,
        });
    }

    rayon::join(|| solve_part1(file_system.clone()), || solve_part2(files, file_system_blocks))
}

fn solve_part1(mut file_system: Vec<Option<usize>>) -> usize {
    let mut first_free = file_system.iter().position(|&x| x.is_none()).unwrap();
    let mut last_used = file_system.iter().rposition(|&x| x.is_some()).unwrap();

    while first_free < last_used {
        file_system.swap(first_free, last_used);
        while file_system[first_free].is_some() {
            first_free += 1;
        }
        while file_system[last_used].is_none() {
            last_used -= 1;
        }
    }

    checksum(file_system)
}

fn checksum(file_system: Vec<Option<usize>>) -> usize {
    file_system
        .into_iter()
        .enumerate()
        .filter_map(|(i, f)| Some(i * f?))
        .sum::<usize>()
}

fn solve_part2(files: Vec<u8>, mut file_system: Vec<ContiguousBlock>) -> usize {
    for file_id in (0..files.len()).rev() {
        let file_len = files[file_id] as usize;
        let src = file_system
            .iter()
            .position(|block| block.file_id == Some(file_id))
            .unwrap();
        let Some(dst_idx) = file_system
            .iter()
            .take(src + 1)
            .position(|block| block.file_id.is_none() && block.len >= file_len)
        else {
            continue;
        };

        // Decrement the length of the destination free space block.
        file_system[dst_idx].len -= file_len;

        // Mark the source file block as free.
        file_system[src].file_id = None;
        // Merge the source file block with adjacent free space blocks.
        if let Some(&ContiguousBlock { file_id: None, len: right_len }) = file_system.get(src + 1) {
            file_system[src].len += right_len;
            file_system.remove(src + 1);
        }
        if let Some(&ContiguousBlock { file_id: None, len: left_len }) = file_system.get(src - 1).filter(|_| dst_idx != (src - 1)) {
            file_system[src].len += left_len;
            file_system.remove(src - 1);
        }

        // Insert the file block at the destination.
        file_system.insert(
            dst_idx,
            ContiguousBlock {
                file_id: Some(file_id),
                len: file_len,
            },
        );
    }

    checksum_blocks(file_system)
}

fn checksum_blocks(file_system: Vec<ContiguousBlock>) -> usize {
    let mut sum = 0;
    let mut pos = 0;

    for block in file_system {
        if let Some(file_id) = block.file_id {
            // sum of the sequence pos...(pos+L-1) is (L*(2*pos + L - 1))/2
            let range_sum = (block.len * (2 * pos + block.len - 1)) / 2;
            sum += range_sum * file_id;
        }
        pos += block.len;
    }

    sum
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
