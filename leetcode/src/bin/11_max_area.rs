use std::cmp::{max, min};
fn main() {
    // let foo = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let foo = vec![1, 1];

    println!("result: {}", max_area(foo));
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0 as usize, (height.len() - 1) as usize);

    if right == 0 {
        return 0;
    }
    let mut m = min(height[left], height[right]) * (right - left) as i32;

    while left < right {
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }

        m = max(m, min(height[left], height[right]) * (right - left) as i32);
    }

    m
}
