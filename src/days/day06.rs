use crate::{
    day_output::DayOutput,
    map::{Map, Row},
};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Copy)]
enum Entry {
    Num(u64),
    Empty,
    Op(Op),
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut nums = Vec::<Vec<u64>>::new();
    let mut rows = Vec::<Row<Entry>>::new();
    let mut ops = Vec::<Op>::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace().peekable();
        let first_token = *tokens.peek().unwrap();
        if matches!(first_token, "*" | "+") {
            let mut tiles = Vec::<Entry>::new();
            for c in line.chars() {
                let e = match c {
                    ' ' => Entry::Empty,
                    '*' => Entry::Op(Op::Mul),
                    '+' => Entry::Op(Op::Add),
                    _ => panic!(),
                };
                if let Entry::Op(op) = e {
                    ops.push(op)
                }
                tiles.push(e);
            }
            rows.push(Row { tiles });
        } else {
            let mut list = Vec::<u64>::new();
            for token in tokens {
                list.push(token.parse().unwrap());
            }
            nums.push(list);

            let mut tiles = Vec::<Entry>::new();
            for c in line.chars() {
                let e = match c {
                    ' ' => Entry::Empty,
                    n => Entry::Num(n.to_digit(10).unwrap() as u64),
                };
                tiles.push(e);
            }
            rows.push(Row { tiles });
        }
    }
    let map = Map { rows };

    let columns = ops.len();

    let mut part1 = 0u64;
    for i in 0..columns {
        let op = &ops[i];
        let val = match op {
            Op::Add => {
                let mut accum: u64 = 0u64;
                for num in nums.iter() {
                    accum += num[i];
                }
                accum
            }
            Op::Mul => {
                let mut accum: u64 = 1u64;
                for num in nums.iter() {
                    accum *= num[i];
                }
                accum
            }
        };
        part1 += val;
    }
    output.part1(part1.to_string());

    let mut part2 = 0u64;
    let oprow = map.rows.last().unwrap();
    let mut ranges = Vec::<(usize, usize)>::new();

    let mut cur_start = 0usize;
    for (i, op) in oprow.tiles[1..].iter().enumerate() {
        if matches!(op, Entry::Op(_)) {
            ranges.push((cur_start, i + 1));
            cur_start = i + 1;
        }
    }
    ranges.push((cur_start, oprow.tiles.len()));

    let mut vals = Vec::<u64>::new();
    for (i0, i1) in ranges.iter() {
        let op = match oprow.tiles[*i0] {
            Entry::Op(op) => op,
            _ => panic!(),
        };
        vals.clear();
        for i in (*i0..*i1).rev() {
            let mut any_found = false;
            let mut buf: u64 = 0;
            for r in map.rows.iter() {
                if let Entry::Num(n) = r.tiles[i] {
                    any_found = true;
                    buf = buf * 10 + n;
                }
            }
            if any_found {
                vals.push(buf);
            }
        }
        part2 += match op {
            Op::Add => vals.iter().sum::<u64>(),
            Op::Mul => vals.iter().product(),
        };
    }
    output.part2(part2.to_string());
}
