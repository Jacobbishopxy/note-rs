use std::cmp::max;

fn main() {
    // let heights = vec![2, 1, 5, 6, 2, 3];
    let heights = vec![2, 4];

    println!("{:?}", largest_rectangle_area(heights));
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    // 用于记录哨兵位置
    let (mut left, mut right) = (vec![0_i32; n], vec![n as i32; n]);
    // 单调栈
    let mut mono_stack: Vec<usize> = vec![];

    for i in 0..n {
        while !mono_stack.is_empty()
            && heights[mono_stack.last().unwrap().clone() as usize] >= heights[i]
        {
            right[mono_stack.last().unwrap().clone() as usize] = i as i32;
            mono_stack.pop();
        }

        left[i] = if mono_stack.is_empty() {
            -1
        } else {
            mono_stack.last().unwrap().clone() as i32
        };
        mono_stack.push(i);
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, (right[i] - left[i] - 1) * heights[i]);
    }
    ans
}
