fn main() {
    // let foo = vec![-1, 0, 1, 2, -1, -4];
    // let foo = vec![-4, -1, -1, 0, 1, 2];
    let foo = vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6];

    println!("result: {:?}", three_sum(foo));
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    if nums.len() < 3 {
        return res;
    }

    let mut nums = nums;
    nums.sort();

    for idx in 0..nums.len() {
        if idx > 0 && nums[idx] == nums[idx - 1] {
            continue;
        }
        let mut idx_right: usize = nums.len() - 1;
        let target = -nums[idx];

        for idx_left in (idx + 1)..nums.len() {
            if idx_left > idx + 1 && nums[idx_left] == nums[idx_left - 1] {
                continue;
            }

            while idx_left < idx_right && nums[idx_left] + nums[idx_right] > target {
                idx_right -= 1;
            }

            if idx_left == idx_right {
                break;
            }

            if nums[idx_left] + nums[idx_right] == target {
                res.push(vec![nums[idx], nums[idx_left], nums[idx_right]]);
            }
        }
    }

    res
}
