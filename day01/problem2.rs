use std::collections::HashSet;

fn main() {
    let data = include_str!("input.txt");

    let mut frequency = 0;
    let mut observed_frequencies = HashSet::new();

    let mut nums =  data.lines().filter_map(|s| s.parse::<i32>().ok()).cycle();
    loop {
        let amount = nums.next().unwrap();
        frequency += amount;

        if observed_frequencies.contains(&frequency) {
            break;
        }

        observed_frequencies.insert(frequency);
    }

    println!("Duplicate found: {}", frequency);
}