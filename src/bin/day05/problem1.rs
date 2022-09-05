pub fn main() {
    let data = include_str!("input.txt");
    let mut chars: Vec<char> = data.chars().collect();
    let mut i = 0;
    while i < chars.len() - 1 {
        if (chars[i].is_ascii_lowercase() && chars[i].to_ascii_uppercase() == chars[i + 1])
            || (chars[i].is_ascii_uppercase() && chars[i].to_ascii_lowercase() == chars[i + 1])
        {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i = i - 1;
            }
        } else {
            i = i + 1;
        }
    }
    println!("{:?}", chars);
    println!("Final length: {}", chars.len())
}
