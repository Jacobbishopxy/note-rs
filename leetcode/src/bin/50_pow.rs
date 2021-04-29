fn main() {
    let x = 2.0;
    let n = -2147483648;

    println!("result: {}", my_pow(x, n));
}

pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1_f64;
    }

    let mut x = x;
    let mut n_abs = n.abs() as u32;
    let mut ans = 1_f64;

    while n_abs > 0 {
        if n_abs % 2 == 1 {
            ans *= x;
        }
        x *= x;
        n_abs /= 2
    }

    if n < 0 {
        1_f64 / ans
    } else {
        ans
    }
}
