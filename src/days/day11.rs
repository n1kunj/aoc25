use std::collections::HashMap;

use crate::day_output::DayOutput;

#[allow(unused)]
struct Dev {
    name: String,
    idx: usize,
    outputs: Vec<usize>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut dev_list = Vec::<&str>::new();
    let mut dev_to_outputs = HashMap::<&str, (usize, Vec<&str>)>::new();
    for (i, line) in input.lines().enumerate() {
        let mut tokens = line.split(" ");
        let name = tokens.next().unwrap();
        let name = &name[..name.len() - 1];
        let mut outputs = Vec::<&str>::new();
        for token in tokens {
            outputs.push(token);
        }
        dev_list.push(name);
        dev_to_outputs.insert(name, (i, outputs));
    }
    dev_to_outputs.insert("out", (dev_list.len(), vec![]));
    dev_list.push("out");

    let mut devs = Vec::<Dev>::new();
    for (idx, name) in dev_list.iter().enumerate() {
        let mut outputs = Vec::<usize>::new();
        for output_name in dev_to_outputs[name].1.iter() {
            outputs.push(dev_to_outputs[output_name].0);
        }
        devs.push(Dev {
            name: name.to_string(),
            idx,
            outputs,
        });
    }
    let devs = devs;

    let you = dev_to_outputs.get("you");
    if let Some(you) = you {
        let you = you.0;

        let mut found_paths = 0usize;
        let mut cur_heads = Vec::<(usize, usize)>::new();
        cur_heads.push((you, 1));

        let mut next_heads = HashMap::<usize, usize>::new();
        loop {
            for cur in cur_heads.drain(..) {
                let outputs = &devs[cur.0].outputs;
                if outputs.is_empty() {
                    found_paths += cur.1;
                    continue;
                }
                for output in outputs {
                    *next_heads.entry(*output).or_insert(0) += cur.1;
                }
            }
            cur_heads.extend(next_heads.drain());
            if cur_heads.is_empty() {
                break;
            }
        }
        output.part1(found_paths.to_string());
    }

    let svr = dev_to_outputs.get("svr");
    let fft = dev_to_outputs.get("fft");
    let dac = dev_to_outputs.get("dac");

    if let Some(svr) = svr
        && let Some(fft) = fft
        && let Some(dac) = dac
    {
        let svr = svr.0;
        let fft = fft.0;
        let dac = dac.0;

        #[derive(PartialEq, Eq, Hash, Copy, Clone)]
        enum Seen {
            None,
            Dac,
            Fft,
            Both,
        }

        let mut found_paths = 0usize;
        let mut cur_heads = Vec::<(usize, Seen, usize)>::new();
        cur_heads.push((svr, Seen::None, 1));

        let mut next_heads = HashMap::<(usize, Seen), usize>::new();
        loop {
            for cur in cur_heads.drain(..) {
                let outputs = &devs[cur.0].outputs;
                if outputs.is_empty() {
                    if cur.1 == Seen::Both {
                        found_paths += cur.2;
                    }
                    continue;
                }
                for output in outputs {
                    let new_seen = if *output == fft {
                        match cur.1 {
                            Seen::None => Seen::Fft,
                            Seen::Dac => Seen::Both,
                            _ => panic!(),
                        }
                    } else if *output == dac {
                        match cur.1 {
                            Seen::None => Seen::Dac,
                            Seen::Fft => Seen::Both,
                            _ => panic!(),
                        }
                    } else {
                        cur.1
                    };
                    *next_heads.entry((*output, new_seen)).or_insert(0) += cur.2;
                }
            }
            cur_heads.extend(next_heads.drain().map(|(k, v)| (k.0, k.1, v)));
            if cur_heads.is_empty() {
                break;
            }
        }
        output.part2(found_paths.to_string());
    }
}
