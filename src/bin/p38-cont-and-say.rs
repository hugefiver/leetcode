use std::iter;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n <= 1 {
            return iter::repeat('1').take(n as usize).collect();
        }

        let s = Self::count_and_say(n - 1);
        let mut result = String::new();
        let mut count = 0;
        let mut last = None;
        s.chars().into_iter().for_each(|x| {
            if let Some(l) = last {
                if l == x {
                    count += 1;
                } else {
                    result.push_str(format!("{}{}", count, l).as_str());
                    count = 1;
                    last = Some(x);
                }
            } else {
                last = Some(x);
                count = 1;
            }
        });
        if count > 0 {
            result.push_str(format!("{}{}", count, last.unwrap()).as_str());
        }
        result
    }
}

struct Solution;

fn main() {
}
