use proconio::input;
use std::{collections::{BTreeMap, HashSet}, vec};

fn main() {
    input! {
        n: usize,
    }
    let mut skill: HashSet<usize> = HashSet::new();
    let mut stack: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 1..n+1 {
        input! {
            a: usize,
            b: usize,
        }

        if (a, b) == (0, 0) {
            skill.insert(i);
        } else if skill.contains(&a) || skill.contains(&b) {
            skill.insert(i);
        } else {
            if a > i {
                stack.entry(a).or_insert(vec![]).push(i);
            }
            if b > i {
                stack.entry(b).or_insert(vec![]).push(i);
            }
        }

        let v = stack.get(&i).unwrap_or(&vec![]).clone();
        if skill.contains(&i) {
            for val in v {
                skill.insert(val);
            }
        }
    }
    println!("{}", skill.len());
}
