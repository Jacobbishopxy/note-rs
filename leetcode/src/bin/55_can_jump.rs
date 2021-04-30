use std::cmp::max;

fn main() {
    let foo = vec![2, 3, 1, 1, 4];
    println!("{}", can_jump(foo));

    let bar = vec![3, 2, 1, 0, 4];
    println!("{}", can_jump(bar));
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut m = 0;
    for i in 0..nums.len() {
        if i > m {
            return false;
        }
        m = max(m, i + nums[i] as usize);
    }
    true
}
