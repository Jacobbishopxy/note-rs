fn main() {
    let m = 10;
    let n = 10;

    println!("{:?}", unique_paths(m, n));
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut ans = 1_i64;
    for i in 1..std::cmp::min(m, n) as i64 {
        ans = ans * ((n + m) as i64 - i - 1) / i;
    }
    ans as i32
}
