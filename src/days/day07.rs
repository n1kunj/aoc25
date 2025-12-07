use std::collections::HashMap;

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

    let mut beamfronts = HashMap::<(isize, isize), u64>::new();
    beamfronts.insert(
        Direction2D::go((start.0 as isize, start.1 as isize), Direction2D::Down),
        1,
    );

    let mut new_beamfronts = HashMap::<(isize, isize), u64>::new();
    let mut exited = 0u64;
    let mut split_count = 0u64;
    loop {
        let mut add_new_beamfront = |beam: (isize, isize), count: u64| {
            new_beamfronts
                .entry(beam)
                .and_modify(|e| *e += count)
                .or_insert(count);
        };
        for (beam, count) in beamfronts.drain() {
            let next = Direction2D::go(beam, Direction2D::Down);
            let t = map.at(next);

            match t {
                Some(t) => match t {
                    Tile::Empty => {
                        add_new_beamfront(next, count);
                    }
                    Tile::Splitter => {
                        split_count += 1;

                        let l = Direction2D::go(next, Direction2D::Left);
                        let r = Direction2D::go(next, Direction2D::Right);
                        add_new_beamfront(l, count);
                        add_new_beamfront(r, count);
                    }
                    Tile::Start => panic!(),
                },
                None => exited += count,
            }
        }
        beamfronts.extend(new_beamfronts.drain());
        if beamfronts.is_empty() {
            break;
        }
    }
    output.part1(split_count.to_string());
    output.part2(exited.to_string());
}
