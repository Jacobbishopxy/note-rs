use std::iter::FromIterator;

fn main() {
    let l1 = vec![3];
    let l2 = vec![-2, -1];

    println!("result: {}", find_median_sorted_arrays(l1, l2));
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut nums1, mut nums2) = (nums1, nums2);

    nums1.append(&mut nums2);

    nums1.sort();

    let mut cache = std::collections::VecDeque::from_iter(nums1);

    loop {
        if cache.len() > 2 {
            cache.pop_front();
            cache.pop_back();
        } else if cache.len() == 2 {
            let c = cache.pop_front().unwrap() + cache.pop_back().unwrap();
            return (c as f64) / (2 as f64);
        } else {
            let c = cache.pop_back().unwrap();
            return c as f64;
        }
    }
}
