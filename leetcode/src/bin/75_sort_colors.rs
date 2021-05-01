use std::ptr::swap;

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    let mut nums1 = nums.clone();

    sort_colors(&mut nums);
    sort_colors2(&mut nums1);

    println!("{:?}", nums);
    println!("{:?}", nums1);
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut ptr0, mut ptr1) = (0, 0);

    for i in 0..nums.len() {
        if nums[i] == 1 {
            unsafe {
                swap(&mut nums[i], &mut nums[ptr1]);
            }
            ptr1 += 1;
        } else if nums[i] == 0 {
            unsafe {
                swap(&mut nums[i], &mut nums[ptr0]);
            }
            if ptr0 < ptr1 {
                unsafe {
                    swap(&mut nums[i], &mut nums[ptr1]);
                }
            }
            ptr0 += 1;
            ptr1 += 1;
        }
    }
}

pub fn sort_colors2(nums: &mut Vec<i32>) {
    let (mut ptr0, mut ptr1) = (0, 0);

    for i in 0..nums.len() {
        let num = nums[i];
        nums[i] = 2;
        if num < 2 {
            nums[ptr1] = 1;
            ptr1 += 1;
        }
        if num < 1 {
            nums[ptr0] = 0;
            ptr0 += 1;
        }
    }
}
