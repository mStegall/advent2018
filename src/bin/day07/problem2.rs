use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("input.txt");

    let line_re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let pairs: Vec<(char, char)> = data
        .lines()
        .filter_map(|line| {
            line_re.captures(line).map(|caps| {
                (
                    caps.get(1).unwrap().as_str().chars().next().unwrap(),
                    caps.get(2).unwrap().as_str().chars().next().unwrap(),
                )
            })
        })
        .collect();

    let mut steps_map: HashMap<char, HashSet<char>> = HashMap::new();

    for (dependency, dependent) in pairs {
        steps_map.entry(dependency).or_default();
        steps_map.entry(dependent).or_default().insert(dependency);
    }

    let mut steps: Vec<char> = vec![];

    let mut free_workers: Vec<()> = vec![(), (), (), (), ()];
    let mut workers: Vec<(char, isize)> = vec![];

    let mut ticks = -1;

    while steps_map.len() > 0 || (ticks == 0 || workers.len() > 0) {

        workers = workers
            .clone()
            .iter()
            .filter_map(|(c, d)| {
                if d - 1 as isize == 0 {
                    steps.push(*c);
                    steps_map = steps_map
                        .clone()
                        .into_iter()
                        .map(|mut s| {
                            s.1.remove(c);
                            s
                        })
                        .collect();
                    free_workers.push(());
                    return None;
                } else {
                    return Some((*c, d - 1));
                }
            })
            .collect();

        let mut next_steps: Vec<_> = steps_map
            .clone()
            .into_iter()
            .map(|(step, dependencies)| (step, dependencies.len()))
            .filter(|s| s.1 == 0)
            .map(|s| s.0)
            .collect();
        next_steps.sort();
        next_steps.reverse();

        while free_workers.len() > 0 && next_steps.len() > 0 {
            let next_step = next_steps.pop().unwrap();
            steps_map.remove(&next_step);

            free_workers.pop();
            let additional = next_step as u32 - 'A' as u32 + 1;
            workers.push((next_step, 60 + additional as isize));
        }

        ticks += 1;
    }

    println!("{:?}", steps.into_iter().collect::<String>());
    println!("{ticks}");
}
