fn main() {
    // let foo = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let foo = vec![vec![1, 4], vec![2, 3]];

    println!("{:?}", merge(foo));
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a.cmp(b));
    let mut res = vec![intervals[0].clone()];

    for i in 1..intervals.len() {
        let l = intervals[i].first().unwrap();
        let r = res.last().unwrap()[1];
        if l > &r {
            res.push(intervals[i].to_vec());
        } else {
            (*res.last_mut().unwrap())[1] = std::cmp::max(intervals[i][1], res.last().unwrap()[1]);
        }
    }

    res
}
