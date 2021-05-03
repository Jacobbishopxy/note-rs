use std::collections::HashMap;

fn main() {
    let s = String::from("ADOBECODEBANCDD");
    let t = String::from("ABC");

    println!("result: {}", min_window(s, t));

    let s = String::from("abc");
    let t = String::from("b");

    println!("result: {}", min_window(s, t));
}

// this solution causes timeout
pub fn min_window(s: String, t: String) -> String {
    let (mut ori, mut cnt) = (HashMap::new(), HashMap::new());

    for i in t.chars() {
        *ori.entry(i).or_insert(1) += 1;
    }

    let (mut len, mut l, mut ans_l, mut ans_r) = (i32::MAX, 0, -1, -1);

    let check = |map: &HashMap<char, i32>| {
        for (k, v) in &ori {
            if map.get(k).unwrap_or(&0) < v {
                return false;
            }
        }
        true
    };

    let s_len = s.chars().count() as i32;
    for r in 0..s_len {
        let k1 = s.chars().nth(r as usize).unwrap();
        if r < s_len && ori.contains_key(&k1) {
            *cnt.entry(k1).or_insert(1) += 1;
        }

        while check(&cnt) && l <= r {
            if r - l + 1 < len {
                len = r - l + 1;
                ans_l = l;
                ans_r = l + len;
            }
            let k2 = s.chars().nth(l as usize).unwrap();
            if ori.contains_key(&k2) {
                *cnt.entry(k2).or_insert(0) -= 1;
            }
            l += 1;
        }
    }

    if ans_l == -1 {
        "".to_string()
    } else {
        s.chars()
            .skip(ans_l as usize)
            .take((ans_r - ans_l) as usize)
            .collect()
    }
}

// 作者：mengsuenyan
// 链接：https://leetcode-cn.com/problems/minimum-window-substring/solution/shuang-zhi-zhen-dong-tai-wei-hu-yi-ge-qu-jian-by-m/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn min_window_ok(s: String, t: String) -> String {
    if s.is_empty() || s.len() < t.len() {
        return "".to_string();
    }
    let cvt = |x: char| ((x as u32) & 0xff) as u8;

    const LEN: usize = 256;
    let (mut ac, mut ec) = ([0; LEN], [0; LEN]);

    t.chars().for_each(|x| {
        ec[cvt(x) as usize] += 1;
    });
    let (mut min_width, mut min_start, mut wnd_start, mut appeared) = (std::i32::MAX, 0, 0, 0);

    let ss: Vec<usize> = s.chars().map(|x| (x as u32) as usize).collect();
    let mut itr = ss.iter().enumerate();

    while let Some(x) = itr.next() {
        // let idx = cvt(x.1) as usize;
        let idx = *x.1;
        if ec[idx] > 0 {
            ac[idx] += 1;
            if ac[idx] <= ec[idx] {
                appeared += 1;
            }
        }

        if appeared == t.len() {
            // let mut tmp = s.chars().skip(wnd_start as usize);
            let mut tmp = ss.iter().skip(wnd_start as usize);
            while let Some(y) = tmp.next() {
                // let idx = cvt(y) as usize;
                let idx = *y;
                if (ac[idx] > ec[idx]) || (ec[idx] == 0) {
                    ac[idx] -= 1;
                    wnd_start += 1;
                } else {
                    break;
                }
            }

            let tp = x.0 as i32;
            if min_width > (1 + tp - wnd_start) {
                min_width = 1 + tp - wnd_start;
                min_start = wnd_start;
            }
        }
    }

    if min_width == std::i32::MAX {
        "".to_string()
    } else {
        s.chars()
            .skip(min_start as usize)
            .take(min_width as usize)
            .collect()
    }
}
