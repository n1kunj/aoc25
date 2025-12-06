use crate::{
    day_output::DayOutput,
    facing::{Facing, FACINGS},
    map::{Map, Row},
};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Paper,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row<Tile>>::new();
    for line in input.lines() {
        let mut tiles = Vec::<Tile>::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Empty,
                '@' => Tile::Paper,
                _ => panic!(),
            };
            tiles.push(tile);
        }
        rows.push(Row { tiles });
    }
    let map = Map { rows };

    let mut part1 = 0usize;
    let mut part2 = 0usize;

    for (y, row) in map.rows.iter().enumerate() {
        for (x, tile) in row.tiles.iter().enumerate() {
            if !matches!(tile, Tile::Paper) {
                continue;
            }
            let mut papers = 0usize;
            for f in FACINGS {
                let d = Facing::go((x as isize, y as isize), *f);
                let nt = map.at(d);
                match nt {
                    Some(tile) => match tile {
                        Tile::Empty => (),
                        Tile::Paper => papers += 1,
                    },
                    None => (),
                }
            }
            if papers < 4 {
                part1 += 1;
            }
        }
    }
    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
