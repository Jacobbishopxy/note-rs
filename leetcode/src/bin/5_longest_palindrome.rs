fn main() {
    let foo = String::from("a");

    println!("result: {:?}", longest_palindrome(foo));
}

pub fn longest_palindrome(s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let mut lp: Vec<char> = vec![];

    loop {
        for idx in 0..=s.len() {
            if check_if_palindrome(&s[0..idx]) && lp.len() < idx {
                lp = (s[0..idx]).iter().cloned().collect();
            }
        }

        s.drain(..1);

        if s.len() <= lp.len() {
            break;
        }
    }

    lp.into_iter().collect()
}

pub fn check_if_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}
