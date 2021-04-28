fn main() {
    let foo = vec![1, 2, 3];

    println!("{:?}", permute(foo));
}

/// 作者：xue-bu-wan-bu-gai-ming
/// 链接：https://leetcode-cn.com/problems/permutations/solution/qi-ta-ren-de-rustxie-fa-du-bu-tai-hao-by-8l25/
/// 来源：力扣（LeetCode）
/// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = (2..=nums.len()).product();
    let mut res = Vec::with_capacity(len);

    recursive(&mut nums, 0, &mut res);

    res
}

pub fn recursive(v: &mut Vec<i32>, idx: usize, res: &mut Vec<Vec<i32>>) {
    if idx == v.len() {
        res.push(v.clone());
        return;
    }

    for i in idx..v.len() {
        unsafe {
            std::ptr::swap(&mut v[idx], &mut v[i]);
        }

        recursive(v, idx + 1, res);

        unsafe {
            std::ptr::swap(&mut v[i], &mut v[idx]);
        }
    }
}
