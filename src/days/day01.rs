use crate::{day_output::DayOutput, direction1d::Direction1D};

struct Rotation {
    d: Direction1D,
    n: u64,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rotations = Vec::<Rotation>::new();
    for line in input.lines() {
        let (ds, ns) = line.split_at(1);
        let d = match ds {
            "L" => Direction1D::Left,
            "R" => Direction1D::Right,
            _ => panic!(),
        };
        let n = ns.parse::<u64>().unwrap();
        rotations.push(Rotation { d, n });
    }

    let mut part1 = 0u64;
    let mut part2 = 0u64;

    let mut r = 50i64;
    for rot in rotations.iter() {
        for _ in 0..rot.n {
            r = Direction1D::go(r, rot.d);
            if r == -1 {
                r += 100;
            }
            if r == 100 {
                r -= 100;
            }
            if r == 0 {
                part2 += 1;
            }
        }
        if r == 0 {
            part1 += 1;
        }
    }

    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
