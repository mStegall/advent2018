use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("input.txt");

    let line_re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let pairs: Vec<(&str, &str)> = data
        .lines()
        .filter_map(|line| {
            line_re
                .captures(line)
                .map(|caps| (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
        })
        .collect();

    let mut steps_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (dependency, dependent) in pairs {
        steps_map.entry(dependency).or_default();
        steps_map.entry(dependent).or_default().insert(dependency);
    }

    let mut steps: Vec<&str> = vec![];

    while steps_map.len() > 0 {
        println!("{steps_map:?}");
        let mut next_steps = steps_map
            .clone()
            .into_iter()
            .map(|(step, dependencies)| (step, dependencies.len()))
            .filter(|s| s.1 == 0)
            .map(|s| s.0)
            .collect::<Vec<_>>();
        println!("{next_steps:?}");
        next_steps.sort();
        let next_step = next_steps[0];
        steps.push(dbg!(next_step));
        steps_map.remove(next_step);
        steps_map = steps_map
            .into_iter()
            .map(|mut s| {
                s.1.remove(next_step);
                s
            })
            .collect();
    }

    println!("{:?}", steps.join(""))
}
