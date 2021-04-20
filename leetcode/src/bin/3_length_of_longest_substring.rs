fn main() {
    let foo = "pwwkew".to_string();

    println!("{}", length_of_longest_substring(foo));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max: i32 = 0;
    let mut res = s;

    loop {
        let mut remain = "".to_string();

        for (idx, i) in res.chars().enumerate() {
            if remain.contains(i) {
                let m = idx as i32;
                if m < max {
                    break;
                }
                max = m;
                break;
            }

            remain.push(i);

            if remain == res {
                max = remain.len() as i32;
            }
        }

        if (res.len() as i32) <= max {
            break;
        }
        res = res.chars().skip(1).collect();
    }

    max
}
