use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let xs = strs.into_iter().map(|s| {
            let mut m = [0; 26];
            s.as_bytes().iter().for_each(|x| {
                m[(x - b'a') as usize] += 1;
            });
            (m, s)
        }).fold(HashMap::new(), |mut acc, (m, s)| {
            let v = acc.entry(m).or_insert(Vec::new());
            v.push(s);
            acc
        });
        xs.into_values().collect()
    }
}

struct Solution;

fn main() {}
