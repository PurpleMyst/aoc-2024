use std::fmt::Display;

use std::iter::repeat_n;

fn generate_ansi_escape(input: usize) -> String {
    use std::hash::*;

    // Hash the input integer to generate a semi-random seed.
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Generate RGB values based on the hash.
    let r = (hash & 0xFF) as u8; // Red
    let g = ((hash >> 8) & 0xFF) as u8; // Green
    let b = ((hash >> 16) & 0xFF) as u8; // Blue

    // Create the ANSI escape sequence for RGB color.
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
fn show(fs: &[Option<usize>]) -> String {
    fs.iter()
        .map(|x| x.map_or(".".to_string(), |x| format!("{}{x}\x1b[0m", generate_ansi_escape(x))))
        .collect::<Vec<String>>()
        .join(",")
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

    let p1 = solve_part1(file_system.clone());
    let p2 = solve_part2(files, file_system);
    debug_assert!(p2 < 6478879023463);

    (p1, p2)
}

fn solve_part1(mut file_system: Vec<Option<usize>>) -> usize {
    loop {
        let Some(last_used) = file_system.iter().rposition(|&x| x.is_some()) else {
            break;
        };
        let Some(first_free) = file_system
            .iter()
            .position(|&x| x.is_none())
            .filter(|&x| file_system[x + 1..].iter().any(|&x| x.is_some()))
        else {
            break;
        };
        file_system.swap(first_free, last_used);

        while file_system.last() == Some(&None) {
            file_system.pop();
        }
    }

    checksum(file_system)
}

fn checksum(file_system: Vec<Option<usize>>) -> usize {
    file_system
        .into_iter()
        .enumerate()
        .filter_map(|(i, f)| f.map(|f| i * f as usize))
        .sum::<usize>()
}

fn solve_part2(files: Vec<u8>, mut file_system: Vec<Option<usize>>) -> usize {
    for file_id in (0..files.len()).rev() {
        let mut l = 0;

        let file_len = files[file_id] as usize;

        let src = file_system.iter().position(|&x| x == Some(file_id)).unwrap();
        let Some(dst) = file_system
            .iter()
            .enumerate()
            .take(src + files[file_id] as usize)
            .find_map(|(i, b)| {
                if l >= files[file_id] as usize {
                    return Some(i - l);
                }

                if b.is_none() {
                    l += 1;
                } else {
                    l = 0;
                }

                None
            })
        else {
            continue;
        };

        file_system[src..src + file_len].fill(None);
        file_system[dst..dst + file_len].fill(Some(file_id));
    }

    checksum(file_system)
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
