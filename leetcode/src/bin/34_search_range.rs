fn main() {
    let nums = vec![1, 3];
    let target = 1;

    println!("{:?}", search_range(nums, target));
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 1 && nums[0] == target {
        return vec![0, 0];
    }

    let mut res: Vec<i32> = vec![-1; 2];

    let mut l: i32 = 0;
    let mut r: i32 = nums.len() as i32 - 1;

    while l <= r && (res[0] == -1 || res[1] == -1) {
        if nums[l as usize] == target {
            res[0] = l;
        }

        if nums[r as usize] == target {
            res[1] = r;
        }

        if res[0] == -1 {
            l += 1;
        }
        if res[1] == -1 {
            r -= 1;
        }
    }

    res
}
