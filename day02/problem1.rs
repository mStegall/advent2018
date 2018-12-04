use std::collections::HashMap;

fn main() {
    let data = include_str!("input.txt");

    let (total_2,total_3) = data.lines().map(|box_id| {
        let mut letters = HashMap::new();

        box_id.chars().for_each(|ch| {
            *letters.entry(ch).or_insert(0) += 1;
        });

        let has_2 = letters.values().find(|&&v| v == 2).is_some();
        let has_3 = letters.values().find(|&&v| v == 3).is_some();

        println!("has 2:{} has 3: {}", has_2, has_3);
        return (has_2, has_3)
    }).fold((0, 0), |(total_2, total_3), (has_2, has_3)| {
        let new_2 = if has_2 {total_2 + 1} else {total_2};
        let new_3 = if has_3 {total_3 + 1} else {total_3};
        return (new_2, new_3);
    });

    println!("total 2:{} total 3: {}", total_2, total_3);
    println!("checksum: {}", total_2 * total_3)
}