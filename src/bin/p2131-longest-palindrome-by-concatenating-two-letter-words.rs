use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let ws = words.into_iter().fold(0, |mut acc, w| {
            let rev = w.chars().rev().collect::<String>();

            match map.get_mut(&rev) {
                Some(v) if *v > 0 => {
                    *v -= 1;
                    acc += 4;
                }
                _ => {
                    map.entry(w).and_modify(|v| *v += 1).or_insert(1);
                }
            }
            acc
        });

        ws + map
            .into_iter()
            .any(|(k, v)| {
                let w = k.as_bytes();
                w[0] == w[1] && v > 0
            })
            .then(|| 2)
            .unwrap_or(0)
    }
}

struct Solution;

fn main() {}
