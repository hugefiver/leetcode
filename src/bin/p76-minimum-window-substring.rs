use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_byte = s.as_bytes();
        let mut map = t.as_bytes().into_iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        });
        let mut n_free = t.len() as i32;
        let (mut len, mut pos) = (s.len(), 0);
        let mut window = VecDeque::<usize>::new();
        for (i, b) in s_byte.iter().enumerate() {
            if let Some(count) = map.get_mut(b) {
                *count -= 1;
                window.push_back(i);
                if *count >= 0 {
                    n_free -= 1;
                }

                while let Some(&idx) = window.front() {
                    let cur_count = map.get_mut(&s_byte[idx]).unwrap();
                    if *cur_count < 0 {
                        *cur_count += 1;
                        window.pop_front();
                    } else {
                        break;
                    }
                }
                if n_free == 0 {
                    let cur_len = i - *window.front().unwrap();
                    if cur_len < len {
                        len = cur_len;
                        pos = i;
                    }
                }
            }
        }
        if len == s.len() {
            "".to_owned()
        } else {
            String::from(&s[pos - len..pos + 1])
        }
    }
}


struct Solution;

fn main() {
    println!("{:?}", Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
}