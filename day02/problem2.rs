use std::collections::HashSet;

fn main() {
    let data = include_str!("input.txt");
    let mut box_ids: HashSet<String> = HashSet::new();
    let mut found_box_1 = String::new();
    let mut found_box_2 = String::new();
    'main: for box_id in data.lines() {
        for box_id_2 in box_ids.iter(){
            let mut missed_one = false;
            let box_string = box_id.to_string();
            let too_different = box_string.chars().zip(box_id_2.chars()).find(|(a, b)| {
                if a!=b && !missed_one{
                    missed_one=true;
                    return false;
                }
                if a!=b && missed_one{
                    return true;
                }
                return false;
            });

            // println!("{:?}", zipped)
            // println!("{:?}", too_different)
            if too_different.is_none(){
                found_box_1 = box_id.to_string();
                found_box_2 = box_id_2.clone();
                println!("{} {}", box_id, box_id_2);
                break 'main;
            }
        }

        box_ids.insert(box_id.to_string());
    }

    let common: String = found_box_1
        .chars()
        .zip(found_box_2.chars())
        .filter_map(|(a, b)| if a == b {Some(a)} else {None})
        .collect();
    println!("{}", common);
}