fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;

    println!("{}", search(nums, target));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(v) = nums.iter().position(|&x| x == target) {
        return v as i32;
    }

    -1
}
