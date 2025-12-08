use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::day_output::DayOutput;

pub fn main(input: &str, output: &mut DayOutput) {
    let mut num_joins: Option<usize> = None;
    let mut boxes = Vec::<[isize; 3]>::new();
    for line in input.lines() {
        if num_joins.is_none() {
            num_joins = Some(line.parse().unwrap());
        } else {
            let mut tokens = line.split(",");
            let b = [
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
            ];
            assert!(tokens.next().is_none());
            boxes.push(b);
        }
    }
    let num_joins = num_joins.unwrap();
    let boxes = boxes;

    #[derive(Debug)]
    struct Dist {
        a: [isize; 3],
        b: [isize; 3],
        d: f32,
    }

    let mut dists = Vec::<Dist>::new();
    for (i, a) in boxes.iter().enumerate() {
        for b in boxes[i + 1..].iter() {
            let d = (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2);
            let d = f32::sqrt(d as f32);
            dists.push(Dist { a: *a, b: *b, d });
        }
    }
    dists.sort_unstable_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

    #[derive(Debug)]
    struct Circuit {
        boxes: HashSet<[isize; 3]>,
    }
    {
        let joins = &dists[..num_joins];

        let mut box_to_circuit = HashMap::<[isize; 3], Rc<RefCell<Circuit>>>::new();
        for b in boxes.iter() {
            let mut c = Circuit {
                boxes: HashSet::new(),
            };
            c.boxes.insert(*b);
            let rc = Rc::new(RefCell::new(c));
            box_to_circuit.insert(*b, rc);
        }
        for j in joins.iter() {
            let a = box_to_circuit.get(&j.a).unwrap().clone();
            let b = box_to_circuit.get(&j.b).unwrap().clone();

            if !Rc::ptr_eq(&a, &b) {
                a.borrow_mut().boxes.extend(b.borrow().boxes.iter());
                for olds in b.borrow().boxes.iter() {
                    box_to_circuit.insert(*olds, a.clone());
                }
            }
        }

        let mut circuits = box_to_circuit.values().collect::<Vec<_>>();
        circuits.sort_by_key(|c| c.borrow().boxes.len());
        circuits.dedup_by_key(|rc| rc.as_ptr());
        circuits.reverse();

        let part1 = circuits[0].borrow().boxes.len()
            * circuits[1].borrow().boxes.len()
            * circuits[2].borrow().boxes.len();

        output.part1(part1.to_string());
    }

    {
        let joins = &dists;

        let mut box_to_circuit = HashMap::<[isize; 3], Rc<RefCell<Circuit>>>::new();
        for b in boxes.iter() {
            let mut c = Circuit {
                boxes: HashSet::new(),
            };
            c.boxes.insert(*b);
            let rc = Rc::new(RefCell::new(c));
            box_to_circuit.insert(*b, rc);
        }
        for j in joins.iter() {
            let a = box_to_circuit.get(&j.a).unwrap().clone();
            let b = box_to_circuit.get(&j.b).unwrap().clone();

            if !Rc::ptr_eq(&a, &b) {
                a.borrow_mut().boxes.extend(b.borrow().boxes.iter());
                for olds in b.borrow().boxes.iter() {
                    box_to_circuit.insert(*olds, a.clone());
                }
            }
            if a.borrow().boxes.len() == boxes.len() {
                output.part2((j.a[0] * j.b[0]).to_string());
                break;
            }
        }
    }
}
