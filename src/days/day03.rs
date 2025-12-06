use crate::day_output::DayOutput;

struct Bank {
    batteries: Vec<(usize, u32)>,
}
pub fn main(input: &str, output: &mut DayOutput) {
    let mut banks = Vec::<Bank>::new();
    for line in input.lines() {
        let mut batteries = Vec::<(usize, u32)>::new();
        for (i, c) in line.chars().enumerate() {
            batteries.push((i, c.to_digit(10).unwrap()));
        }
        banks.push(Bank { batteries });
    }

    let calc_joltage = |bank: &Bank, n: usize| -> u64 {
        let num_batteries = bank.batteries.len();

        let mut last_bat: Option<usize> = None;
        let mut joltage = 0u64;

        for bat_idx in 0..n {
            let mut cur_best = match last_bat {
                Some(last_bat) => bank.batteries[last_bat + 1],
                None => bank.batteries[0],
            };
            for i in cur_best.0 + 1..(num_batteries - (n - 1) + bat_idx) {
                let bat = bank.batteries[i];
                if cur_best.1 < bat.1 {
                    cur_best = bat;
                }
            }
            last_bat = Some(cur_best.0);
            joltage += cur_best.1 as u64 * 10u64.pow(n as u32 - 1u32 - bat_idx as u32);
        }
        joltage
    };

    let part1 = banks.iter().map(|b| calc_joltage(b, 2)).sum::<u64>();
    let part2 = banks.iter().map(|b| calc_joltage(b, 12)).sum::<u64>();

    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
