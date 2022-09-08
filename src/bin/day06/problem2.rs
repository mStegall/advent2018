use itertools::Itertools;

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

    let count = itertools::iproduct!(1..max_x,1..max_y)
        .map(|(x, y)| {
            points
                .iter()
                .map(|(x1, y1)| (x - x1).abs() + (y - y1).abs())
                .sum::<isize>()
        })
        .filter(|d| *d < 10000)
        .count();

    println!("{count}");
}
