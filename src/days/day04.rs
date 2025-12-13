use std::collections::HashSet;

use crate::{
    day_output::DayOutput,
    facing::{FACINGS, Facing},
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

    let mut is_first_iteration = true;
    let mut removed_papers = HashSet::<(isize, isize)>::new();
    let mut next_removed_papers = Vec::<(isize, isize)>::new();
    loop {
        for (y, row) in map.rows.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                if !matches!(tile, Tile::Paper) {
                    continue;
                }
                let pos = (x as isize, y as isize);
                if removed_papers.contains(&pos) {
                    continue;
                }
                let mut papers = 0usize;
                for f in FACINGS {
                    let d = Facing::go(pos, *f);
                    let nt = map.at(d);
                    if let Some(tile) = nt {
                        match tile {
                            Tile::Empty => (),
                            Tile::Paper => {
                                if !removed_papers.contains(&d) {
                                    papers += 1
                                }
                            }
                        }
                    }
                }
                if papers < 4 {
                    if is_first_iteration {
                        part1 += 1;
                    }
                    part2 += 1;
                    next_removed_papers.push(pos);
                }
            }
        }
        is_first_iteration = false;
        if next_removed_papers.is_empty() {
            break;
        }
        removed_papers.extend(next_removed_papers.drain(..));
    }
    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
