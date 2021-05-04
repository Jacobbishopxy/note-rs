fn main() {
    let nums = vec![1, 2, 3];

    println!("{:?}", subsets(nums));
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut r = Res {
        t: vec![],
        res: vec![],
    };

    r.dfs(0, &nums);

    r.res
}

struct Res {
    t: Vec<i32>,
    res: Vec<Vec<i32>>,
}

impl Res {
    // cur: 当前位置
    // nums: 原数组引用
    fn dfs(&mut self, cur: i32, nums: &Vec<i32>) {
        if cur == nums.len() as i32 {
            // 记录答案
            self.res.push(self.t.clone());
            return;
        }
        // 考虑选择当前位置
        self.t.push(nums[cur as usize]);
        self.dfs(cur + 1, nums);
        // 考虑不选择当前位置
        self.t.pop();
        self.dfs(cur + 1, nums);
    }
}
