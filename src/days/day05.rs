use crate::day_output::DayOutput;

#[derive(Clone, Copy, Debug)]
struct Range {
    a: u64,
    b: u64,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut in_ranges = true;
    let mut ranges = Vec::<Range>::new();
    let mut ids = Vec::<u64>::new();
    for line in input.lines() {
        if in_ranges {
            if line.is_empty() {
                in_ranges = false;
            } else {
                let mut tokens = line.split('-');
                let a = tokens.next().unwrap().parse::<u64>().unwrap();
                let b = tokens.next().unwrap().parse::<u64>().unwrap();
                assert!(tokens.next().is_none());
                ranges.push(Range { a, b });
            }
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    let ranges = ranges;

    let mut part1 = 0u64;

    for id in ids {
        for range in ranges.iter() {
            if id >= range.a && id <= range.b {
                part1 += 1;
                break;
            }
        }
    }
    output.part1(part1.to_string());

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_unstable_by_key(|r| r.a);

    let mut merged_ranges = Vec::<Range>::new();

    let mut cur_range = sorted_ranges[0];
    for range in sorted_ranges[1..].iter() {
        if range.a > cur_range.b {
            merged_ranges.push(cur_range);
            cur_range = *range;
        } else {
            cur_range.b = cur_range.b.max(range.b);
        }
    }
    merged_ranges.push(cur_range);

    let mut part2 = 0u64;
    for range in merged_ranges {
        part2 += range.b - range.a + 1;
    }
    output.part2(part2.to_string());
}
