use std::collections::HashMap;

fn main() {
    let foo = "[".to_string();

    println!("result: {}", is_valid(foo));
}

pub fn is_valid(s: String) -> bool {
    let left = ['(', '{', '['];
    let right = [')', '}', ']'];

    let map = left
        .iter()
        .cloned()
        .zip(right.iter().cloned())
        .fold(vec![], |mut acc, x| {
            acc.push(x);
            acc
        })
        .iter()
        .cloned()
        .collect::<HashMap<char, char>>();

    let mut cache: Vec<char> = vec![];

    for i in s.chars() {
        if left.contains(&i) {
            cache.push(i);
        } else {
            if let Some(v) = cache.last() {
                if map[v] == i {
                    cache.pop();
                    continue;
                }
            }
            return false;
        }
    }

    if cache.len() != 0 {
        false
    } else {
        true
    }
}
