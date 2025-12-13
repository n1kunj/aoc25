use std::collections::HashSet;

use crate::{
    day_output::DayOutput,
    map::{Map, Row},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Present(usize),
}

#[allow(unused)]
#[derive(Debug)]
struct Region {
    x: usize,
    y: usize,
    counts: Vec<usize>,
    shape_idxs: Vec<usize>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut shapes = Vec::<Map<Tile>>::new();
    let mut regions = Vec::<Region>::new();

    let mut cur_shape = Option::<Vec<Row<Tile>>>::None;

    for line in input.lines() {
        if line.is_empty() {
            if let Some(shape) = cur_shape.take() {
                shapes.push(Map { rows: shape });
            }
        } else if line.ends_with(":") {
            cur_shape = Some(vec![]);
        } else if let Some(cur) = &mut cur_shape {
            let mut tiles = Vec::<Tile>::new();
            for c in line.chars() {
                let t = match c {
                    '#' => Tile::Present(shapes.len()),
                    '.' => Tile::Empty,
                    _ => panic!(),
                };
                tiles.push(t);
            }
            cur.push(Row { tiles });
        } else {
            let mut tokens = line.split_whitespace();
            let dims = tokens.next().unwrap();
            let dims = &dims[0..dims.len() - 1];
            let mut dims = dims.split("x");
            let x = dims.next().unwrap().parse::<usize>().unwrap();
            let y = dims.next().unwrap().parse::<usize>().unwrap();
            let mut counts = Vec::<usize>::new();
            let mut shape_idxs = Vec::<usize>::new();
            for (i, token) in tokens.enumerate() {
                let c = token.parse::<usize>().unwrap();
                counts.push(c);
                for _ in 0..c {
                    shape_idxs.push(i);
                }
            }
            regions.push(Region {
                x,
                y,
                counts,
                shape_idxs,
            });
        }
    }

    let shapes = shapes;
    let regions = regions;

    let mut shape_presents = Vec::<usize>::new();
    for shape in shapes.iter() {
        let mut presents = 0usize;
        for row in shape.rows.iter() {
            for t in row.tiles.iter() {
                if let Tile::Present(_) = t {
                    presents += 1;
                }
            }
        }
        shape_presents.push(presents);
    }

    let mut shape_variants = Vec::<Vec<Map<Tile>>>::new();

    fn rotate_right(shape: &Map<Tile>) -> Map<Tile> {
        let mut out = shape.clone();
        for (y, row) in shape.rows.iter().enumerate() {
            for (x, t) in row.tiles.iter().enumerate() {
                let new_x = row.tiles.len() - 1 - y;
                let new_y = x;
                *out.at_mut((new_x as isize, new_y as isize)).unwrap() = *t;
            }
        }
        out
    }

    fn flip_y(shape: &Map<Tile>) -> Map<Tile> {
        let mut out = shape.clone();
        for (y, row) in shape.rows.iter().enumerate() {
            for (x, t) in row.tiles.iter().enumerate() {
                let new_x = x;
                let new_y = row.tiles.len() - 1 - y;
                *out.at_mut((new_x as isize, new_y as isize)).unwrap() = *t;
            }
        }
        out
    }

    for shape in shapes.iter() {
        let mut variants = HashSet::<Map<Tile>>::new();

        let r0 = shape.clone();
        let r1 = rotate_right(&r0);
        let r2 = rotate_right(&r1);
        let r3 = rotate_right(&r2);

        let r4 = flip_y(&r3);
        let r5 = rotate_right(&r4);
        let r6 = rotate_right(&r5);
        let r7 = rotate_right(&r6);

        variants.insert(r0);
        variants.insert(r1);
        variants.insert(r2);
        variants.insert(r3);
        variants.insert(r4);
        variants.insert(r5);
        variants.insert(r6);
        variants.insert(r7);
        shape_variants.push(variants.drain().collect::<Vec<_>>());
    }

    let shape_presents = shape_presents;
    let shape_variants = shape_variants;

    let mut part1 = 0usize;
    for region in regions.iter() {
        let spaces = region.x * region.y;
        let to_fill = region
            .shape_idxs
            .iter()
            .map(|i| shape_presents[*i])
            .sum::<usize>();
        if to_fill > spaces {
            continue;
        }

        let mut rows = Vec::<Row<Tile>>::new();
        for _ in 0..region.y {
            rows.push(Row {
                tiles: vec![Tile::Empty; region.x],
            });
        }
        let map = Map { rows };

        #[allow(unused)]
        fn print_map(map: &Map<Tile>) {
            for (y, row) in map.rows.iter().enumerate() {
                for (x, t) in row.tiles.iter().enumerate() {
                    let s = match t {
                        Tile::Empty => ".",
                        Tile::Present(t) => &t.to_string(),
                    };
                    print!("{s}");
                }
                println!();
            }
            println!();
        }

        fn recurse(
            region: &Region,
            shape_variants: &Vec<Vec<Map<Tile>>>,
            map: &Map<Tile>,
            shape_idx: usize,
        ) -> bool {
            if region.shape_idxs.get(shape_idx).is_none() {
                return true;
            }
            let shape = region.shape_idxs[shape_idx];
            let variants = &shape_variants[shape];
            for ypos in 0..=region.y - variants[0].rows.len() {
                for xpos in 0..=region.x - variants[0].rows[0].tiles.len() {
                    'restart: for shape_variant in variants.iter() {
                        // Check if it fits before committing.
                        for (y, row) in shape_variant.rows.iter().enumerate() {
                            for (x, t) in row.tiles.iter().enumerate() {
                                if *t == Tile::Empty {
                                    continue;
                                }
                                let dstx = (xpos + x) as isize;
                                let dsty = (ypos + y) as isize;
                                if let Tile::Present(_) = map.at((dstx, dsty)).unwrap() {
                                    continue 'restart;
                                }
                            }
                        }

                        let mut map = map.clone();

                        for (y, row) in shape_variant.rows.iter().enumerate() {
                            for (x, t) in row.tiles.iter().enumerate() {
                                if *t == Tile::Empty {
                                    continue;
                                }
                                let dstx = (xpos + x) as isize;
                                let dsty = (ypos + y) as isize;
                                let dstt = map.at_mut((dstx, dsty)).unwrap();
                                match *dstt {
                                    Tile::Empty => *dstt = Tile::Present(shape_idx),
                                    Tile::Present(_) => panic!(),
                                }
                            }
                        }
                        let fits = recurse(region, shape_variants, &map, shape_idx + 1);
                        if fits {
                            return true;
                        }
                    }
                }
            }
            false
        }

        let fits = recurse(region, &shape_variants, &map, 0);
        if fits {
            part1 += 1;
        }
    }

    output.part1(part1.to_string());
}
