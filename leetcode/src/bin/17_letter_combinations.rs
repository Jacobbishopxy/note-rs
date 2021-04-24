use std::collections::HashMap;

fn main() {
    let map = [
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<char, &str>>();

    let lc = LC(map);

    let foo = "23".to_string();

    println!("result: {:?}", lc.letter_combinations(foo));
}

pub struct LC(HashMap<char, &'static str>);

impl LC {
    fn recursive<'a>(&self, s: &str, res: &mut Vec<String>, buf: &mut Vec<&'a str>) {
        // println!("0. {:?} | {:?}", res, buf);
        let current = if let Some(i) = s.chars().nth(0) {
            i
        } else {
            // 当 res 为空且 buf 不为空时，将 buf 所有数据传入 res 并结束 recursive
            if !buf.is_empty() {
                res.push(buf.iter().map(|&s| s).collect());
            }
            return;
        };
        // 当前数字下所对应的字符串
        let mapping = self.0[&current];

        for i in 0..mapping.len() {
            // buf 缓存字符串切片第 i 个子字符串
            buf.push(&mapping[i..=i]);
            // 递归：当前 s 的剩余子字符串
            // println!("1. {:?} | {:?}", res, buf);
            self.recursive(&s[1..], res, buf);
            // buf 缓存移除以完成的第 i 个子字符串
            buf.pop();
            // println!("2. {:?} | {:?}", res, buf);
        }
    }

    pub fn letter_combinations(&self, digits: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];

        self.recursive(&digits, &mut res, &mut vec![]);

        res
    }
}
