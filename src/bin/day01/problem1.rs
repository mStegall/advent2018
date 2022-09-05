pub fn main() {
        let data = include_str!("input.txt");
    
        let sum = data.lines().filter_map(|s| s.parse::<i32>().ok()).sum::<i32>();
    
        println!("Final Frequency {}", sum)
    }
