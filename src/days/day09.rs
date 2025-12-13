use std::collections::HashMap;

use crate::{
    day_output::DayOutput,
    direction2d::{Direction2D, DIRECTIONS},
    map::{Map, Row},
};

pub fn main(input: &str, output: &mut DayOutput) {
    let mut tiles = Vec::<(u64, u64)>::new();

    for line in input.lines() {
        let mut tokens = line.split(",");
        let x: u64 = tokens.next().unwrap().parse().unwrap();
        let y: u64 = tokens.next().unwrap().parse().unwrap();
        assert!(tokens.next().is_none());
        tiles.push((x, y));
    }
    let tiles = tiles;

    let mut part1 = 0u64;

    for (i, a) in tiles.iter().enumerate() {
        for b in tiles[i..].iter() {
            let dx = a.0.abs_diff(b.0) + 1;
            let dy = a.1.abs_diff(b.1) + 1;
            let area = dx * dy;
            part1 = part1.max(area);
        }
    }

    output.part1(part1.to_string());

    let mut sortedxtiles = tiles.iter().map(|t| t.0).collect::<Vec<_>>();
    sortedxtiles.sort();
    sortedxtiles.dedup();
    let x_to_cx = sortedxtiles
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i as isize))
        .collect::<HashMap<_, _>>();

    let mut sortedytiles = tiles.iter().map(|t| t.1).collect::<Vec<_>>();
    sortedytiles.sort();
    sortedytiles.dedup();
    let y_to_cy = sortedytiles
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i as isize))
        .collect::<HashMap<_, _>>();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Unknown,
        None,
        Red,
        Green,
    }

    let mut rows = Vec::<Row<Tile>>::new();
    for _ in 0..=sortedytiles.len() {
        rows.push(Row {
            tiles: vec![Tile::Unknown; sortedxtiles.len() + 1],
        });
    }
    let mut map = Map { rows };

    let mut prev_tile = *tiles.last().unwrap();
    for t in tiles.iter() {
        let t0 = (
            *x_to_cx.get(&prev_tile.0).unwrap(),
            *y_to_cy.get(&prev_tile.1).unwrap(),
        );
        let t1 = (*x_to_cx.get(&t.0).unwrap(), *y_to_cy.get(&t.1).unwrap());
        *map.at_mut(t0).unwrap() = Tile::Red;
        if t0.0 == t1.0 {
            let mut ys = [t0.1, t1.1];
            ys.sort_unstable();
            for y in ys[0] + 1..ys[1] {
                *map.at_mut((t0.0, y)).unwrap() = Tile::Green;
            }
        } else {
            let mut xs = [t0.0, t1.0];
            xs.sort_unstable();
            for x in xs[0] + 1..xs[1] {
                *map.at_mut((x, t0.1)).unwrap() = Tile::Green;
            }
        }
        prev_tile = *t;
    }

    loop {
        let mut any_new_nones = false;
        for y in 0..map.rows.len() {
            for x in 0..map.rows[y].tiles.len() {
                let p = (x as isize, y as isize);
                let t = map.at(p).unwrap();
                if t != Tile::Unknown {
                    continue;
                }
                for d in DIRECTIONS {
                    let n = Direction2D::go(p, d);
                    let nt = map.at(n);
                    let is_t_none = match nt {
                        Some(nt) => matches!(nt, Tile::None),
                        None => true,
                    };
                    if is_t_none {
                        any_new_nones = true;
                        *map.at_mut(p).unwrap() = Tile::None;
                        break;
                    }
                }
            }
        }
        if !any_new_nones {
            break;
        }
    }

    let mut part2 = 0u64;

    for (i, a) in tiles.iter().enumerate() {
        for b in tiles[i..].iter() {
            let dx = a.0.abs_diff(b.0) + 1;
            let dy = a.1.abs_diff(b.1) + 1;
            let area = dx * dy;
            if area > part2 {
                let ca = (*x_to_cx.get(&a.0).unwrap(), *y_to_cy.get(&a.1).unwrap());
                let cb = (*x_to_cx.get(&b.0).unwrap(), *y_to_cy.get(&b.1).unwrap());

                let mut cxs = [ca.0, cb.0];
                cxs.sort_unstable();
                let mut cys = [ca.1, cb.1];
                cys.sort_unstable();
                let mut has_none = false;
                'outer: for cy in cys[0]..=cys[1] {
                    for cx in cxs[0]..=cxs[1] {
                        let t = map.at((cx, cy)).unwrap();
                        if t == Tile::None {
                            has_none = true;
                            break 'outer;
                        }
                    }
                }
                if !has_none {
                    part2 = area;
                }
            }
        }
    }
    output.part2(part2.to_string());
}
