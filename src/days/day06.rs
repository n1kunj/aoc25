use crate::day_output::DayOutput;

enum Op {
    Add,
    Mul,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut nums = Vec::<Vec<u64>>::new();
    let mut ops = Vec::<Op>::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace().peekable();
        let first_token = *tokens.peek().unwrap();
        if matches!(first_token, "*" | "+") {
            for token in tokens {
                match token {
                    "*" => ops.push(Op::Mul),
                    "+" => ops.push(Op::Add),
                    _ => panic!(),
                }
            }
        } else {
            let mut list = Vec::<u64>::new();
            for token in tokens {
                list.push(token.parse().unwrap());
            }
            nums.push(list);
        }
    }

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
}
