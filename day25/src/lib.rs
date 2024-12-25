use std::fmt::Display;

use rayon::prelude::*;

#[derive(Debug)]
enum SchematicType {
    Key,
    Door,
}

#[derive(Debug)]
struct Schematic {
    ty: SchematicType,
    columns: [u8; 5],
}

impl Schematic {
    fn parse(s: &str) -> Self {
        let ty = if s.starts_with(".....") {
            SchematicType::Key
        } else {
            SchematicType::Door
        };

        let mut columns = [0; 5];

        s.lines().for_each(|row| {
            row.bytes().zip(columns.iter_mut()).for_each(|(b, col)| {
                if b == b'#' {
                    *col += 1;
                }
            })
        });

        columns.iter_mut().for_each(|col| {
            *col -= 1;
        });

        Self { ty, columns }
    }
}

fn fit_together(key: [u8; 5], door: [u8; 5]) -> bool {
    std::iter::zip(key.iter(), door.iter()).all(|(k, d)| (k + d) <= 5)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let schematics = include_str!("input.txt")
        .split("\n\n")
        .map(Schematic::parse);

    let mut keys = Vec::new();
    let mut doors = Vec::new();
    for schematic in schematics {
        match schematic.ty {
            SchematicType::Key => keys.push(schematic.columns),
            SchematicType::Door => doors.push(schematic.columns),
        }
    }

    let p1 = keys
        .par_iter()
        .map(|&key| doors.iter().filter(|&&door| fit_together(key, door)).count())
        .sum::<usize>();

    (p1, "Merry Christmas!")
}
