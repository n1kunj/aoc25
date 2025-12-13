use std::collections::HashMap;

use crate::day_output::DayOutput;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut machines = Vec::<Machine>::new();
    for line in input.lines() {
        let mut lights = Vec::<bool>::new();
        let mut buttons = Vec::<Vec<usize>>::new();
        let mut joltage = Vec::<usize>::new();
        for token in line.split_whitespace() {
            let contents = &token[1..token.len() - 1];
            match token.chars().next().unwrap() {
                '[' => {
                    for c in contents.chars() {
                        let l = match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        };
                        lights.push(l);
                    }
                }
                '(' => {
                    let mut button = Vec::<usize>::new();
                    for l in contents.split(",") {
                        button.push(l.parse().unwrap());
                    }
                    buttons.push(button);
                }
                '{' => {
                    for l in contents.split(",") {
                        joltage.push(l.parse().unwrap());
                    }
                }
                _ => panic!(),
            }
        }
        machines.push(Machine {
            lights,
            buttons,
            joltage,
        });
    }

    let mut part1 = 0usize;
    for m in machines.iter() {
        let mut best_states = HashMap::<Vec<bool>, usize>::new();

        let initial_state = vec![false; m.lights.len()];
        let mut cur_states = Vec::<Vec<bool>>::new();
        let mut pushes = 0usize;

        cur_states.push(initial_state);

        best_states.insert(cur_states[0].clone(), pushes);

        let mut next_states = Vec::<Vec<bool>>::new();
        let pushes = 'outer: loop {
            pushes += 1;
            for state in cur_states.drain(..) {
                for b in m.buttons.iter() {
                    let mut new_state = state.clone();
                    for l in b {
                        new_state[*l] = !state[*l];
                    }
                    if new_state == m.lights {
                        break 'outer pushes;
                    }
                    if !best_states.contains_key(&new_state) {
                        best_states.insert(new_state.clone(), pushes);
                        next_states.push(new_state);
                    }
                }
            }
            cur_states.append(&mut next_states);
        };
        part1 += pushes;
    }
    output.part1(part1.to_string());

    let mut part2 = 0usize;
    for m in machines.iter() {
        let js = &m.joltage;
        let mut jbs = vec![Vec::<usize>::new(); js.len()];
        for (i, b) in m.buttons.iter().enumerate() {
            for j in b {
                jbs[*j].push(i);
            }
        }
        println!("{js:?}");
        let jbs = jbs;

        fn next_valid_range(
            m: &Machine,
            joltages: &[usize],
            missings: &mut [usize],
            next_bidx: usize,
        ) -> (usize, usize) {
            let mut minb = 0usize;
            let mut maxb = usize::MAX;

            for jidx in m.buttons[next_bidx].iter() {
                let j = m.joltage[*jidx];
                let curj = joltages[*jidx];
                let missing = missings[*jidx];
                assert!(missing >= 1);
                assert!(curj <= j);
                let delta = j - curj;
                if missing == 1 {
                    minb = minb.max(delta);
                }
                maxb = maxb.min(delta);
                // }
            }

            (minb, maxb)
        }

        let mut pushes = vec![Option::<usize>::None; m.buttons.len()];
        let mut joltages = vec![0usize; m.joltage.len()];
        let mut missings = jbs.iter().map(|jb| jb.len()).collect::<Vec<_>>();
        let mut best = usize::MAX;
        fn recurse(
            m: &Machine,
            pushes: &mut [Option<usize>],
            joltages: &mut [usize],
            missings: &mut [usize],
            pushed: usize,
            cur_pushes: usize,
            best: &mut usize,
        ) {
            let unpushed = pushes.len() - pushed;
            if unpushed == 0 {
                if cur_pushes < *best {
                    *best = cur_pushes;
                    dbg!(*best);
                }
                return;
            }

            let mut best_bidx = usize::MAX;
            let mut best_range = usize::MAX;
            let mut best_range_bidx = (0usize, usize::MAX);
            for (bidx, push) in pushes.iter().enumerate() {
                if push.is_some() {
                    continue;
                }
                let (bmin, bmax) = next_valid_range(m, joltages, missings, bidx);
                if bmin > bmax {
                    return;
                }
                let delta = bmax - bmin;
                if delta < best_range {
                    best_bidx = bidx;
                    best_range = delta;
                    best_range_bidx = (bmin, bmax);
                }
            }

            let (bmin, bmax) = best_range_bidx;
            for j in m.buttons[best_bidx].iter() {
                missings[*j] -= 1;
            }
            for i in bmin..=bmax {
                if unpushed == pushes.len() {
                    println!("{i}");
                }

                if cur_pushes + i >= *best {
                    break;
                }
                pushes[best_bidx] = Some(i);
                for j in m.buttons[best_bidx].iter() {
                    joltages[*j] += i;
                }
                recurse(
                    m,
                    pushes,
                    joltages,
                    missings,
                    pushed + 1,
                    cur_pushes + i,
                    best,
                );
                for j in m.buttons[best_bidx].iter() {
                    joltages[*j] -= i;
                }
                pushes[best_bidx] = None;
            }
            for j in m.buttons[best_bidx].iter() {
                missings[*j] += 1;
            }
        }

        recurse(
            m,
            &mut pushes,
            &mut joltages,
            &mut missings,
            0,
            0,
            &mut best,
        );
        dbg!(best);
        part2 += best;
    }
    output.part2(part2.to_string());
}
