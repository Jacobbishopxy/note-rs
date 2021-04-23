fn main() {
    let s = String::from("aaaab");
    let p = String::from("a*..");

    println!("result: {}", is_match(s, p));
}

pub fn is_match(s: String, p: String) -> bool {
    fn is_match_str(s: &str, p: &str) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }

        let m = { s.len() > 0 && (s.as_bytes()[0] == p.as_bytes()[0] || p.as_bytes()[0] == 46) };
        if p.len() >= 2 && p.as_bytes()[1] == 42 {
            return is_match_str(s, &p[2..]) || (m && is_match_str(&s[1..], p));
        }

        m && is_match_str(&s[1..], &p[1..])
    }

    is_match_str(&s, &p)
}
