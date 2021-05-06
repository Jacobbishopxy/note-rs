fn main() {
    let s = "12".to_string();
    println!("result: {}", num_decodings(s));
}

pub fn num_decodings(s: String) -> i32 {
    let n = s.chars().count();

    let (mut a, mut b, mut c) = (0, 1, 0);

    for i in 1..=n {
        c = 0;

        if s.chars().nth(i - 1).unwrap() != '0' {
            c += b;
        }
        if i > 1
            && s.chars().nth(i - 2).unwrap() != '0'
            && (s.chars().nth(i - 2).unwrap().to_digit(10).unwrap() * 10
                + s.chars().nth(i - 1).unwrap().to_digit(10).unwrap())
                <= 26
        {
            c += a;
        }
        a = b;
        b = c;
    }

    c
}
