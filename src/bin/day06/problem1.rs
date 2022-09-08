use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data = include_str!("input.txt");

    let points: Vec<(isize, isize)> = data
        .split("\n")
        .filter_map(|s| {
            let mut split = s.split(", ").filter_map(|i| i.parse().ok());
            split.next_tuple()
        })
        .collect();

    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    println!("Max X: {max_x}\nMax Y: {max_y}\n{points:?}");

    let mut infinite_ids: HashSet<usize> = HashSet::new();
    let mut board: HashMap<usize, usize> = HashMap::new();
    
    for x in 1..max_x {
        for y in 1..max_y {
            let distances = points.iter().enumerate().map(|( id,(x1,y1))| (id, (x - x1).abs() + (y - y1).abs()));
            let min_distance = distances.clone().min_by_key(|i| i.1).unwrap();
            if distances.filter(|i| i.1 == min_distance.1).count() > 1{
                continue;
            }

            if x == 1 || y == 1 || x == max_x || y == max_y {
                infinite_ids.insert(min_distance.0);
            } else {
                board.entry(min_distance.0).and_modify(|i| *i += 1).or_insert(1);
            }
        }
    }

    println!("{infinite_ids:?}");
    dbg!(board
        .iter()
        .filter(|p| !infinite_ids.contains(p.0))
        .max_by_key(|p| p.1));
}
