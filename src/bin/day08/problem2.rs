fn parse(mut nums: &[i32]) -> (&[i32], i32) {
    let mut packets = nums[0];
    let mut metas = nums[1];

    let mut sum = 0;

    nums = &nums[2..];

    let mut results: Vec<i32> = vec![0];
    if packets > 0 {
        while packets > 0 {
            let (next, meta_sum) = parse(&nums);
            nums = next;
            results.push(meta_sum);
            packets -= 1;
        }
        while metas > 0 {
            match results.get(nums[0] as usize) {
                Some(v) => sum += v,
                None => (),
            }
            nums = &nums[1..];
            metas -= 1;
        }
    } else {
        while metas > 0 {
            sum += nums[0];
            nums = &nums[1..];
            metas -= 1;
        }
    }

    return (nums, sum);
}

pub fn main() {
    let data = include_str!("input.txt");

    let nums_vec = data
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();

    let nums: &[i32] = nums_vec.as_slice();

    println!("{:?}", parse(nums))
}
