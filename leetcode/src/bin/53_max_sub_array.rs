use std::cmp::max;

fn main() {
    let foo = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let bar = foo.clone();

    // `max_sub_array` consumes less memory
    println!("{:?}", max_sub_array(foo));
    println!("{:?}", max_sub_array_2(bar));
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut pre, mut max_ans) = (0, nums[0]);

    for i in nums {
        pre = max(pre + i, i);
        max_ans = max(max_ans, pre);
    }

    max_ans
}

struct Status {
    l_sum: i32,
    r_sum: i32,
    m_sum: i32,
    i_sum: i32,
}

fn push_up(l: Status, r: Status) -> Status {
    let i_sum = l.i_sum + r.i_sum;
    let l_sum = max(l.l_sum, l.i_sum + r.l_sum);
    let r_sum = max(r.r_sum, r.i_sum + l.r_sum);
    let m_sum = max(max(l.m_sum, r.m_sum), l.r_sum + r.l_sum);

    Status {
        l_sum,
        r_sum,
        m_sum,
        i_sum,
    }
}

fn get(nums: &mut Vec<i32>, l: i32, r: i32) -> Status {
    if l == r {
        return Status {
            l_sum: nums[l as usize],
            r_sum: nums[l as usize],
            m_sum: nums[l as usize],
            i_sum: nums[l as usize],
        };
    }

    let m = (l + r) >> 1;

    let l_sub = get(nums, l, m);
    let r_sub = get(nums, m + 1, r);

    push_up(l_sub, r_sub)
}

// divide and conquer
fn max_sub_array_2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len() as i32;
    let nums = &mut nums;
    get(nums, 0, len - 1).m_sum
}
