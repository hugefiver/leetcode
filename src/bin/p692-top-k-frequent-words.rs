use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();

        for w in words {
            *map.entry(w).or_insert(0) += 1;
        }

        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_unstable_by(|(w1, n1), (w2, n2)| {
            if n1 == n2 {
                w1.cmp(w2)
            } else {
                n2.cmp(n1)
            }
        });
        v.into_iter().take(k as usize).map(|(w, _)| w).collect()
    }
}

struct Solution;

fn main() {
}
