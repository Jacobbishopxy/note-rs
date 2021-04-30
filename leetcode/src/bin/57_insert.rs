use std::cmp::{max, min};

fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];

    println!("{:?}", insert(intervals, new_interval));
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut left, mut right) = (new_interval[0], new_interval[1]);
    let mut placed = false;
    let mut res = vec![];

    for i in intervals.iter() {
        if i[0] > right {
            if !placed {
                res.push(vec![left, right]);
                placed = true;
            }
            res.push(i.clone());
        } else if i[1] < left {
            res.push(i.clone());
        } else {
            left = min(left, i[0]);
            right = max(right, i[1]);
        }
    }

    if !placed {
        res.push(vec![left, right]);
    }

    res
}
