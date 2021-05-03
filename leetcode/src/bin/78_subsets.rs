fn main() {
    let nums = vec![1, 2, 3];

    println!("{:?}", subsets(nums));
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut r = Res {
        t: vec![],
        res: vec![],
    };

    r.dfs(0, &mut nums);

    r.res
}

struct Res {
    t: Vec<i32>,
    res: Vec<Vec<i32>>,
}

impl Res {
    fn dfs(&mut self, cur: i32, nums: &mut Vec<i32>) {
        if cur == nums.len() as i32 {
            self.res.push(self.t.clone());
            return;
        }
        self.t.push(nums[cur as usize]);
        self.dfs(cur + 1, nums);
        self.t.pop();
        self.dfs(cur + 1, nums);
    }
}
