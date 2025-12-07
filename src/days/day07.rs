use std::collections::HashSet;

use crate::{
    day_output::DayOutput,
    direction2d::Direction2D,
    map::{Map, Row},
};

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Splitter,
    Start,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row<Tile>>::new();
    let mut start: Option<(usize, usize)> = None;
    for line in input.lines() {
        let mut row = Vec::<Tile>::new();
        for c in line.chars() {
            let t = match c {
                '.' => Tile::Empty,
                'S' => Tile::Start,
                '^' => Tile::Splitter,
                _ => panic!(),
            };
            if t == Tile::Start {
                start = Some((row.len(), rows.len()));
            }
            row.push(t);
        }
        rows.push(Row { tiles: row });
    }
    let start = start.unwrap();
    let map = Map { rows };

    let mut beamfronts = HashSet::<(isize, isize)>::new();

    beamfronts.insert(Direction2D::go(
        (start.0 as isize, start.1 as isize),
        Direction2D::Down,
    ));

    let mut new_beamfronts = HashSet::<(isize, isize)>::new();

    let mut split_count = 0u64;

    loop {
        for beam in beamfronts.drain() {
            let next = Direction2D::go(beam, Direction2D::Down);
            let t = map.at(next);
            if let Some(t) = t {
                match t {
                    Tile::Empty => {
                        new_beamfronts.insert(next);
                    }
                    Tile::Splitter => {
                        let l = Direction2D::go(next, Direction2D::Left);
                        let r = Direction2D::go(next, Direction2D::Right);
                        split_count += 1;
                        new_beamfronts.insert(l);
                        new_beamfronts.insert(r);
                    }
                    Tile::Start => panic!(),
                };
            }
        }
        beamfronts.extend(new_beamfronts.drain());
        if beamfronts.is_empty() {
            break;
        }
    }

    output.part1(split_count.to_string());
}
