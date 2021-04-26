fn main() {
    println!("{:?}", generate_parenthesis(3));
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    S::new(n).generate_parenthesis()
}
pub struct S {
    pub n: i32,
    res: Vec<String>,
}

impl S {
    pub fn new(n: i32) -> Self {
        S { n, res: vec![] }
    }

    fn dfs(&mut self, left: i32, right: i32, str: &mut String) {
        // 返回的字符串长度永远是 n 的两倍。答案放入 res，结束递归
        if str.chars().count() == (2 * self.n) as usize {
            self.res.push(str.as_str().to_string());
            return;
        }

        // 左括号不满时
        if left < self.n as i32 {
            str.push('(');
            self.dfs(left + 1, right, str);
            str.pop();
        }

        // 右括号少于左括号时
        if right < left {
            str.push(')');
            self.dfs(left, right + 1, str);
            str.pop();
        }
    }

    pub fn generate_parenthesis(&mut self) -> Vec<String> {
        self.dfs(0, 0, &mut "".to_string());

        self.res.clone()
    }
}
