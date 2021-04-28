fn main() {
    let foo = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    println!("result: {}", trap(foo));
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut res = 0;

    let (mut left, mut right) = (0, height.len() as i32 - 1);
    let (mut left_max, mut right_max) = (0, 0);

    while left < right {
        if height[left as usize] < height[right as usize] {
            if height[left as usize] >= left_max {
                left_max = height[left as usize];
            } else {
                res += left_max - height[left as usize];
            }
            left += 1;
        } else {
            if height[right as usize] >= right_max {
                right_max = height[right as usize];
            } else {
                res += right_max - height[right as usize];
            }
            right -= 1;
        }
    }

    res
}
