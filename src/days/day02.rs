use std::fmt::Write;

use num::Integer;

use crate::day_output::DayOutput;

pub fn main(input: &str, output: &mut DayOutput) {
    let mut part1 = 0u64;
    let mut part2 = 0u64;
    for id in input.split(",") {
        let mut tokens = id.split("-").into_iter();
        let first_s = tokens.next().unwrap();
        let second_s = tokens.next().unwrap();
        assert!(tokens.next().is_none());

        let first = first_s.parse::<u64>().unwrap();
        let second = second_s.parse::<u64>().unwrap();

        let mut buf = String::new();
        for i in first..=second {
            buf.clear();
            write!(&mut buf, "{i}").unwrap();

            for d in (1..=buf.len() / 2).rev() {
                let (div, rem) = buf.len().div_rem(&d);
                if rem != 0 {
                    continue;
                }

                let ss = &buf[0..d];
                let mut any_not_matching = false;
                for n in 1..div {
                    if ss != &buf[d * n..d * (n + 1)] {
                        any_not_matching = true;
                        break;
                    }
                }
                if !any_not_matching {
                    if buf.len() % 2 == 0 && d == buf.len() / 2 {
                        part1 += i;
                    }
                    part2 += i;
                    break;
                }
            }
        }
    }

    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
