fn main() {
    let v = vec![-1, -2, -3, -4, -5];
    let t = -8;

    println!("result: {:?}", two_sum(v, t));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    let mut first: i32 = 0;
    let mut second: i32 = 0;

    for idx in 0..nums.len() {
        if let [head, tail @ ..] = &nums[..] {
            first = idx as i32;
            second += 1;
            for (idxx, &nn) in tail.iter().enumerate() {
                if head + nn == target {
                    second += idxx as i32;
                    return vec![first, second];
                }
            }
            nums.drain(..1);
        }
    }

    vec![0, 0]
}
