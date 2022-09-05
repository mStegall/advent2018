use regex::Regex;
use std::collections::HashMap;

fn count_minutes(mut map: [i32; 60], mut start: usize, end: usize) -> [i32; 60] {
    while start < end {
        map[start] = map[start] + 1;
        start = start + 1;
    }

    map
}

pub fn main() {
    let data = include_str!("input.txt");

    let shift_change_re =
        Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}] Guard #(\d+) begins shift").unwrap();
    let wake_up_re = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})] wakes up").unwrap();
    let fall_asleep_re = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})] falls asleep").unwrap();

    let mut guard_number_active = 0;
    let mut awake = true;
    let mut asleep_time = 0;

    let mut guards: HashMap<i32, [i32; 60]> = HashMap::new();

    let mut lines: Vec<&str> = data.lines().collect();
    lines.sort_by(|a, b| a[0..18].cmp(&b[0..18]));

    for line in lines {
        if shift_change_re.is_match(line) {
            if !awake {
                match guards.get(&guard_number_active) {
                    Some(a) => {
                        guards.insert(guard_number_active, count_minutes(*a, asleep_time, 59));
                    }
                    None => {
                        guards.insert(guard_number_active, count_minutes([0; 60], asleep_time, 59));
                    }
                }

                awake = true;
                asleep_time = 0;
            }
            let caps = shift_change_re.captures(line).unwrap();
            let guard_num_string = caps.get(1).unwrap().as_str();
            guard_number_active = guard_num_string.parse::<i32>().unwrap();
        } else if wake_up_re.is_match(line) {
            if !awake {
                let caps = wake_up_re.captures(line).unwrap();
                let time = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                match guards.get(&guard_number_active) {
                    Some(a) => {
                        guards.insert(guard_number_active, count_minutes(*a, asleep_time, time));
                    }
                    None => {
                        guards.insert(guard_number_active, count_minutes([0; 60], asleep_time, time));
                    }
                }

                awake = true;
                asleep_time = 0;
            }
        } else if fall_asleep_re.is_match(line) {
            let caps = fall_asleep_re.captures(line).unwrap();
            let time = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            asleep_time = time;
            awake = false;
        }
    }


    let mut best_guard_id = 0;
    let mut max_sleep = 0;
 
    for (key, val) in guards.iter(){
        let total_sleep: i32 = val.iter().sum();
        if total_sleep > max_sleep{
            max_sleep = total_sleep;
            best_guard_id = *key;
        }
    }

    let best_guard = guards.get(&best_guard_id).unwrap();

    let best_minute = best_guard.iter().enumerate().max_by_key(|a| a.1).unwrap();

    let answer = best_minute.0 as isize * best_guard_id as isize;
    println!("{answer}")
}
